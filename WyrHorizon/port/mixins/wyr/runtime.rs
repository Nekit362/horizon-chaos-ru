pub mod rules {
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

pub static CATALOG: &[Choice] = &[
Choice { id: "dummy", ru: "Ничего не произойдёт", positive: true, persistent: true, stackable: false, supported: true, random: false },
Choice { id: "mvp", ru: "Получить ранг MVP", positive: true, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "invisibility", ru: "Стать невидимым", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "double_mining_speed", ru: "Добывать блоки вдвое быстрее", positive: true, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "jump_height_increase", ru: "Прыгать выше", positive: true, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "any_mob_milkable", ru: "Доить любое существо", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "explosion_immunity", ru: "Не получать урон от взрывов", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "fall_damage_immunity", ru: "Не получать урон от падения", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "edible_leather", ru: "Есть кожу", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "fire_immunity", ru: "Не получать урон от огня", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "treasure_fishing_loot", ru: "Всегда вылавливать сокровища", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "bone_meal_on_sneak", ru: "Удобрять землю костной мукой при приседании", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "diamond_ore_drops", ru: "Получать только алмазы из любой руды", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "smelt_inventory", ru: "Переплавить всё содержимое инвентаря", positive: true, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "fletching_table_use", ru: "Наконец-то найти применение столу лучника", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "speed_increase", ru: "Двигаться быстрее", positive: true, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "enchant_inventory", ru: "Зачаровать всё содержимое инвентаря", positive: true, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "keep_inventory", ru: "Сохранять инвентарь после смерти", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "no_cobweb_collision", ru: "Свободно проходить сквозь паутину", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "faster_pigs", ru: "Быстрее ездить на свиньях", positive: true, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "frost_walker", ru: "Всегда иметь «Ледоход»", positive: true, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "egg_ender_pearl", ru: "Использовать яйца как жемчуг Эндера", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "remove_negative_choice", ru: "Отменить случайный отрицательный выбор", positive: true, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "short", ru: "Стать маленьким", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "knockback", ru: "Сильнее отбрасывать врагов", positive: true, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "infinite_shearing", ru: "Стричь овец, не лишая их шерсти", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "set_items_to_full_stack", ru: "Заполнить все стопки предметов в инвентаре", positive: true, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "unbreakable_tools", ru: "Инструменты больше не теряют прочность", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "all_advancements", ru: "Получить все достижения", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "faster_climbing", ru: "Быстрее карабкаться", positive: true, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "creepers_explode_flowers", ru: "Криперы взрываются россыпью цветов", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "piglins_barter_ender_eyes", ru: "Всегда получать очи Эндера при обмене с пиглинами", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "cheap_trades", ru: "Покупать у жителей очень дёшево", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "instant_taming", ru: "Приручать животных с первой попытки", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "goat_horn_buff", ru: "Получать полезный эффект при каждом использовании козьего рога", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "no_spider_climbing", ru: "Пауки не могут карабкаться к вам", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "total_shield_blocking", ru: "Щит защищает со всех сторон", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "extra_heart", ru: "Получить дополнительное сердце", positive: true, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "healed_by_cactus", ru: "Лечиться от прикосновения к кактусу", positive: true, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "unlimited_games", ru: "Игры, бесконечные игры", positive: true, persistent: true, stackable: false, supported: true, random: false },
Choice { id: "no_animations", ru: "У вас больше нет анимаций", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "remove_chunk", ru: "Чанк под вами будет удалён", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "randomized_block_drops", ru: "Из блоков выпадают случайные предметы", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "hotbar_slot_fov", ru: "Поле зрения зависит от выбранной ячейки панели быстрого доступа", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "hide_health", ru: "Ваше здоровье скрыто", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "adventure_mode_in_darkness", ru: "В полной темноте вы переходите в режим приключения", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "noclip_shovel_mineable_blocks", ru: "Проваливаться сквозь блоки, которые легко копаются лопатой", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "falling_step_blocks", ru: "Блоки над пустотой обрушиваются под ногами", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "leaves_to_creepers", ru: "Листва рядом с вами превращается в криперов", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "flower_allergy", ru: "У вас аллергия на цветы", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "teleport_to_world_height", ru: "Телепортироваться к верхней границе мира", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "teleport_to_overworld", ru: "Телепортироваться в обычный мир", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "teleport_to_nether", ru: "Телепортироваться в Незер", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "evaporate_water", ru: "Вода вокруг вас испаряется", positive: false, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "beds_always_explode", ru: "Кровати всегда взрываются", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "projectiles_instantly_kill", ru: "Снаряды мгновенно убивают вас", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "inedible_meat", ru: "Вы больше не можете есть мясо", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "random_shaders", ru: "Каждые 10 секунд включается случайный шейдер", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "armor_falls_off_when_jumping", ru: "Броня спадает с вас при прыжке", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "disable_crafting_tables", ru: "Вы больше не можете пользоваться верстаками", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "nausea_near_fish", ru: "Рядом с рыбой вас тошнит", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "double_block_item_depletion", ru: "Установка блоков расходует предметы вдвое быстрее", positive: false, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "enderman_carrying", ru: "Эндермены могут поднимать вас", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "thirst_bar", ru: "У вас появляется шкала жажды", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "vine_boom", ru: "Звук Vine boom", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "babify_zombies", ru: "Зомби рядом с вами становятся маленькими", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "offset_crosshair", ru: "Ваш прицел смещён", positive: false, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "shaky_buttons", ru: "Кнопки интерфейса трясутся", positive: false, persistent: true, stackable: true, supported: true, random: true },
Choice { id: "e_block_damage", ru: "Вы получаете урон на блоках с буквой «e» в английском названии", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "remove_positive_choice", ru: "Потерять случайный положительный выбор", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "set_items_to_single", ru: "В каждой стопке инвентаря остаётся по одному предмету", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "blinded_by_sunlight", ru: "Солнечный свет ослепляет вас", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "inverted_movement", ru: "Клавиши движения работают наоборот", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "unsupported_transparency", ru: "Игровой движок больше не поддерживает прозрачность", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "only_pick_negative_choices", ru: "Вы можете выбирать только отрицательные последствия", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "vilify_rabbits", ru: "Кролики рядом с вами становятся кровожадными", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "no_side_block_placement", ru: "Блоки можно ставить только сверху и снизу других блоков", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "frog_food", ru: "Лягушки пытаются вас съесть", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "no_fence_gate_opening", ru: "Вы не можете открывать калитки", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "explode", ru: "Устроить взрыв, который не ранит вас", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "sink_boats", ru: "Лодки под вами тонут", positive: false, persistent: true, stackable: false, supported: true, random: true },
Choice { id: "no_more_video_games", ru: "Никаких больше видеоигр", positive: false, persistent: false, stackable: false, supported: true, random: false },
Choice { id: "no_games", ru: "Никаких игр", positive: false, persistent: false, stackable: false, supported: true, random: false },
Choice { id: "catastrophe_1", ru: "Чёрная дыра", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "catastrophe_2", ru: "Метеоритный шторм", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "catastrophe_3", ru: "Убийца планет", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "catastrophe_4", ru: "Жезл Бога", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "catastrophe_5", ru: "Провал", positive: false, persistent: false, stackable: false, supported: true, random: true },
Choice { id: "catastrophe_6", ru: "Боевой заряд", positive: false, persistent: false, stackable: false, supported: true, random: true },
];


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

}
use super::Game;
use horizon_client::{hud::HudBatch, items::ItemStack};
use rules::{Player, choice};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::BTreeMap, path::PathBuf};

pub const ID: &str = "wyr";
const CHANNEL: &str = "wyr:state";

#[derive(Default)]
struct Server {
    current_actor: Option<u128>,
    mvp: std::collections::BTreeSet<u128>,
    blast_safe: std::collections::BTreeSet<i32>,
    completed: bool,
    root: PathBuf,
    players: BTreeMap<u128, Player>,
    entities: BTreeMap<i32, u128>,
    intents: BTreeMap<u128, Request>,
    commands: Vec<String>,
    ticks: u64,
}
#[derive(Default)]
struct Client {
    player: Option<Player>,
    reply: Option<usize>,
    action: Option<Request>,
    seen: u64,
    selected: usize,
    crosshair_start: Option<usize>,
}
#[derive(Clone, Serialize, Deserialize)]
struct Request {
    nonce: u64,
    side: Option<usize>,
    action: String,
    value: u32,
}
thread_local! {
    static SERVER: RefCell<Server> = RefCell::new(Server::default());
    static CLIENT: RefCell<Client> = RefCell::new(Client::default());
}

