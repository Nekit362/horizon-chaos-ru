use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Choice {
    pub id: &'static str,
    pub ru: &'static str,
    pub positive: bool,
    pub persistent: bool,
    pub stackable: bool,
    pub supported: bool,
    pub random: bool,
}

include!("catalog.rs");

pub fn choice(id: &str) -> Option<&'static Choice> {
    CATALOG.iter().find(|c| c.id == id)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pair {
    pub positive: String,
    pub negative: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Disaster {
    pub number: u8,
    pub ticks: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub active: BTreeMap<String, u32>,
    pub pending: Option<[Pair; 2]>,
    #[serde(default)]
    pub disaster: Option<Disaster>,
    pub nonce: u64,
    pub thirst: u32,
    pub thirst_cooldown: u32,
    pub countdown: u32,
    pub rate: u32,
    pub random: u64,
    #[serde(default)]
    pub darkness_mode: Option<i32>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            active: BTreeMap::new(),
            pending: None,
            disaster: None,
            nonce: 0,
            thirst: 20,
            thirst_cooldown: 400,
            countdown: 3600,
            rate: 3600,
            random: 0x9e3779b97f4a7c15,
            darkness_mode: None,
        }
    }
}

impl Player {
    pub fn count(&self, id: &str) -> u32 {
        self.active.get(id).copied().unwrap_or(0)
    }
    pub fn has(&self, id: &str) -> bool {
        self.count(id) > 0
    }
    pub fn random_index(&mut self, len: usize) -> usize {
        self.random ^= self.random << 13;
        self.random ^= self.random >> 7;
        self.random ^= self.random << 17;
        if len == 0 {
            0
        } else {
            self.random as usize % len
        }
    }
    fn available(&self, positive: bool, dimension: &str) -> Vec<&'static str> {
        CATALOG
            .iter()
            .filter(|c| {
                c.positive == positive
                    && c.supported
                    && c.random
                    && (c.stackable || !c.persistent || !self.has(c.id))
                    && !(c.id == "teleport_to_overworld" && dimension == "minecraft:overworld")
                    && !(c.id == "teleport_to_nether" && dimension == "minecraft:the_nether")
            })
            .map(|c| c.id)
            .collect()
    }
    fn pick_two(&mut self, mut options: Vec<&'static str>) -> [String; 2] {
        let mut result = ["dummy".to_string(), "dummy".to_string()];
        for slot in &mut result {
            if !options.is_empty() {
                let i = self.random_index(options.len());
                *slot = options.swap_remove(i).to_string();
            }
        }
        result
    }
    pub fn offer(&mut self, dimension: &str) {
        if self.disaster.is_some() {return;}
        let negative=self.pick_two(self.available(false,dimension));
        self.nonce = self.nonce.wrapping_add(1);
        if let Some(number)=self.choose_disaster(&negative) {
            self.pending=None;
            self.disaster=Some(Disaster{number,ticks:100});
            return;
        }
        let positive = self.pick_two(self.available(true, dimension));
        self.pending = Some([
            Pair {
                positive: positive[0].clone(),
                negative: negative[0].clone(),
            },
            Pair {
                positive: positive[1].clone(),
                negative: negative[1].clone(),
            },
        ]);
    }
    pub fn select(&mut self, nonce: u64, side: usize) -> Option<Vec<String>> {
        if self.disaster.is_some() {return None;}
        if nonce != self.nonce || side > 1 {
            return None;
        }
        let pairs = self.pending.take()?;
        let pair = &pairs[side];
        let mut ids = Vec::new();
        if !self.has("only_pick_negative_choices") {
            ids.push(pair.positive.clone());
        }
        ids.push(pair.negative.clone());
        for id in &ids {
            let c = choice(id)?;
            if c.persistent && id != "dummy" {
                let n = self.active.entry(id.clone()).or_default();
                *n = if c.stackable { n.saturating_add(1) } else { 1 };
            }
        }
        Some(ids)
    }
    pub fn tick_disaster(&mut self)->Option<u8> {
        let d=self.disaster.as_mut()?;
        if d.ticks>0 {d.ticks-=1;None} else {Some(d.number)}
    }
    fn choose_disaster(&mut self,options:&[String;2])->Option<u8> {
        let disasters:Vec<u8>=options.iter().filter_map(|id|id.strip_prefix("catastrophe_").and_then(|n|n.parse::<u8>().ok())).collect();
        if disasters.is_empty(){None}else{Some(disasters[self.random_index(disasters.len())])}
    }
    pub fn remove_random(&mut self, positive: bool) -> Option<String> {
        let ids: Vec<String> = self
            .active
            .keys()
            .filter(|id| choice(id).is_some_and(|c| c.positive == positive))
            .cloned()
            .collect();
        if ids.is_empty() {
            return None;
        }
        let i = self.random_index(ids.len());
        self.active.remove(&ids[i]);
        Some(ids[i].clone())
    }
    pub fn tick_thirst(&mut self, creative: bool) -> bool {
        if !self.has("thirst_bar") || creative {
            return false;
        }
        self.thirst_cooldown = self.thirst_cooldown.saturating_sub(1);
        if self.thirst_cooldown != 0 {
            return false;
        }
        if self.thirst == 0 {
            self.thirst_cooldown = 200;
            true
        } else {
            self.thirst -= 1;
            self.thirst_cooldown = 400;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn dilemma(p:&mut Player) {
        loop {p.disaster=None;p.offer("minecraft:overworld");if p.pending.is_some(){break;}}
    }
    #[test]
    fn offers_have_distinct_positive_and_negative_effects() {
        let mut p = Player::default();
        for _ in 0..100 {
            p.offer("minecraft:overworld");
            if p.disaster.take().is_some() {continue;}
            let a = p.pending.as_ref().unwrap();
            assert_ne!(a[0].positive, a[1].positive);
            assert_ne!(a[0].negative, a[1].negative);
            for pair in a {
                assert!(choice(&pair.positive).unwrap().positive);
                assert!(!choice(&pair.negative).unwrap().positive);
            }
        }
    }
    #[test]
    fn stale_and_replayed_answers_never_grant_rewards() {
        let mut p = Player::default();
        dilemma(&mut p);
        assert!(p.select(p.nonce.wrapping_sub(1), 0).is_none());
        assert!(p.select(p.nonce, 2).is_none());
        assert!(p.pending.is_some());
        assert!(p.select(p.nonce, 0).is_some());
        assert!(p.select(p.nonce, 0).is_none());
    }
    #[test]
    fn negative_only_is_a_penalty_not_a_free_positive() {
        let mut p = Player::default();
        p.active.insert("only_pick_negative_choices".into(), 1);
        dilemma(&mut p);
        let ids = p.select(p.nonce, 0).unwrap();
        assert_eq!(ids.len(), 1);
        assert!(!choice(&ids[0]).unwrap().positive);
    }
    #[test]
    fn dehydration_uses_original_twenty_second_and_ten_second_intervals() {
        let mut p = Player::default();
        p.active.insert("thirst_bar".into(), 1);
        for _ in 0..399 {
            assert!(!p.tick_thirst(false));
        }
        assert_eq!(p.thirst, 20);
        assert!(!p.tick_thirst(false));
        assert_eq!(p.thirst, 19);
        p.thirst = 0;
        p.thirst_cooldown = 1;
        assert!(p.tick_thirst(false));
        for _ in 0..199 {
            assert!(!p.tick_thirst(false));
        }
        assert!(p.tick_thirst(false));
    }
    #[test]
    fn persistent_nonstacking_choices_do_not_reappear() {
        let mut p = Player::default();
        p.active.insert("fire_immunity".into(), 1);
        assert!(
            !p.available(true, "minecraft:overworld")
                .contains(&"fire_immunity")
        );
        assert!(
            !p.available(false, "minecraft:the_nether")
                .contains(&"teleport_to_nether")
        );
    }
    #[test]
    fn save_round_trip_keeps_an_unanswered_dilemma() {
        let mut p = Player::default();
        dilemma(&mut p);
        p.active.insert("speed_increase".into(), 3);
        let json = serde_json::to_string(&p).unwrap();
        let mut loaded: Player = serde_json::from_str(&json).unwrap();
        assert_eq!(p.pending, loaded.pending);
        assert_eq!(loaded.count("speed_increase"), 3);
        assert!(loaded.select(p.nonce, 1).is_some());
    }
    #[test]
    fn catastrophe_is_forced_saved_and_counts_down_to_zero() {
        let mut p=Player::default();
        for _ in 0..1000 {p.offer("minecraft:overworld");if p.disaster.is_some(){break;}}
        let number=p.disaster.as_ref().unwrap().number;
        assert!((1..=6).contains(&number));assert!(p.pending.is_none());
        assert!(p.select(p.nonce,0).is_none());assert!(p.select(p.nonce,1).is_none());
        let mut loaded:Player=serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        let nonce=loaded.nonce;loaded.offer("minecraft:overworld");assert_eq!(loaded.nonce,nonce);
        for _ in 0..100 {assert_eq!(loaded.tick_disaster(),None);}
        assert_eq!(loaded.disaster.as_ref().unwrap().ticks,0);
        assert_eq!(loaded.tick_disaster(),Some(number));
    }
    #[test]
    fn two_catastrophes_choose_only_one_at_random() {
        let mut p=Player::default();let mut seen=std::collections::BTreeSet::new();
        for _ in 0..200 {seen.insert(p.choose_disaster(&["catastrophe_2".into(),"catastrophe_6".into()]).unwrap());}
        assert_eq!(seen,[2,6].into_iter().collect());
        assert_eq!(p.choose_disaster(&["speed_increase".into(),"catastrophe_4".into()]),Some(4));
    }
}