fn seed(id: u128) -> u64 {
    ((id >> 64) as u64 ^ id as u64 ^ 0xd1b54a32d192ed03).max(1)
}
fn uuid(id: u128) -> String {
    let s = format!("{id:032x}");
    format!(
        "{}-{}-{}-{}-{}",
        &s[..8],
        &s[8..12],
        &s[12..16],
        &s[16..20],
        &s[20..]
    )
}
fn queue(id: u128, command: String) {
    SERVER.with(|s| {
        s.borrow_mut()
            .commands
            .push(format!("execute as {} at @s run {command}", uuid(id)))
    });
}
fn local(game: &Game<'_>) -> Option<u128> {
    game.native_acting_player().map(|p| p.uuid)
}
fn server_player(id: u128) -> Player {
    SERVER.with(|s| s.borrow().players.get(&id).cloned().unwrap_or_default())
}
pub(in crate::game) fn has(game: &Game<'_>, id: &str) -> bool {
    if !game.native_on(ID) {
        return false;
    }
    if game.native_window() {
        CLIENT.with(|c| c.borrow().player.as_ref().is_some_and(|p| p.has(id)))
    } else {
        local(game).is_some_and(|p| server_player(p).has(id))
    }
}
fn by_entity(entity: i32) -> Player {
    SERVER.with(|s| {
        let s = s.borrow();
        s.entities
            .get(&entity)
            .and_then(|id| s.players.get(id))
            .cloned()
            .unwrap_or_default()
    })
}
fn load(game: &Game<'_>) {
    let Some(root) = game.native_world_dir() else {
        return;
    };
    SERVER.with(|s| {
        let mut s = s.borrow_mut();
        if s.root != root {
            *s = Server::default();
            s.root = root;
        }
    });
}
pub(in crate::game) fn save(_game: &Game<'_>) {}
fn tell(game: &mut Game<'_>, id: u128, text: &str) {
    let _ = game.native_tell(id, horizon_client::text::Text::literal(text));
}
fn encode_player(
    p: &Player,
) -> Result<horizon_world::nbt::Compound, horizon_world::attachments::AttachmentError> {
    let text = serde_json::to_string(p)
        .map_err(|_| horizon_world::attachments::AttachmentError::Decode)?;
    Ok([(
        "json".into(),
        horizon_world::nbt::Tag::String(text.as_str().into()),
    )]
    .into())
}
fn decode_player(
    c: &horizon_world::nbt::Compound,
) -> Result<Player, horizon_world::attachments::AttachmentError> {
    let Some(horizon_world::nbt::Tag::String(text)) = c.get(&"json".into()) else {
        return Err(horizon_world::attachments::AttachmentError::Decode);
    };
    serde_json::from_str(&text.to_string_lossy())
        .map_err(|_| horizon_world::attachments::AttachmentError::Decode)
}
static PLAYER_STATE: horizon_world::attachments::AttachmentType<Player> =
    horizon_world::attachments::AttachmentType {
        namespace: ID,
        name: "player",
        default: Player::default,
        codec: Some(horizon_world::attachments::AttachmentCodec {
            encode: encode_player,
            decode: decode_player,
        }),
        copy_on_death: true,
        sync: false,
    };
fn encode_kept(
    items: &Vec<ItemStack>,
) -> Result<horizon_world::nbt::Compound, horizon_world::attachments::AttachmentError> {
    Ok([(
        "items".into(),
        horizon_world::nbt::Tag::List(
            items
                .iter()
                .map(horizon_items::item_nbt::item_to_nbt)
                .collect(),
        ),
    )]
    .into())
}
fn decode_kept(
    c: &horizon_world::nbt::Compound,
) -> Result<Vec<ItemStack>, horizon_world::attachments::AttachmentError> {
    if let Some(horizon_world::nbt::Tag::List(items)) = c.get(&"items".into()) {
        Ok(items
            .iter()
            .filter_map(horizon_items::item_nbt::item_from_nbt)
            .collect())
    } else {
        Ok(Vec::new())
    }
}
static KEPT: horizon_world::attachments::AttachmentType<Vec<ItemStack>> =
    horizon_world::attachments::AttachmentType {
        namespace: ID,
        name: "kept",
        default: Vec::new,
        codec: Some(horizon_world::attachments::AttachmentCodec {
            encode: encode_kept,
            decode: decode_kept,
        }),
        copy_on_death: true,
        sync: false,
    };
pub(in crate::game) fn server_tick(game: &mut Game<'_>, phase: horizon_client::mod_api::ServerTickPhase) {
    if !matches!(
        phase,
        horizon_client::mod_api::ServerTickPhase::ServerTickEnd { .. }
    ) {
        return;
    }
    load(game);
    advanced::nickname_tick(game);
    let commands = SERVER.with(|s| {
        let mut s = s.borrow_mut();
        s.ticks += 1;
        std::mem::take(&mut s.commands)
    });
    for cmd in commands {
        if !game.native_run_command(&cmd) {
            log::warn!("WYR: игровая команда не выполнена: {cmd}");
        }
    }
    if SERVER.with(|s| s.borrow().ticks % 200 == 0) {
        save(game);
    }
    if SERVER.with(|s|s.borrow().ticks%20==0) {
        let players=game.native_server_players();
        let rows:Vec<_>=players.iter().map(|who|serde_json::json!({"uuid":who.uuid.to_string(),"entity":who.entity_id,"flags":server_player(who.uuid).active})).collect();
        for chunk in rows.chunks(8) {
            if let Ok(bytes)=serde_json::to_vec(chunk) {for who in &players {let _=game.native_net_send_to_player(who.uuid,"wyr:entities",&bytes);}}
        }
    }
}
fn sync(game: &mut Game<'_>, id: u128, p: &Player) {
    if let Ok(bytes) = serde_json::to_vec(p) {
        let _ = game.native_net_send_to_player(id, CHANNEL, &bytes);
    }
}
pub(in crate::game) fn receive_server(game: &Game<'_>, player: u128, channel: &str, bytes: &[u8]) {
    if channel != CHANNEL || bytes.len() > 256 || !game.native_on(ID) {
        return;
    }
    if let Ok(request) = serde_json::from_slice::<Request>(bytes) {
        SERVER.with(|s| {
            s.borrow_mut().intents.insert(player, request);
        });
    }
}
pub(in crate::game) fn receive_client(game: &Game<'_>, channel: &str, bytes: &[u8]) {
    if game.native_on(ID) {catastrophes::receive(game,channel,bytes);}
    if channel=="wyr:entities" && bytes.len()<=30000 && game.native_on(ID) {
        if let Ok(rows)=serde_json::from_slice::<Vec<serde_json::Value>>(bytes) {
            for row in rows {
                let uuid=row["uuid"].as_str().and_then(|s|s.parse::<u128>().ok());
                let entity=row["entity"].as_i64().and_then(|v|i32::try_from(v).ok());
                if let (Some(uuid),Some(entity),Ok(flags))=(uuid,entity,serde_json::from_value(row["flags"].clone())) {horizon_world::wyr_shared::update_remote(uuid,entity,flags);}
            }
        }
        return;
    }
    if channel != CHANNEL || bytes.len() > 16000 || !game.native_on(ID) {
        return;
    }
    if let Ok(player) = serde_json::from_slice::<Player>(bytes) {
        CLIENT.with(|c| c.borrow_mut().player = Some(player));
    }
}
pub(in crate::game) fn reset_client() {
    catastrophes::clear();
    horizon_world::wyr_shared::set_client(Default::default());
    CLIENT.with(|c| *c.borrow_mut() = Client::default());
}
pub(in crate::game) fn tick_player(game: &mut Game<'_>) {
    if game.native_window() {
        return;
    }
    if !game.native_on(ID) {
        if let Some(id) = local(game) {
            if SERVER.with(|s| s.borrow_mut().players.remove(&id).is_some()) {
                refresh_attributes(id, &Player::default());
            }
        }
        return;
    }
    load(game);
    let Some(who) = game.native_acting_player() else {
        return;
    };
    let id = who.uuid;
    let Ok(mut p) = game
        .native_attachments(horizon_world::attachments::Holder::Player(id))
        .and_then(|a| a.get(&PLAYER_STATE))
    else {
        return;
    };
    if table::world_disabled(game) {p.rate=0;p.countdown=0;}
    if p.nonce == 0 && p.active.is_empty() {
        p.random = seed(id);
    }
    let first = SERVER.with(|s| {
        let mut s = s.borrow_mut();
        s.entities.insert(who.entity_id, id);
        s.players.insert(id, p.clone()).is_none()
    });
    if first {
        refresh_attributes(id, &p);
    }
    let request = SERVER.with(|s| s.borrow_mut().intents.remove(&id));
    let mut changed = false;
    if let Some(r) = request {
        if let Some(side) = r.side {
            if let Some(ids) = p.select(r.nonce, side) {
                apply(game, id, &mut p, &ids, false);
                changed = true;
            }
        } else if r.action == "open" {
            // Reopening never rerolls or postpones the forced choice.
            if p.pending.is_none() && who.permission >= 2 {
                p.offer(&who.dimension);
            }
            changed = true;
        } else if who.permission >= 2 {
            match r.action.as_str() {
                "rate" => {
                    p.rate = if r.value == 0 {
                        0
                    } else {
                        r.value.clamp(600, 72000)
                    };
                    p.countdown = p.rate;
                    if p.rate == 0 {
                        p.pending = None;
                    }
                    changed = true;
                }
                "clear" => {
                    p.active.clear();
                    p.pending = None;
                    p.disaster = None;
                    p.thirst = 20;
                    refresh_attributes(id, &p);
                    changed = true;
                }
                _ => {}
            }
        } else {
            tell(
                game,
                id,
                "Изменять интервал и сбрасывать последствия может только оператор.",
            );
        }
    }
    if let Some(number)=p.tick_disaster() {
        if catastrophes::dispatch(game,id,number,who.position) {
            p.disaster=None;
            tell(game,id,&format!("Удачи (Номер {number})"));
        } else if SERVER.with(|s|s.borrow().ticks%100==0) {
            tell(game,id,"Катастрофа ожидает запуска: нужен владелец обычного мира в том же измерении.");
        }
        changed=true;
    } else if p.disaster.is_some() {changed=true;}
    if p.rate > 0 && p.disaster.is_none() {
        p.countdown = p.countdown.saturating_sub(1);
        if p.countdown == 0 {
            if p.pending.is_some() {
                let side = p.random_index(2);
                if let Some(ids) = p.select(p.nonce, side) {
                    apply(game, id, &mut p, &ids, true);
                }
            }
            p.offer(&who.dimension);
            p.countdown = p.rate;
            changed = true;
        }
    }
    if p.tick_thirst(who.game_mode == 1 || who.game_mode == 3) {
        queue(id, "damage @s 2 minecraft:generic".into());
    }
    // Publish before helpers can trigger another hook.
    SERVER.with(|s| s.borrow_mut().players.insert(id, p.clone()));
    horizon_world::wyr_shared::update_player(id,who.entity_id,game.lan_acting,p.active.clone());
    horizon_world::wyr_shared::update_vehicle(id,game.riding,p.active.clone());
    ongoing(game, id, &mut p, &who);
    if SERVER.with(|s| s.borrow().ticks % 20 == 0) {
        deliver_kept(game, id);
    }
    SERVER.with(|s| s.borrow_mut().players.insert(id, p.clone()));
    if let Ok(a) = game.native_attachments(horizon_world::attachments::Holder::Player(id)) {
        let _ = a.set(&PLAYER_STATE, p.clone());
    }
    if changed || SERVER.with(|s| s.borrow().ticks % 20 == 0) {
        sync(game, id, &p);
    }
}

fn refresh_attributes(id: u128, p: &Player) {
    let modifiers = [
        (
            "movement_speed",
            "speed",
            0.5 * f64::from(p.count("speed_increase")),
            "add_multiplied_total",
        ),
        (
            "block_break_speed",
            "mining",
            2f64.powi(p.count("double_mining_speed").min(20) as i32) - 1.0,
            "add_multiplied_total",
        ),
        (
            "jump_strength",
            "jump",
            if p.has("jump_height_increase") {
                0.1 + 0.1 * f64::from(p.count("jump_height_increase"))
            } else {
                0.0
            },
            "add_value",
        ),
        (
            "max_health",
            "hearts",
            2.0 * f64::from(p.count("extra_heart")),
            "add_value",
        ),
        (
            "scale",
            "short",
            if p.has("short") { -0.5 } else { 0.0 },
            "add_multiplied_total",
        ),
    ];
    for (attribute, key, amount, operation) in modifiers {
        queue(
            id,
            format!("attribute @s minecraft:{attribute} modifier remove wyr:{key}"),
        );
        if amount != 0.0 {
            queue(
                id,
                format!(
                    "attribute @s minecraft:{attribute} modifier add wyr:{key} {amount} {operation}"
                ),
            );
        }
    }
}
fn apply(game: &mut Game<'_>, id: u128, p: &mut Player, ids: &[String], forced: bool) {
    tell(
        game,
        id,
        if forced {
            "Вы слишком долго думали, и игра выбрала за вас:"
        } else {
            "Вы выбрали:"
        },
    );
    for name in ids {
        if let Some(c) = choice(name) {
            tell(
                game,
                id,
                &format!("{} {}", if c.positive { "+" } else { "−" }, c.ru),
            );
        }
        match name.as_str() {
            "no_more_video_games"|"no_games"=>queue(id,"ban @s Никаких больше игр".into()),
            "explode" => {
                if let Some(who)=game.native_acting_player() {
                    SERVER.with(|s|{s.borrow_mut().blast_safe.insert(who.entity_id);});
                    game.native_explode(&who.dimension,who.position,8.0,false,crate::game::native_explosions::NativeBlast::Tnt);
                    SERVER.with(|s|{s.borrow_mut().blast_safe.remove(&who.entity_id);});
                }
            }
            "elytra" => grant(game, id, ItemStack::new("minecraft:elytra", 1)),
            "bundle" => grant(game, id, ItemStack::new("minecraft:bundle", 1)),
            "unlimited_bacon" => grant(game, id, ItemStack::new("wyr:unlimited_bacon", 1)),
            "enchant_inventory" => {
                let enchants = &game.items().enchantments.list;
                if !enchants.is_empty() {
                    let e = &enchants[p.random_index(enchants.len())];
                    let key = e.id.clone();
                    let level = 1 + p.random_index(e.max_level.max(1) as usize) as u32;
                    for slot in (0..36).chain(100..=103).chain(std::iter::once(-106)) {
                        if let Some(s) = game.native_carried_stack(slot) {
                            if s.enchantment_level(&key) == 0 {
                                s.enchant(&key, level);
                            }
                        }
                    }
                }
            }
            "set_items_to_single" | "set_items_to_full_stack" => {
                for slot in (0..36).chain(100..=103).chain(std::iter::once(-106)) {
                    let item = game.native_carried_stack(slot).map(|s| s.item.clone());
                    let max = item.map(|item| game.items().get(&item).map_or(64, |i| i.max_stack));
                    if let Some(max) = max {
                        if let Some(s) = game.native_carried_stack(slot) {
                            s.count = if name == "set_items_to_single" {
                                1
                            } else {
                                max
                            };
                        }
                    }
                }
            }
            "smelt_inventory" => {
                for slot in (0..36).chain(100..=103).chain(std::iter::once(-106)) {
                    let cooked = game.native_carried_stack(slot).cloned().and_then(|s| {
                        game.recipes().cook("smelting", &s).map(|r| {
                            let mut out = r.result.clone();
                            out.count = out.count.saturating_mul(s.count);
                            out
                        })
                    });
                    if let Some(cooked) = cooked {
                        if let Some(s) = game.native_carried_stack(slot) {
                            *s = cooked;
                        }
                    }
                }
            }
            "all_advancements" => queue(id, "advancement grant @s everything".into()),
            "teleport_to_world_height" => {
                let (min, height) = game.native_world_height();
                queue(id, format!("tp @s ~ {} ~", min + height));
            }
            "teleport_to_overworld" | "teleport_to_nether" => {
                if let Some(at) = game.native_player() {
                    let nether = name == "teleport_to_nether";
                    let factor = if nether { 0.125 } else { 8.0 };
                    let dimension = if nether {
                        "minecraft:the_nether"
                    } else {
                        "minecraft:overworld"
                    };
                    let factor = if at.dimension == "minecraft:the_end" {
                        1.0
                    } else {
                        factor
                    };
                    queue(
                        id,
                        format!(
                            "execute in {dimension} run tp @s {} {} {}",
                            at.x * factor,
                            at.y.clamp(8.0, if nether { 248.0 } else { 312.0 }),
                            at.z * factor
                        ),
                    );
                }
            }
            "remove_chunk" => {
                if let Some(at) = game.native_player() {
                    let x = (at.x.floor() as i32).div_euclid(16) * 16;
                    let z = (at.z.floor() as i32).div_euclid(16) * 16;
                    let (min, height) = game.native_world_height();
                    // Small fill slices stay below vanilla's command block limit.
                    for y in (min..min + height).step_by(16) {
                        queue(
                            id,
                            format!(
                                "fill {x} {y} {z} {} {} {} minecraft:air",
                                x + 15,
                                (y + 15).min(min + height - 1),
                                z + 15
                            ),
                        );
                    }
                }
            }
            "remove_positive_choice" | "remove_negative_choice" => {
                if let Some(removed) = p.remove_random(name == "remove_positive_choice") {
                    tell(
                        game,
                        id,
                        &format!(
                            "Удалён выбор: {}",
                            choice(&removed).map_or(removed.as_str(), |c| c.ru)
                        ),
                    );
                } else {
                    tell(game, id, "Нечего удалять: подходящих выборов нет.");
                }
            }
            "thirst_bar" => {
                p.thirst = 20;
                p.thirst_cooldown = 400;
            }
            _ => {}
        }
    }
    refresh_attributes(id, p);
}

fn ongoing(
    game: &mut Game<'_>,
    id: u128,
    p: &mut Player,
    who: &horizon_client::mod_api::NativeServerPlayer,
) {
    if p.has("faster_climbing") {
        if let Some(m) = game.player.as_mut() {
            if m.last_climbable.is_some() && m.velocity.y > 0.0 {
                m.velocity.y = m
                    .velocity
                    .y
                    .max(0.2 + 0.2 * f64::from(p.count("faster_climbing")));
                m.needs_sync = true;
            }
        }
    }
    let ticks = SERVER.with(|s| s.borrow().ticks);
    if ticks % 20 != 0 {
        return;
    }
    let pos = who.position.map(|x| x.floor() as i32);
    let dim = who.dimension.as_str();
    if p.has("invisibility") {
        queue(id, "effect give @s minecraft:invisibility 2 0 true".into());
    }
    if p.has("adventure_mode_in_darkness") && who.game_mode != 1 && who.game_mode != 3 {
        let (block, sky) = game.native_light(pos);
        if sky == 0 && block == 0 && who.game_mode == 0 {
            p.darkness_mode = Some(0);
            queue(id, "gamemode adventure @s".into());
        } else if (sky > 0 || block > 0) && p.darkness_mode.take().is_some() {
            queue(id, "gamemode survival @s".into());
        }
    } else if p.darkness_mode.take().is_some() && who.game_mode == 2 {
        queue(id, "gamemode survival @s".into());
    }
    if p.has("blinded_by_sunlight") {
        let (_, sky) = game.native_light(pos);
        let time = game.native_day_time().rem_euclid(24000);
        if sky >= 15 && time < 12500 {
            queue(id, "effect give @s minecraft:blindness 2 0 true".into());
        }
    }
    let under = [pos[0], pos[1] - 1, pos[2]];
    let beneath = game.native_block(under);
    if p.has("e_block_damage")
        && who.game_mode != 3
        && beneath
            .strip_prefix("minecraft:")
            .unwrap_or(&beneath)
            .contains('e')
        && !matches!(
            beneath.as_str(),
            "minecraft:air" | "minecraft:water" | "minecraft:lava"
        )
    {
        queue(id, "damage @s 1 minecraft:generic".into());
    }
    if p.has("flower_allergy")
        && surrounding(game, pos, 3, |s| {
            game.items().block_in_tag("minecraft:flowers", s)
        })
    {
        let velocity = game
            .player
            .as_ref()
            .map(|m| m.velocity)
            .unwrap_or(horizon_core::geometry::Vec3::new(0.0, 0.0, 0.0));
        let norm = (velocity.x * velocity.x + velocity.z * velocity.z).sqrt();
        game.native_push_player(if norm > 0.00001 {
            [
                -(velocity.x / norm * 0.5) as f32,
                0.5,
                -(velocity.z / norm * 0.5) as f32,
            ]
        } else {
            [0.0, 0.5, 0.0]
        });
        game.native_level_play_sound("minecraft:entity.panda.sneeze", who.position, 7, 1.0, 0.75);
        if p.random_index(10) == 0 {
            queue(
                id,
                "summon minecraft:item ~ ~1 ~ {Item:{id:\"minecraft:slime_ball\",count:1}}".into(),
            );
        }
    }
    if p.has("healed_by_cactus") && surrounding(game, pos, 1, |s| s == "minecraft:cactus") {
        if let Some(m) = game.player.as_mut() {
            let _ = m.set_health(m.health.health() + 1.0, false);
        }
    }
    if p.has("evaporate_water") {
        let radius = (1 + p.count("evaporate_water")).min(16) as i32;
        for x in -radius..=radius {
            for y in -radius..=radius {
                for z in -radius..=radius {
                    let at = [pos[0] + x, pos[1] + y, pos[2] + z];
                    if game.native_block(at) == "minecraft:water" {
                        let _ = game.native_set_block(
                            dim,
                            at,
                            horizon_world::registry::BlockStateId(0),
                            3,
                        );
                    }
                }
            }
        }
    }
    if p.has("leaves_to_creepers") {
        for x in -2..=2 {
            for y in -2..=2 {
                for z in -2..=2 {
                    let at = [pos[0] + x, pos[1] + y, pos[2] + z];
                    if game.native_block(at).ends_with("_leaves") {
                        let _ = game.native_set_block(
                            dim,
                            at,
                            horizon_world::registry::BlockStateId(0),
                            3,
                        );
                        let _ = game.native_summon(
                            dim,
                            "minecraft:creeper",
                            at.map(|v| f64::from(v) + 0.5),
                            Default::default(),
                        );
                    }
                }
            }
        }
    }
    if p.has("falling_step_blocks")
        && beneath != "minecraft:air"
        && game.native_block([under[0], under[1] - 1, under[2]]) == "minecraft:air"
    {
        let state = game.world.block(under[0], under[1], under[2]);
        let _ = game.native_set_block(dim, under, horizon_world::registry::BlockStateId(0), 3);
        let _ = game.native_spawn_falling_block(
            dim,
            state,
            [
                f64::from(under[0]) + 0.5,
                f64::from(under[1]),
                f64::from(under[2]) + 0.5,
            ],
            [0.0; 3],
            Default::default(),
        );
    }
    if p.has("babify_zombies") {
        queue(id,"execute as @e[type=minecraft:zombie,distance=..12] run data merge entity @s {IsBaby:1b}".into());
    }
    if p.has("nausea_near_fish") {
        for fish in ["cod", "salmon", "pufferfish", "tropical_fish"] {
            queue(
                id,
                format!(
                    "execute if entity @e[type=minecraft:{fish},distance=..8] run effect give @s minecraft:nausea 5 0 true"
                ),
            );
        }
        if game.inventory.iter().flatten().any(|s| {
            ["cod", "salmon", "pufferfish", "tropical_fish"]
                .iter()
                .any(|fish| s.item.contains(fish))
        }) {
            queue(id, "effect give @s minecraft:nausea 5 0 true".into());
        }
    }
}
fn surrounding(game: &Game<'_>, at: [i32; 3], radius: i32, pred: impl Fn(&str) -> bool) -> bool {
    for x in -radius..=radius {
        for y in -radius..=radius {
            for z in -radius..=radius {
                if pred(&game.native_block([at[0] + x, at[1] + y, at[2] + z])) {
                    return true;
                }
            }
        }
    }
    false
}
fn flower(id: &str) -> bool {
    [
        "dandelion",
        "poppy",
        "orchid",
        "allium",
        "azure_bluet",
        "tulip",
        "oxeye_daisy",
        "cornflower",
        "lily_of_the_valley",
        "wither_rose",
        "sunflower",
        "lilac",
        "rose_bush",
        "peony",
        "torchflower",
    ]
    .iter()
    .any(|f| id.contains(f))
}

pub(in crate::game) fn damage(entity: i32, kind: &str, ty: &str, amount: &mut f32, canceled: &mut bool) {
    if kind != "minecraft:player" {
        return;
    }
    let p = by_entity(entity);
    if ty.contains("explosion") && SERVER.with(|s|s.borrow().blast_safe.contains(&entity)) {*canceled=true;}
    if (p.has("fire_immunity")
        && ["in_fire", "on_fire", "lava", "hot_floor"]
            .iter()
            .any(|v| ty.ends_with(v)))
        || (p.has("fall_damage_immunity") && ty == "minecraft:fall")
        || (p.has("explosion_immunity") && ty.contains("explosion"))
        || (p.has("healed_by_cactus") && ty == "minecraft:cactus")
    {
        *canceled = true;
    }
    if p.has("projectiles_instantly_kill")
        && ["arrow", "trident", "mob_projectile", "fireball", "thrown"]
            .iter()
            .any(|v| ty.ends_with(v))
    {
        *amount = f32::MAX;
    }
}
pub(in crate::game) fn knockback(attacker: i32, strength: &mut f64) {
    let p = by_entity(attacker);
    *strength *= 1.0 + f64::from(p.count("knockback"));
}
pub(in crate::game) fn right_click(game: &mut Game<'_>, pos: [i32; 3], face: [i32; 3], canceled: &mut bool) {
    let block = game.native_block(pos);
    if (has(game, "disable_crafting_tables") && block == "minecraft:crafting_table")
        || (has(game, "no_fence_gate_opening") && block.ends_with("_fence_gate"))
    {
        *canceled = true;
    }
    if has(game, "beds_always_explode") && block.ends_with("_bed") {
        *canceled = true;
        if let Some(id) = local(game) {
            queue(
                id,
                format!("setblock {} {} {} minecraft:air", pos[0], pos[1], pos[2]),
            );
            queue(
                id,
                format!(
                    "summon minecraft:tnt {} {} {} {{fuse:0}}",
                    pos[0], pos[1], pos[2]
                ),
            );
        }
    }
    // Restrict block-items only; interactions like buckets remain usable.
    if has(game, "no_side_block_placement") && face[1] == 0 {
        if let Some(s) = game.inventory.get(game.selected).and_then(|s| s.as_ref()) {
            if game.pack.assets.registry.default_state(&s.item).is_some() {
                *canceled = true;
            }
        }
    }
}
pub(in crate::game) fn placed(game: &mut Game<'_>) {
    let Some(id) = local(game) else {
        return;
    };
    let n = server_player(id).count("double_block_item_depletion");
    if n == 0 {
        return;
    }
    if let Some(slot) = game.inventory.get_mut(game.selected) {
        if let Some(s) = slot {
            s.count = s.count.saturating_sub((1u32 << n.min(16)) - 1);
            if s.count == 0 {
                *slot = None;
            }
        }
    }
}
pub(in crate::game) fn can_eat(game: &Game<'_>, stack: &ItemStack, can: &mut bool) {
    if &*stack.item == "minecraft:leather"
        && (!has(game, "edible_leather") || has(game, "inedible_meat"))
    {
        *can = false;
    }
    if has(game, "inedible_meat")
        && [
            "beef",
            "porkchop",
            "chicken",
            "mutton",
            "rabbit",
            "rotten_flesh",
            "cod",
            "salmon",
            "pufferfish",
            "tropical_fish",
            "unlimited_bacon",
        ]
        .iter()
        .any(|v| stack.item.contains(v))
    {
        *can = false;
    }
}
pub(in crate::game) fn item_damage(game: &Game<'_>, amount: &mut i32, cancel: &mut bool) {
    if has(game, "unbreakable_tools") {
        *amount = 0;
        *cancel = true;
    }
}
pub(in crate::game) fn shield_angle(game: &Game<'_>, value: f64) -> f64 {
    if has(game, "total_shield_blocking") { 0.0 } else { value }
}
pub(in crate::game) fn consumed(game: &mut Game<'_>, stack: &ItemStack, result: &mut Option<ItemStack>) {
    let Some(id) = local(game) else {
        return;
    };
    let mut p = server_player(id);
    if &*stack.item == "minecraft:potion"
        && stack.potion() == Some("minecraft:water")
        && p.has("thirst_bar")
    {
        p.thirst = (p.thirst + 8).min(20);
        SERVER.with(|s| s.borrow_mut().players.insert(id, p.clone()));
        sync(game, id, &p);
        if let Ok(a) = game.native_attachments(horizon_world::attachments::Holder::Player(id)) {
            let _ = a.set(&PLAYER_STATE, p.clone());
        }
    }
    if &*stack.item == "wyr:unlimited_bacon" {
        *result = Some(stack.clone());
    }
}
pub(in crate::game) fn jumped(game: &mut Game<'_>) {
    if !has(game, "armor_falls_off_when_jumping") {
        return;
    }
    let Some(id) = local(game) else {
        return;
    };
    let armor = std::mem::take(&mut game.armor);
    for stack in armor.into_iter().flatten() {
        let _ = game.native_give_stack(id, stack);
    }
}
pub(in crate::game) fn loot(
    game: &mut Game<'_>,
    context: &horizon_client::mod_api::NativeLootDrops<'_>,
    drops: &mut Vec<ItemStack>,
) {
    let id = if let Some((entity, "minecraft:player")) = context.entity {
        SERVER.with(|s| s.borrow().entities.get(&(entity as i32)).copied())
    } else {
        context.player
    };
    let Some(id) = id else {
        return;
    };
    let mut p = server_player(id);
    if context
        .entity
        .is_some_and(|(_, kind)| kind == "minecraft:player")
        && p.has("keep_inventory")
    {
        if let Ok(a) = game.native_attachments(horizon_world::attachments::Holder::Player(id)) {
            if let Ok(mut kept) = a.get(&KEPT) {
                kept.append(drops);
                let _ = a.set(&KEPT, kept);
            }
        }
        return;
    }
    if context.block.is_some() && !context.used {
        if p.has("diamond_ore_drops") && context.table.contains("ore") {
            drops.clear();
            drops.push(ItemStack::new("minecraft:diamond", 1));
        }
        if p.has("randomized_block_drops") {
            let ids = game.items().order.clone();
            if !ids.is_empty() {
                for drop in drops.iter_mut() {
                    let item = ids[p.random_index(ids.len())].clone();
                    *drop = ItemStack::new(&item, drop.count);
                }
            }
        }
    }
    SERVER.with(|s| s.borrow_mut().players.insert(id, p));
}
pub(in crate::game) fn respawn(game: &mut Game<'_>, id: u128) {
    deliver_kept(game, id);
    refresh_attributes(id, &server_player(id));
}
fn deliver_kept(game: &mut Game<'_>, id: u128) {
    let kept = game
        .native_attachments(horizon_world::attachments::Holder::Player(id))
        .and_then(|a| a.get(&KEPT))
        .unwrap_or_default();
    let mut remaining = Vec::new();
    for stack in kept {
        if let Some(stack) = game.native_give_stack(id, stack) {
            remaining.push(stack);
        }
    }
    if let Ok(a) = game.native_attachments(horizon_world::attachments::Holder::Player(id)) {
        let _ = a.set(&KEPT, remaining);
    }
}
fn grant(game: &mut Game<'_>, id: u128, stack: ItemStack) {
    if let Some(stack) = game.native_give_stack(id, stack) {
        if let Ok(a) = game.native_attachments(horizon_world::attachments::Holder::Player(id)) {
            if let Ok(mut kept) = a.get(&KEPT) {
                kept.push(stack);
                let _ = a.set(&KEPT, kept);
            }
        }
        tell(
            game,
            id,
            "Освободите место в инвентаре: награда сохранена до получения.",
        );
    }
}

pub(in crate::game) fn frame(game: &mut Game<'_>) {
    if !game.native_window() {
        return;
    }
    if !game.native_on(ID) {
        CLIENT.with(|c| *c.borrow_mut() = Client::default());
        horizon_world::wyr_shared::set_client(Default::default());
        return;
    }
    catastrophes::tick(game);
    let flags=CLIENT.with(|c|c.borrow().player.as_ref().map(|p|p.active.clone()).unwrap_or_default());
    horizon_world::wyr_shared::set_client(flags.clone());
    if let Some(who)=game.native_acting_player() {horizon_world::wyr_shared::update_vehicle(who.uuid,game.riding,flags);}
    let (open, reply, action) = CLIENT.with(|c| {
        let mut c = c.borrow_mut();
        let open = c
            .player
            .as_ref()
            .is_some_and(|p| (p.pending.is_some() || p.disaster.is_some()) && p.nonce != c.seen);
        (open, c.reply.take(), c.action.take())
    });
    if open && game.native_mod_screen_open(ID) {
        CLIENT.with(|c| {
            let mut c = c.borrow_mut();
            c.seen = c.player.as_ref().map_or(0, |p| p.nonce);
        });
    }
    if let Some(action) = action {
        send_request(game, &action);
    }
    if let Some(side) = reply {
        let nonce = CLIENT.with(|c| c.borrow().player.as_ref().map_or(0, |p| p.nonce));
        send_request(
            game,
            &Request {
                nonce,
                side: Some(side),
                action: String::new(),
                value: 0,
            },
        );
        game.native_mod_screen_close(ID);
    }
    if CLIENT.with(|c| {
        c.borrow()
            .player
            .as_ref()
            .is_some_and(|p| p.pending.is_none() && p.disaster.is_none())
    }) {
        game.native_mod_screen_close(ID);
    }
}
fn send_request(game: &mut Game<'_>, request: &Request) {
    if let Ok(bytes) = serde_json::to_vec(request) {
        if let Err(e) = game.native_net_send_to_server(CHANNEL, &bytes) {
            game.native_chat(&format!("WYR: сервер не поддерживает мод ({e})"));
        }
    }
}
pub(in crate::game) fn key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_on(ID) {
        return;
    }
    if key == game.native_mod_key(ID, "open", "key.keyboard.v") {
        let has_pending = CLIENT.with(|c| {
            c.borrow()
                .player
                .as_ref()
                .is_some_and(|p| p.pending.is_some() || p.disaster.is_some())
        });
        if has_pending {
            game.native_mod_screen_open(ID);
        } else {
            game.native_chat("Сейчас нет предложения выбора. Новое появится по таймеру.");
        }
    }
    if game.native_mod_screen() == Some(ID) {
        if CLIENT.with(|c|c.borrow().player.as_ref().is_some_and(|p|p.disaster.is_some())) {return;}
        match key {
            "key.keyboard.left" | "key.keyboard.a" => CLIENT.with(|c| c.borrow_mut().selected = 0),
            "key.keyboard.right" | "key.keyboard.d" => CLIENT.with(|c| c.borrow_mut().selected = 1),
            "key.keyboard.enter" => CLIENT.with(|c| {
                let mut c = c.borrow_mut();
                c.reply = Some(c.selected);
            }),
            _ => {}
        }
    }
}
fn wrap(game: &Game<'_>, text: &str, width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        let next = if line.is_empty() {
            word.to_string()
        } else {
            format!("{line} {word}")
        };
        if game.native_text_width(&next) > width && !line.is_empty() {
            lines.push(line);
            line = word.to_string();
        } else {
            line = next;
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}
pub(in crate::game) fn draw(game: &Game<'_>, batch: &mut HudBatch) {
    if game.native_mod_screen() != Some(ID) || !game.native_on(ID) {
        return;
    }
    let view = CLIENT.with(|c| {
        let c = c.borrow();
        c.player.clone().map(|p| (p, c.selected))
    });
    let Some((p, selected)) = view else {
        return;
    };
    if let Some(d)=p.disaster.as_ref() {
        let bw=260.0_f32.min(batch.width-16.0);let x=(batch.width-bw)/2.0;let y=(batch.height-140.0)/2.0;
        game.native_fill(batch,[0.0,0.0,batch.width,batch.height],0xc0000000);
        game.native_fill(batch,[x,y,bw,140.0],0xf03d1720);
        game.native_text(batch,&format!("Удачи (Номер {})",d.number),[x+18.0,y+18.0],0xffaaaa,true);
        if let Some(name)=catastrophes::NAMES.get(d.number.saturating_sub(1) as usize) {game.native_text(batch,name,[x+18.0,y+45.0],0xffffff,true);}
        game.native_text(batch,&format!("{}",d.ticks.div_ceil(20)),[x+bw/2.0-3.0,y+83.0],0xffdd66,true);
        if d.ticks==0 {game.native_text(batch,"Ожидание запуска…",[x+18.0,y+114.0],0xcccccc,true);}
        return;
    }
    let Some(ref pairs) = p.pending else {
        return;
    };
    let w = 440.0;
    let scale = ((batch.width - 16.0) / w)
        .min((batch.height - 16.0) / 210.0)
        .clamp(0.1, 1.0);
    let x = (batch.width - w * scale) / 2.0;
    let oy = (batch.height - 210.0 * scale) / 2.0;
    let y = oy + 20.0;
    game.native_fill(batch, [0.0, 0.0, batch.width, batch.height], 0xb0000000);
    let start = batch.vertices.len();
    game.native_text(batch, "Что бы ты выбрал?", [x, y - 18.0], 0xffffff, true);
    for (i, pair) in pairs.iter().enumerate() {
        let bx = x + i as f32 * (w + 8.0) / 2.0;
        let bw = (w - 8.0) / 2.0;
        game.native_fill(
            batch,
            [bx, y, bw, 145.0],
            if i == selected {
                0xf02a4054
            } else {
                0xf01d222b
            },
        );
        let mut ty = y + 12.0;
        if !p.has("only_pick_negative_choices") {
            if let Some(c) = choice(&pair.positive) {
                for line in wrap(game, c.ru, bw - 20.0) {
                    game.native_text(batch, &line, [bx + 10.0, ty], 0x77ee88, true);
                    ty += 11.0;
                }
            }
        }
        ty = ty.max(y + 65.0);
        if let Some(c) = choice(&pair.negative) {
            for line in wrap(game, c.ru, bw - 20.0) {
                game.native_text(batch, &line, [bx + 10.0, ty], 0xff8888, true);
                ty += 11.0;
            }
        }
        game.native_text(
            batch,
            if i == selected {
                "[ Подтвердить ]"
            } else {
                "Выбрать"
            },
            [bx + 12.0, y + 127.0],
            0xffffff,
            true,
        );
    }
    game.native_text(
        batch,
        &format!("До нового выбора: {} сек", p.countdown / 20),
        [x, y + 155.0],
        0xcccccc,
        true,
    );
    game.native_text(
        batch,
        "← → — выбор; Enter — подтвердить; V — открыть снова",
        [x, y + 170.0],
        0xaaaaaa,
        true,
    );
    batch.scale_since(start, x, oy, scale, scale);
}
pub(in crate::game) fn mouse(game: &Game<'_>, button: i32, canceled: &mut bool) {
    if CLIENT.with(|c|c.borrow().player.as_ref().is_some_and(|p|p.disaster.is_some())) {*canceled=true;return;}
    if button != 0 || game.native_mod_screen() != Some(ID) {
        return;
    }
    let Some(r) = game.renderer.as_ref() else {
        return;
    };
    let batch = HudBatch::new([r.config.width, r.config.height]);
    let w = 440.0;
    let scale = ((batch.width - 16.0) / w)
        .min((batch.height - 16.0) / 210.0)
        .clamp(0.1, 1.0);
    let x = (batch.width - w * scale) / 2.0;
    let oy = (batch.height - 210.0 * scale) / 2.0;
    let y = oy + 20.0;
    let [mx, my] = game.native_cursor();
    let mx = x + (mx - x) / scale;
    let my = oy + (my - oy) / scale;
    for i in 0..2 {
        let bx = x + i as f32 * (w + 8.0) / 2.0;
        let bw = (w - 8.0) / 2.0;
        if mx >= bx && mx < bx + bw && my >= y && my < y + 145.0 {
            CLIENT.with(|c| {
                let mut c = c.borrow_mut();
                if c.selected == i && my >= y + 118.0 {
                    c.reply = Some(i);
                } else {
                    c.selected = i;
                }
            });
            *canceled = true;
        }
    }
}
pub(in crate::game) fn hud(game: &Game<'_>, batch: &mut HudBatch) {
    CLIENT.with(|c| {
        let c = c.borrow();
        let Some(p) = &c.player else {
            return;
        };
        if p.has("thirst_bar") {
            let y = batch.height
                - if game.player.as_ref().is_some_and(|m| m.air < 300) {
                    59.0
                } else {
                    49.0
                };
            for i in 0..10 {
                let level = i * 2 + 1;
                let name = if level < p.thirst {
                    "full"
                } else if level == p.thirst {
                    "half"
                } else {
                    "empty"
                };
                game.native_sprite(
                    batch,
                    &format!("wyr:hud/thirst_{name}"),
                    [0.0, 0.0, 7.0, 9.0],
                    [batch.width / 2.0 + 92.0 - i as f32 * 8.0 - 9.0, y, 7.0, 9.0],
                );
            }
        }
    });
}
pub(in crate::game) fn layer_pre(layer: &str, batch: &HudBatch) {
    if layer == "minecraft:crosshair" {
        CLIENT.with(|c| c.borrow_mut().crosshair_start = Some(batch.vertices.len()));
    }
}
pub(in crate::game) fn layer_post(layer: &str, batch: &mut HudBatch) {
    if layer != "minecraft:crosshair" {
        return;
    }
    CLIENT.with(|c| {
        let mut c = c.borrow_mut();
        let count = c.player.as_ref().map_or(0, |p| p.count("offset_crosshair"));
        if let Some(start) = c.crosshair_start.take() {
            let end = (start + 6).min(batch.vertices.len());
            for v in &mut batch.vertices[start..end] {
                v.pos[0] += 8.0 * count as f32;
                v.pos[1] -= 4.0 * count as f32;
            }
        }
    });
}
pub(in crate::game) fn movement(forward: &mut f32, strafe: &mut f32) {
    CLIENT.with(|c| {
        if c.borrow()
            .player
            .as_ref()
            .is_some_and(|p| p.has("inverted_movement"))
        {
            *forward = -*forward;
            *strafe = -*strafe;
        }
    });
}
pub(in crate::game) fn fov(game: &Game<'_>, degrees: &mut f32) {
    CLIENT.with(|c| {
        if c.borrow()
            .player
            .as_ref()
            .is_some_and(|p| p.has("hotbar_slot_fov"))
        {
            *degrees = 20.0 + game.selected as f32 * 10.0;
        }
    });
}
pub(in crate::game) fn gui_layer(layer: &str, canceled: &mut bool) {
    if layer == "minecraft:player_health" {
        CLIENT.with(|c| {
            if c.borrow()
                .player
                .as_ref()
                .is_some_and(|p| p.has("hide_health"))
            {
                *canceled = true;
            }
        });
    }
}
pub(in crate::game) fn health_hud(
    game: &mut Game<'_>,
    inputs: &horizon_client::hud_layout::HealthInputs,
    wi: i32,
    hi: i32,
    draws: &mut Option<Vec<horizon_client::hud_layout::GuiDraw>>,
) {
    let hidden = CLIENT.with(|c| {
        c.borrow()
            .player
            .as_ref()
            .is_some_and(|p| p.has("hide_health"))
    });
    if !hidden || draws.is_some() {
        return;
    }
    let mut layout = game.native_health_hud_state().layout(inputs, wi, hi);
    layout.retain(|draw|!matches!(draw,horizon_client::hud_layout::GuiDraw::Sprite{sprite,..} if sprite.starts_with("hud/heart/")));
    *draws = Some(layout);
}
pub(in crate::game) fn command(game: &mut Game<'_>, command: &str, ran: &mut bool) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.first() != Some(&"wyr") {
        return;
    }
    *ran = true;
    match parts.get(1).copied() {
        None | Some("open") => {
            game.native_mod_screen_open(ID);
            CLIENT.with(|c| {
                c.borrow_mut().action = Some(Request {
                    nonce: 0,
                    side: None,
                    action: "open".into(),
                    value: 0,
                })
            });
        }
        Some("rate") => {
            if let Some(seconds) = parts.get(2).and_then(|s| s.parse::<u32>().ok()) {
                CLIENT.with(|c| {
                    c.borrow_mut().action = Some(Request {
                        nonce: 0,
                        side: None,
                        action: "rate".into(),
                        value: seconds.saturating_mul(20),
                    })
                });
            } else {
                game.native_chat("/wyr rate <секунды>; 0 — отключить; минимум 30 секунд");
            }
        }
        Some("clear") => CLIENT.with(|c| {
            c.borrow_mut().action = Some(Request {
                nonce: 0,
                side: None,
                action: "clear".into(),
                value: 0,
            })
        }),
        Some("list") => CLIENT.with(|c| {
            if let Some(p) = &c.borrow().player {
                for (id, n) in &p.active {
                    game.native_chat(&format!(
                        "{} ×{n}",
                        choice(id).map_or(id.as_str(), |c| c.ru)
                    ));
                }
            }
        }),
        _ => game.native_chat("WYR: /wyr open, /wyr list, /wyr rate <секунды>, /wyr clear"),
    }
}
pub(in crate::game) fn listed(mods: &mut Vec<horizon_client::mod_api::NativeMod>) {
    mods.push(horizon_client::mod_api::NativeMod {
        id: ID.into(),
        title: "Что бы ты выбрал? — Rust 0.1.0".into(),
        description: "Дилеммы, бонусы и проклятия. Русский перевод. V — открыть выбор.".into(),
        keys: vec![[
            "open".into(),
            "Открыть выбор".into(),
            "key.keyboard.v".into(),
        ]],
    });
}

pub mod extra {
// Additional WYR mechanics, translated from the supplied WYR 1.0.0 behaviour.
// This file is embedded as game::wyr::extra, so parent-module helpers stay private.
use crate::game::Game;
use horizon_client::{items::ItemStack, mobs::Species};
use horizon_core::{position::BlockPos, random::RandomSource};
use horizon_world::level::{LevelAccess, LevelEvent};
use std::{cell::RefCell, collections::BTreeMap, path::PathBuf};

fn active(game: &Game<'_>) -> bool {
    game.native_on(super::ID) && !game.native_window() && game.remote.is_none()
}

// PlayerEntityMixin and MobEntityMixin: a bucket milks every mob and player,
// while exchangeStack keeps creative buckets and puts a stacked bucket's milk
// in the inventory. This hook already runs only after the server's reach checks.
pub(in crate::game) fn interact_entity(
    game: &mut Game<'_>, entity: i32, kind: &str,
    stack: Option<&ItemStack>, claimed: &mut bool,
) {
    if *claimed || !active(game) || !super::has(game,"any_mob_milkable")
        || !stack.is_some_and(|s| &*s.item=="minecraft:bucket" && s.count>0) {return;}
    let living=kind=="minecraft:player" || game.mobs.iter().any(|m|m.id==entity as u32 && m.alive());
    if !living {return;}
    let Some(id)=super::local(game) else {return;};
    *claimed=true;
    if game.creative {
        if game.native_count_items("minecraft:milk_bucket")==0 {
            super::grant(game,id,ItemStack::new("minecraft:milk_bucket",1));
        }
    } else {
        game.consume_held();
        if game.held().is_none() {*game.held_mut()=Some(ItemStack::new("minecraft:milk_bucket",1));}
        else {super::grant(game,id,ItemStack::new("minecraft:milk_bucket",1));}
    }
    if let Some(p)=game.native_player() {
        game.native_level_play_sound("minecraft:entity.cow.milk",[p.x,p.y,p.z],7,1.0,1.0);
    }
}

// BoneMealOnSneakChoice: try above the feet, then the feet, then below.
// In particular, this does not use a player's crosshair or consume their items.
pub(in crate::game) fn toggle_sneak(game:&mut Game<'_>,sneaking:bool) {
    if !sneaking || !active(game) || !super::has(game,"bone_meal_on_sneak") {return;}
    let Some(p)=game.native_player() else{return;};
    let foot=BlockPos::new(p.x.floor() as i32,p.y.floor() as i32,p.z.floor() as i32);
    game.install_live_features();
    game.with_level(|b,level,ticks| {
        let above=BlockPos::new(foot.x,foot.y+1,foot.z);
        let below=BlockPos::new(foot.x,foot.y-1,foot.z);
        let grown=if b.grow_crop(level,ticks,above) || b.grow_water_plant(level,ticks,above,None) {Some(above)}
            else if b.grow_crop(level,ticks,foot) {Some(foot)}
            else if b.grow_crop(level,ticks,below) {Some(below)} else{None};
        if let Some(pos)=grown {level.emit(LevelEvent::LevelEvent{id:1505,pos,data:0});}
    });
    if let Err(error)=game.process_level_events() {log::warn!("WYR bone meal: {error}");}
}

// FletchingTableBlockMixin: the table disappears without dropping itself and
// drops sixteen tipped arrows of one randomly selected nonempty vanilla potion.
pub(in crate::game) fn right_click_block(game:&mut Game<'_>,pos:[i32;3],canceled:&mut bool) {
    if *canceled || !active(game) || !super::has(game,"fletching_table_use")
        || game.native_block(pos)!="minecraft:fletching_table" {return;}
    let potions:Vec<_>=horizon_client::brewing::POTIONS.iter().filter(|p|!p.effects.is_empty()).collect();
    if potions.is_empty() {return;}
    let index=game.world.random.next_int_bound(potions.len() as i32).unwrap_or(0) as usize;
    let stack=ItemStack::new("minecraft:tipped_arrow",16).with_potion(potions[index].id);
    let Some(air)=game.pack.assets.registry.default_state("minecraft:air") else{return;};
    let dim=game.dim.id();
    if !game.native_set_block(dim,pos,air,3) {return;}
    *canceled=true;
    let at=[f64::from(pos[0])+0.5,f64::from(pos[1])+0.5,f64::from(pos[2])+0.5];
    game.spawn_item(stack,at,[0.0,0.2,0.0],10);
    game.native_level_play_sound("minecraft:entity.player.levelup",at,4,1.0,2.0);
}

// SheepEntityMixin restores the wool AFTER the ordinary shearing operation.
// NativeLootDrops.used is exactly that point, after sheared=true and before drops.
pub(in crate::game) fn loot_drops(game:&mut Game<'_>,context:&horizon_client::mod_api::NativeLootDrops<'_>) {
    if !active(game) || !context.used || !context.tool.is_some_and(|s|&*s.item=="minecraft:shears") {return;}
    let Some(id)=context.player else{return;};
    if !super::server_player(id).has("infinite_shearing") {return;}
    let Some((entity,"minecraft:sheep"))=context.entity else{return;};
    if let Some(m)=game.mobs.iter_mut().find(|m|m.id==entity && m.alive() && !m.is_baby()) {
        m.extra.sheared=false;
        m.needs_sync=true;
        game.world.modified.insert((m.position.x.floor() as i32>>4,m.position.z.floor() as i32>>4));
    }
}

// GoatHornItemMixin: an actual horn use gives a random non-instant beneficial
// effect for 300 ticks, amplifier zero. A horn with no instrument never starts.
pub(in crate::game) fn use_item_start(game:&mut Game<'_>,stack:&ItemStack,canceled:&mut bool) {
    if *canceled || !active(game) || &*stack.item!="minecraft:goat_horn"
        || !super::has(game,"goat_horn_buff") {return;}
    let effects:Vec<_>=horizon_client::brewing::EFFECTS.iter().filter(|e|
        e.category==horizon_client::brewing::Category::Beneficial && !e.instant).collect();
    if effects.is_empty(){return;}
    let i=game.world.random.next_int_bound(effects.len() as i32).unwrap_or(0) as usize;
    game.add_player_effect(horizon_client::effects::EffectInstance::new(effects[i].id,300,0));
}

thread_local! {
    static PREVIOUS:RefCell<BTreeMap<(PathBuf,u128),([f64;3],bool)>>=RefCell::new(BTreeMap::new());
}

// Invoke after the parent's tick_player published the acting player's choices.
pub(in crate::game) fn player_tick(game:&mut Game<'_>) {
    if !active(game) {return;}
    let Some(who)=game.native_acting_player() else{return;};
    let p=super::server_player(who.uuid);
    let Some(m)=game.player.as_ref() else{return;};
    let at=[m.position.x,m.position.y,m.position.z];
    let ground=m.ground.on_ground;
    let flying=m.flying;
    let half=f64::from(m.dimensions.width)*0.5;
    let height=f64::from(m.dimensions.height);
    let root=game.native_world_dir().unwrap_or_default();
    let previous=PREVIOUS.with(|previous| {
        let mut previous=previous.borrow_mut();
        // Old worlds' position caches must not grow forever or affect another save.
        previous.retain(|(world,_),_|world==&root);
        previous.insert((root,who.uuid),(at,ground)).unwrap_or((at,false))
    });
    if p.has("vilify_rabbits") {
        // getEntitiesByClass against the player's box expanded by twelve blocks.
        for rabbit in &mut game.mobs {
            let rhalf=f64::from(rabbit.width())*0.5;
            if rabbit.species()==Species::Rabbit && rabbit.alive() && !rabbit.is_baby()
                && (rabbit.position.x-at[0]).abs()<=12.0+half+rhalf
                && (rabbit.position.z-at[2]).abs()<=12.0+half+rhalf
                && rabbit.position.y+f64::from(rabbit.height())>=at[1]-12.0
                && rabbit.position.y<=at[1]+height+12.0 && rabbit.extra.variant!=99 {
                // Rabbit's normal AI installs its killer-bunny goals on its next tick.
                rabbit.extra.variant=99;rabbit.needs_sync=true;
                game.world.modified.insert((rabbit.position.x.floor() as i32>>4,rabbit.position.z.floor() as i32>>4));
            }
        }
    }
    if p.has("vine_boom") && !flying {
        let pos=at.map(|v|v.floor() as i32);
        if game.native_block(pos)=="minecraft:vine" {
            if let Some(air)=game.pack.assets.registry.default_state("minecraft:air") {
                if game.native_set_block(&who.dimension,pos,air,3) {
                    let center=pos.map(|v|f64::from(v)+0.5);
                    game.native_level_play_sound("wyr:vine_boom",center,7,1.0,1.0);
                    // Original VineBoomChoice uses ExplosionSourceType.NONE: blocks stay.
                    game.native_explode(&who.dimension,center,1.0,false,crate::game::native_explosions::NativeBlast::Keep);
                }
            }
        }
    }
    if p.has("frost_walker") {
        // Run Horizon's real Frost Walker effect with a virtual enchantment.
        // Real equipment is restored before the hook returns; no item is issued,
        // repaired, enchanted or removed. Isolate all other armor effects so they
        // do not run twice. Vanilla's radius is capped at sixteen blocks.
        let old_level=game.armor[3].as_ref().map_or(0,|s|s.enchantment_level("minecraft:frost_walker"));
        let level=old_level.saturating_add(p.count("frost_walker")).min(14);
        let mut boots=ItemStack::new("minecraft:leather_boots",1);
        boots.enchant("minecraft:frost_walker",level);
        let armor=std::mem::take(&mut game.armor);
        game.armor[3]=Some(boots);
        game.location_changed_effects(previous.0,previous.1);
        game.armor=armor;
    }
}

}

pub mod advanced {
use crate::game::Game;
use horizon_client::items::ItemStack;
use horizon_core::random::RandomSource;

pub(in crate::game) fn enter_action(game:&Game<'_>) {
    super::SERVER.with(|s|s.borrow_mut().current_actor=super::local(game));
}
pub(in crate::game) fn current_count(id:&str)->u32 {
    super::SERVER.with(|s|{let s=s.borrow();s.current_actor.and_then(|id|s.players.get(&id)).map_or(0,|p|p.count(id))})
}
pub(in crate::game) fn fishing_table(value:&'static str)->&'static str {
    if current_count("treasure_fishing_loot")>0 {"minecraft:gameplay/fishing/treasure"}else{value}
}
pub(in crate::game) fn tame_roll(value:bool)->bool {value || current_count("instant_taming")>0}
pub(in crate::game) fn tame_interact(game:&mut Game<'_>,entity:i32,stack:Option<&ItemStack>,claimed:&mut bool) {
    if *claimed || game.native_window() || !super::has(game,"instant_taming") {return;}
    let Some(item)=stack.map(|s|s.item.as_ref()) else{return;};
    let Some(index)=game.mobs.iter().position(|m|m.id==entity as u32&&m.alive()&&!m.extra.tame) else{return;};
    use horizon_client::mobs::Species;
    let species=game.mobs[index].species();
    let accepts=match species {
        Species::Wolf=>item=="minecraft:bone"&&!horizon_client::mobs::tame::angry(&game.mobs[index]),
        Species::Cat=>game.items().item_in_tag("minecraft:cat_food",item),
        Species::Parrot=>game.items().item_in_tag("minecraft:parrot_food",item),
        _=>false,
    };
    if !accepts{return;}
    let Some(owner)=super::local(game) else{return;};
    if !game.creative {game.consume_held();}
    let m=&mut game.mobs[index];
    if species==Species::Wolf {horizon_client::mobs::tame::set_tame(m,true);m.nav_stop();m.ai.target=None;} else {m.extra.tame=true;}
    m.extra.owner=Some(horizon_client::lan::playerdata::uuid_ints(owner));m.persistent=true;m.needs_sync=true;
    if species!=Species::Parrot {m.extra.sitting=true;}
    game.world.modified.insert((m.position.x.floor() as i32>>4,m.position.z.floor() as i32>>4));
    let kind=m.kind.id.clone();
    game.mob_event_particles(entity as u32,7);
    game.native_entity_tame(entity as u32,&kind,Some(owner));
    *claimed=true;
}

pub(in crate::game) fn egg_impact(game:&mut Game<'_>,dimension:&str,kind:&str,owner:u32,at:[f64;3],canceled:bool) {
    if canceled || kind!="minecraft:egg" || !game.native_on(super::ID) {return;}
    let player=game.native_server_players().into_iter().find(|p|p.entity_id==owner as i32&&p.dimension==dimension&&p.health>0.0);
    if let Some(player)=player {
        if super::server_player(player.uuid).has("egg_ender_pearl") {
            super::queue(player.uuid,format!("tp @s {} {} {}",at[0],at[1],at[2]));
            super::queue(player.uuid,"damage @s 5 minecraft:fall".into());
        }
    }
}
pub(in crate::game) fn barter(game:&mut Game<'_>,mob:u32,value:Vec<ItemStack>)->Vec<ItemStack> {
    if !game.native_on(super::ID) {return value;}
    if !game.mobs.iter().any(|m|m.id==mob&&m.alive()) {return value;}
    let enabled=game.native_server_players().iter().any(|p|p.dimension==game.dim.id()&&p.health>0.0&&p.game_mode!=3&&super::server_player(p.uuid).has("piglins_barter_ender_eyes"));
    if enabled {let count=1+game.world.random.next_int_bound(4).unwrap_or(0) as u32;vec![ItemStack::new("minecraft:ender_eye",count)]}else{value}
}
pub(in crate::game) fn nickname_tick(game:&mut Game<'_>) {
    if game.native_window() {return;}
    for player in game.native_server_players() {
        let enabled=game.native_on(super::ID)&&super::server_player(player.uuid).has("mvp");
        let changed=super::SERVER.with(|s| {let mut s=s.borrow_mut();if enabled {s.mvp.insert(player.uuid)}else{s.mvp.remove(&player.uuid)}});
        if changed {let name=format!("[MVP] {}",player.name);game.native_set_nickname(player.uuid,if enabled{Some(&name)}else{None});}
    }
}
pub(in crate::game) fn level_pre(game:&Game<'_>,dimension:&str) {
    let enabled=game.native_on(super::ID)&&game.native_server_players().iter().any(|p|p.dimension==dimension&&p.health>0.0&&p.game_mode!=3&&super::server_player(p.uuid).has("piglins_barter_ender_eyes"));
    horizon_world::wyr_shared::set_barter(enabled);
}
pub(in crate::game) fn mounted_tame(game:&mut Game<'_>) {
    if game.native_window() || !super::has(game,"instant_taming") {return;}
    let Some(index)=game.mount_index() else{return;};
    let Some(owner)=super::local(game) else{return;};
    let m=&mut game.mobs[index];
    if horizon_client::mobs::equine::is_equine(m.species()) && !m.extra.tame {
        m.extra.tame=true;m.extra.owner=Some(horizon_client::lan::playerdata::uuid_ints(owner));m.persistent=true;m.needs_sync=true;
        let id=m.id;let kind=m.kind.id.clone();game.mob_event_particles(id,7);game.native_entity_tame(id,&kind,Some(owner));
    }
}

}

pub mod table {
use crate::game::Game;
use horizon_client::{containers::ModBlockEntity, items::ItemStack};
use horizon_world::{attachments::{AttachmentType,AttachmentCodec,AttachmentError,Holder},nbt::{Compound,Tag}};
use horizon_core::random::RandomSource;

pub const TABLE:&str="wyr:unrathering_table";
const QUESTS:[(&str,&str);12]=[
    ("diamond_block","алмазный блок"),("sniffer_egg","яйцо нюхача"),
    ("netherite_ingot","незеритовый слиток"),("echo_shard","осколок эха"),
    ("music_disc_pigstep","пластинка Pigstep"),("heart_of_the_sea","сердце моря"),
    ("axolotl_bucket","ведро с аксолотлем"),("ender_eye","око Эндера"),
    ("respawn_anchor","якорь возрождения"),("gravel","гравий"),
    ("cake","торт"),("end_crystal","кристалл Энда"),
];
fn number(nbt:&Compound,key:&str)->i32 {match nbt.get(&key.into()){Some(Tag::Int(v))=>*v,_=>0}}
fn set_number(nbt:&mut Compound,key:&str,value:i32) {nbt.insert(key.into(),Tag::Int(value));}
fn wanted(nbt:&Compound)->Option<String> {
    let i=number(nbt,"index").max(0) as usize;
    match nbt.get(&"quests".into()){Some(Tag::List(items))=>match items.get(i){Some(Tag::String(s))=>Some(s.to_string_lossy()),_=>None},_=>None}
}
fn encode_done(done:&bool)->Result<Compound,AttachmentError>{Ok([("done".into(),Tag::Byte(i8::from(*done)))].into())}
fn decode_done(nbt:&Compound)->Result<bool,AttachmentError>{Ok(matches!(nbt.get(&"done".into()),Some(Tag::Byte(1))))}
static COMPLETED:AttachmentType<bool>=AttachmentType{namespace:super::ID,name:"completed",default:||false,codec:Some(AttachmentCodec{encode:encode_done,decode:decode_done}),copy_on_death:false,sync:false};

pub(in crate::game) fn world_disabled(game:&mut Game<'_>)->bool {
    if game.dim.id()=="minecraft:overworld" {
        if let Ok(done)=game.native_attachments(Holder::Level).and_then(|a|a.get(&COMPLETED)) {
            super::SERVER.with(|s|s.borrow_mut().completed=done);
        }
    }
    super::SERVER.with(|s|s.borrow().completed)
}
pub(in crate::game) fn create(block:&str,entity:&mut Option<(String,Compound)>) {
    if block==TABLE {*entity=Some((TABLE.into(),Compound::new()));}
}
pub(in crate::game) fn click(game:&mut Game<'_>,pos:[i32;3],canceled:&mut bool) {
    if *canceled||!game.native_on(super::ID)||game.native_block(pos)!=TABLE{return;}
    *canceled=true;
    let Some(player)=super::local(game) else{return;};
    if game.dim.id()!="minecraft:overworld" {super::tell(game,player,"Стол отмены выбора работает в обычном мире.");return;}
    let held=game.held().map(|s|s.item.to_string());
    let mut quests:Vec<&str>=QUESTS.iter().map(|(id,_)|*id).collect();
    for i in (1..quests.len()).rev() {let j=game.world.random.next_int_bound((i+1) as i32).unwrap_or(0) as usize;quests.swap(i,j);}
    let result=game.native_with_mod_block_entity(pos,|entity| {
        if number(&entity.nbt,"ending")>0 {return (false,None);}
        let first=!entity.nbt.contains_key(&"quests".into());
        if first {
            let items=quests.iter().take(5).map(|id|Tag::String(format!("minecraft:{id}").as_str().into())).collect();
            entity.nbt.insert("quests".into(),Tag::List(items));set_number(&mut entity.nbt,"index",0);
        }
        let consumed=!first&&wanted(&entity.nbt).as_deref()==held.as_deref();
        if consumed {let next=number(&entity.nbt,"index")+1;set_number(&mut entity.nbt,"index",next);}
        let next=wanted(&entity.nbt);
        if next.is_none() {set_number(&mut entity.nbt,"ending",1);}
        (consumed,next)
    });
    if let Some((consumed,next))=result {
        if consumed {game.consume_held();}
        if let Some(next)=next {
            let ru=QUESTS.iter().find(|(id,_)|next.strip_prefix("minecraft:")==Some(*id)).map_or(next.as_str(),|(_,ru)|*ru);
            super::tell(game,player,&format!("Следующее подношение: {ru}, 1 шт. Возьмите предмет в руку и нажмите на стол."));
        } else {super::tell(game,player,"Все пять подношений приняты. Предложения выбора отключаются…");}
        game.native_level_play_sound("minecraft:block.enchantment_table.use",pos.map(|v|f64::from(v)+0.5),4,1.0,2.0);
        game.native_block_entity_sync(pos);
    }
}
pub(in crate::game) fn tick(game:&mut Game<'_>,entity:&mut ModBlockEntity) {
    if entity.id!=TABLE||!game.native_on(super::ID){return;}
    let ending=number(&entity.nbt,"ending");
    if ending==0||ending>200{return;}
    if ending==1 {
        if let Ok(a)=game.native_attachments(Holder::Level){let _=a.set(&COMPLETED,true);}
        super::SERVER.with(|s|s.borrow_mut().completed=true);
    }
    if ending==1||ending==20||ending==120||ending==200 {
        for player in game.native_server_players() {
            let Ok(mut p)=game.native_attachments(Holder::Player(player.uuid)).and_then(|a|a.get(&super::PLAYER_STATE)) else{continue;};
            if ending==1 {p.rate=0;p.countdown=0;p.pending=None;}
            if ending==20 {super::tell(game,player.uuid,"Предложения «Что бы ты выбрал?» отключены!");}
            if ending==120 {super::tell(game,player.uuid,"…");}
            if ending==200 {
                p.nonce=p.nonce.wrapping_add(1);
                p.pending=Some([
                    super::rules::Pair{positive:"unlimited_bacon".into(),negative:"no_more_video_games".into()},
                    super::rules::Pair{positive:"unlimited_games".into(),negative:"no_games".into()},
                ]);
                super::tell(game,player.uuid,"Последняя дилемма. Как и в оригинале, отрицательный выбор блокирует игрока в этом мире.");
            }
            if let Ok(a)=game.native_attachments(Holder::Player(player.uuid)){let _=a.set(&super::PLAYER_STATE,p.clone());}
            super::SERVER.with(|s|s.borrow_mut().players.insert(player.uuid,p.clone()));super::sync(game,player.uuid,&p);
        }
    }
    set_number(&mut entity.nbt,"ending",ending+1);
}
pub(in crate::game) fn render(game:&Game<'_>,entity:&ModBlockEntity,draw:&mut crate::game::native_block_entity_draw::BlockEntityDraw) {
    if entity.id!=TABLE {return;}
    if let Some(item)=wanted(&entity.nbt) {
        draw.push();draw.translate([0.5,1.0,0.5]);
        draw.rotate_about([0.0,1.0,0.0],(draw.game_time as f32+draw.partial)*3.0,[0.0;3]);draw.scale([0.5;3]);
        game.native_draw_item(draw,&ItemStack::new(&item,1),horizon_assets::item_model::DisplayContext::Ground,None,0);
        draw.pop();
    }
}

}

pub mod fork_features {
use crate::game::Game;
use horizon_client::items::ItemStack;
use horizon_core::random::RandomSource;

pub(in crate::game) fn tick(game:&mut Game<'_>) {
    if game.native_window() || !game.native_on(super::ID) {return;}
    // Vanilla's tryAttack carried its living player target after a successful hit.
    if super::has(game,"enderman_carrying") && game.riding.is_none() && !game.creative && !game.spectator() {
        if let Some((horizon_client::mobs::Who::Mob(id),when))=game.player_hurt_by() {
            if game.world.game_time-when<=1 && game.mobs.iter().any(|m|m.id==id&&m.alive()&&m.species()==horizon_client::mobs::Species::Enderman) {
                game.start_riding(id);game.position_rider();
            }
        }
    }
}
pub(in crate::game) fn creeper_flowers(game:&mut Game<'_>,at:[f64;3])->bool {
    if !game.native_on(super::ID) {return false;}
    let enabled=game.native_server_players().iter().any(|p|p.dimension==game.dim.id()&&p.health>0.0&&p.game_mode!=3&&super::server_player(p.uuid).has("creepers_explode_flowers"));
    if !enabled {return false;}
    let flowers:Vec<_>=game.items().block_tags.get("minecraft:small_flowers").into_iter().flat_map(|s|s.iter()).filter_map(|name|game.pack.assets.registry.default_state(name)).collect();
    if flowers.is_empty(){return false;}
    for _ in 0..32 {
        let i=game.world.random.next_int_bound(flowers.len() as i32).unwrap_or(0) as usize;
        let vx=game.world.random.next_gaussian()*0.25;let vy=game.world.random.next_gaussian().abs()*0.25;let vz=game.world.random.next_gaussian()*0.25;
        game.native_spawn_falling_block(game.dim.id(),flowers[i],at,[vx,vy,vz],Default::default());
    }
    game.native_level_play_sound("minecraft:entity.creeper.death",at,4,1.0,1.0);
    true
}
pub(in crate::game) fn prepare_shovel(game:&Game<'_>) {
    let registry=game.pack.assets.registry;
    let key=registry as *const _ as usize;
    if horizon_world::wyr_shared::shovel_ready(key){return;}
    let names=game.items().block_tags.get("minecraft:mineable/shovel");
    let mut ids=Vec::new();
    for id in 0..registry.len() {
        if let Some(state)=registry.state(horizon_world::registry::BlockStateId(id as u32)) {
            if names.is_some_and(|names|names.contains(&state.name)){ids.push(id as u32);}
        }
    }
    horizon_world::wyr_shared::set_shovel(key,ids);
}

}

pub mod catastrophes {
//! Fixed WYR catastrophe numbers. Countdown and drawing belong to WYR's offer UI.
use crate::game::Game;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::sync::Mutex;
static AUTHORIZED: Mutex<BTreeMap<u128, i64>> = Mutex::new(BTreeMap::new());
thread_local! { static WAITING: RefCell<VecDeque<(u8,[f64;3])>> = RefCell::new(VecDeque::new()); }
pub const CHANNEL: &str = "wyr:catastrophe";
pub const NAMES: [&str; 6] = ["Чёрная дыра", "Метеоритный шторм", "Убийца планет", "Жезл Бога", "Провал грунта", "Боевой заряд"];

/// Called by the authoritative WYR countdown for this player. Coordinates are
/// captured at zero, rather than when the warning first appeared.
pub(in crate::game) fn dispatch(game: &mut Game<'_>, player: u128, number: u8, at: [f64; 3]) -> bool {
    if !(1..=6).contains(&number) || at.iter().any(|v| !v.is_finite()) { return false; }
    let Some(host) = horizon_world::wyr_shared::host_uuid() else { return false; };
    // The host's native engine performs the server-authorized event. A guest
    // never receives a console permission grant. Cross-dimension rendering and
    // dedicated servers need a separate server playback host.
    let players=game.native_server_players();
    let Some(source)=players.iter().find(|p|p.uuid==player) else{return false;};
    if !players.iter().any(|p|p.uuid==host && p.dimension==source.dimension) {return false;}
    let mut data = vec![number];
    for v in at { data.extend_from_slice(&v.to_le_bytes()); }
    if game.native_net_send_to_player(host, CHANNEL, &data).is_err() {return false;}
    // Only the integrated owner's native renderer may execute an internally
    // dispatched WYR job; this never authorizes another player's messages.
    let mut active=AUTHORIZED.lock().unwrap_or_else(|e|e.into_inner());
    let now=game.world.game_time;
    let previous=active.get(&host).copied().unwrap_or(now).max(now);
    active.insert(host,previous.saturating_add(180*20));
    true
}

/// Only server-to-client traffic reaches this function; never accept a client's
/// proposed catastrophe or position as an authoritative WYR event.
pub(in crate::game) fn receive(_game: &Game<'_>, channel: &str, data: &[u8]) {
    if channel != CHANNEL || data.len() != 25 { return; }
    let at = std::array::from_fn(|i| f64::from_le_bytes(data[1+i*8..9+i*8].try_into().unwrap()));
    if !(1..=6).contains(&data[0]) || at.iter().any(|v|!v.is_finite()){return;}
    WAITING.with(|q|q.borrow_mut().push_back((data[0],at)));
}

/// Called from the mutable client frame. Preserve queued disasters while a
/// previous strike is still running; there is no second wheel or choice.
pub(in crate::game) fn tick(game: &mut Game<'_>) {
    if !game.native_window() || !game.native_on(super::ID) {return;}
    let job=WAITING.with(|q|q.borrow().front().copied());
    if let Some((number,at))=job {
        if start(game,number,at) {WAITING.with(|q|{q.borrow_mut().pop_front();});}
    }
}
pub(in crate::game) fn clear() {
    WAITING.with(|q|q.borrow_mut().clear());
    AUTHORIZED.lock().unwrap_or_else(|e|e.into_inner()).clear();
}
pub(in crate::game) fn authorized_owner(game:&Game<'_>,player:u128)->bool {
    if horizon_world::wyr_shared::host_uuid()!=Some(player){return false;}
    AUTHORIZED.lock().unwrap_or_else(|e|e.into_inner()).get(&player).is_some_and(|until|game.world.game_time<*until)
}

pub(in crate::game) fn start(game: &mut Game<'_>, number: u8, at: [f64; 3]) -> bool {
    crate::game::orbital_glue::wyr_start(game, number, at)
}

}
