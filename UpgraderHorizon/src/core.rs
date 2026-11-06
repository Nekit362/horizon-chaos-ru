// Server-side economy shared by the game bundle and its standalone tests.
use std::collections::BTreeMap;

pub const HOUSE_EDGE: f64 = 0.55;
pub const MAX_CHANCE: f64 = 0.65;
pub const SPIN_TICKS: u32 = 80;
pub const ATTEMPT_WINDOW: u64 = 1_200;
pub const ATTEMPT_LIMIT: usize = 3;
pub const MAX_TARGET_COUNT: u32 = 64;

#[derive(Clone, Debug)]
pub struct Recipe {
    pub ingredients: Vec<Vec<(String, u32)>>,
    pub result: String,
    pub count: u32,
}

pub fn blocked(item: &str) -> bool {
    let id = item.strip_prefix("minecraft:").unwrap_or(item);
    !item.starts_with("minecraft:") || id.ends_with("_spawn_egg") || id.starts_with("infested_")
        || matches!(id, "air" | "bedrock" | "barrier" | "light" | "structure_block" | "structure_void"
            | "jigsaw" | "command_block" | "chain_command_block" | "repeating_command_block"
            | "command_block_minecart" | "debug_stick" | "knowledge_book" | "spawner"
            | "budding_amethyst" | "end_portal_frame" | "reinforced_deepslate" | "petrified_oak_slab"
            | "farmland" | "dirt_path" | "trial_spawner" | "vault" | "suspicious_sand"
            | "suspicious_gravel" | "trial_key" | "ominous_trial_key" | "potion" | "splash_potion"
            | "lingering_potion" | "tipped_arrow" | "enchanted_book" | "written_book"
            | "suspicious_stew" | "filled_map" | "bundle" | "dragon_egg")
}

// Prices are deliberately NOT multiplied uniformly: that would leave all odds unchanged.
// Renewable material is cheap; progression and rare loot is much more expensive.
pub fn base_prices() -> BTreeMap<String, f64> {
    let mut out = BTreeMap::new();
    let groups: &[(f64, &str)] = &[
        (0.25, "dirt cobblestone cobbled_deepslate sand red_sand gravel netherrack end_stone granite diorite andesite tuff calcite basalt blackstone snowball wheat_seeds kelp moss_block mud root_dirt rooted_dirt"),
        (0.5, "stone deepslate flint bamboo cactus sugar_cane soul_sand soul_soil clay_ball"),
        (1.0, "oak_log spruce_log birch_log jungle_log acacia_log dark_oak_log mangrove_log cherry_log pale_oak_log crimson_stem warped_stem"),
        (1.5, "wheat potato carrot beetroot apple egg chorus_fruit string feather bone rotten_flesh spider_eye ink_sac"),
        (2.0, "porkchop beef chicken mutton cod salmon tropical_fish rabbit sweet_berries glow_berries melon_slice pumpkin cocoa_beans brown_mushroom red_mushroom"),
        (4.0, "leather slime_ball rabbit_hide resin_clump honeycomb pufferfish"),
        (8.0, "coal charcoal redstone glowstone_dust lapis_lazuli gunpowder magma_cream glow_ink_sac quartz amethyst_shard copper_ingot raw_copper"),
        (16.0, "prismarine_shard prismarine_crystals obsidian crying_obsidian"),
        (80.0, "iron_ingot raw_iron"),
        (160.0, "gold_ingot raw_gold"),
        (200.0, "ender_pearl blaze_rod phantom_membrane nautilus_shell"),
        (320.0, "emerald"),
        (1_200.0, "ghast_tear rabbit_foot shulker_shell heart_of_the_sea echo_shard sponge wet_sponge scute turtle_scute armadillo_scute"),
        (2_000.0, "diamond"),
        (3_000.0, "saddle name_tag"),
        (6_000.0, "netherite_upgrade_smithing_template"),
        (9_000.0, "ancient_debris netherite_scrap wither_skeleton_skull"),
        (16_000.0, "totem_of_undying trident heavy_core"),
        (30_000.0, "enchanted_golden_apple elytra"),
        (50_000.0, "nether_star"),
        (1_000.0, "sniffer_egg dragon_breath breeze_rod ominous_bottle"),
        (6_000.0, "music_disc_13 music_disc_cat music_disc_blocks music_disc_chirp music_disc_far music_disc_mall music_disc_mellohi music_disc_stal music_disc_strad music_disc_ward music_disc_11 music_disc_wait music_disc_pigstep music_disc_otherside music_disc_5 music_disc_relic music_disc_creator music_disc_creator_music_box music_disc_precipice disc_fragment_5"),
    ];
    for (value, ids) in groups {
        for id in ids.split_whitespace() { out.insert(format!("minecraft:{id}"), *value); }
    }
    out
}

pub fn seed_common(id: &str) -> Option<f64> {
    let name = id.strip_prefix("minecraft:")?;
    if blocked(id) { return None; }
    if name.ends_with("_leaves") || name.ends_with("_sapling") || name.ends_with("_propagule")
        || name.ends_with("_fungus") || name.ends_with("_roots") || name.ends_with("_coral")
        || name.ends_with("_coral_fan") || name.ends_with("_tulip")
        || matches!(name, "dandelion" | "poppy" | "blue_orchid" | "allium" | "azure_bluet"
            | "oxeye_daisy" | "cornflower" | "lily_of_the_valley" | "sunflower" | "lilac"
            | "rose_bush" | "peony" | "pink_petals" | "wildflowers" | "cactus_flower"
            | "short_grass" | "tall_grass" | "fern" | "large_fern" | "vine" | "lily_pad"
            | "seagrass" | "dead_bush" | "sea_pickle" | "hanging_roots" | "spore_blossom"
            | "torchflower" | "pitcher_plant" | "torchflower_seeds" | "pitcher_pod"
            | "brown_egg" | "blue_egg" | "beetroot_seeds" | "pumpkin_seeds" | "melon_seeds") {
        return Some(0.5);
    }
    if name.ends_with("_log") || name.ends_with("_stem") { return Some(1.0); }
    if name.ends_with("_sherd") || name.ends_with("_armor_trim_smithing_template") { return Some(4_000.0); }
    if name.ends_with("_banner_pattern") { return Some(2_000.0); }
    if name.ends_with("_head") || name == "skeleton_skull" { return Some(6_000.0); }
    None
}

pub fn prices_for(ids: &[String], recipes: &[Recipe]) -> BTreeMap<String, f64> {
    let mut prices = base_prices();
    for id in ids {
        if let Some(value) = seed_common(id) { prices.entry(id.clone()).or_insert(value); }
    }
    // Cheapest complete recipe, not an arbitrary fallback price. Templates and containers
    // are counted conservatively so unknown ingredients never become a free high-tier item.
    for _ in 0..64 {
        let mut changed = false;
        for recipe in recipes {
            if blocked(&recipe.result) || recipe.count == 0 || recipe.ingredients.is_empty() { continue; }
            let cost = recipe.ingredients.iter().filter(|slot| !slot.is_empty())
                .map(|slot| slot.iter().filter_map(|(id, count)| prices.get(id).map(|v| v * f64::from(*count)))
                    .reduce(f64::min)).collect::<Option<Vec<_>>>();
            let Some(parts) = cost else { continue; };
            let value = parts.iter().sum::<f64>() / f64::from(recipe.count);
            if !value.is_finite() || value < 0.001 { continue; }
            if prices.get(&recipe.result).is_none_or(|old| value < *old - 0.000_001) {
                prices.insert(recipe.result.clone(), value); changed = true;
            }
        }
        if !changed { break; }
    }
    prices.retain(|id, value| ids.contains(id) && !blocked(id) && value.is_finite() && *value > 0.0);
    prices
}

pub fn target_count(requested: u32, max_stack: u32) -> u32 {
    requested.clamp(1, MAX_TARGET_COUNT.min(max_stack).max(1))
}

pub fn chance(input_value: f64, target_unit: f64, count: u32) -> f64 {
    if !input_value.is_finite() || !target_unit.is_finite() || input_value <= 0.0 || target_unit <= 0.0 || count == 0 { return 0.0; }
    // No minimum-chance floor: dirt cannot acquire a subsidised chance at a nether star.
    (HOUSE_EDGE * input_value / (target_unit * f64::from(count))).clamp(0.0, MAX_CHANCE)
}

#[derive(Clone, Debug, Default)]
pub struct Limiter { pub attempts: Vec<u64> }
impl Limiter {
    pub fn remaining(&self, now: u64) -> u64 {
        let active: Vec<_> = self.attempts.iter().copied().filter(|t| now.saturating_sub(*t) < ATTEMPT_WINDOW).collect();
        if active.len() < ATTEMPT_LIMIT { 0 } else { active[0].saturating_add(ATTEMPT_WINDOW).saturating_sub(now) }
    }
    pub fn record(&mut self, now: u64) -> bool {
        self.attempts.retain(|t| now.saturating_sub(*t) < ATTEMPT_WINDOW);
        if self.attempts.len() >= ATTEMPT_LIMIT { return false; }
        self.attempts.push(now); true
    }
}

// The fixed top pointer reads the rotating wheel; only the server decides won.
pub fn wheel_stop_angle(from: f64, odds: f64, won: bool) -> f64 {
    let tau = std::f64::consts::TAU;
    let green = odds.clamp(0.0, 1.0) * tau;
    let landing = if won { green * 0.5 } else { green + (tau - green) * 0.5 };
    let rotation = (-landing).rem_euclid(tau);
    from + tau * 2.0 + (rotation - from).rem_euclid(tau)
}

pub fn wheel_brake(from: f64, to: f64, progress: f64) -> f64 {
    let t = progress.clamp(0.0, 1.0);
    from + (to - from) * (1.0 - (1.0 - t).powi(3))
}

pub const WHEEL_SPEED: f64 = std::f64::consts::TAU * 2.2;
#[derive(Clone, Debug, Default)]
pub struct WheelAnimation {
    pub angle: f64,
    pub odds: f64,
    pub result: Option<bool>,
    start: Option<(f64, f64)>,
    stop: Option<(f64, f64, f64, f64)>,
}
impl WheelAnimation {
    pub fn begin(&mut self, now: f64, odds: f64) {
        self.update(now);
        self.angle=self.angle.rem_euclid(std::f64::consts::TAU);
        self.odds=odds.clamp(0.0,1.0);self.result=None;
        self.start=Some((now,self.angle));self.stop=None;
    }
    pub fn finish(&mut self, now:f64, odds:f64, won:bool) {
        if self.stop.is_some() {return;} // An open/snapshot packet cannot restart braking.
        self.update(now);self.odds=odds.clamp(0.0,1.0);self.result=Some(won);
        let to=wheel_stop_angle(self.angle,self.odds,won);
        // Cubic ease-out starts at 3*distance/duration. Matching the current
        // angular speed makes the transition into braking continuous.
        let duration=3.0*(to-self.angle)/WHEEL_SPEED;
        self.stop=Some((now,self.angle,to,duration));
    }
    pub fn update(&mut self, now:f64) {
        if let Some((when,from,to,duration))=self.stop {
            self.angle=wheel_brake(from,to,(now-when).max(0.0)/duration);
        } else if let Some((when,from))=self.start {
            self.angle=from+(now-when).max(0.0)*WHEEL_SPEED;
        }
    }
    pub fn running(&self)->bool {self.start.is_some() && self.stop.is_none()}
    pub fn braking(&self,now:f64)->bool {self.stop.is_some_and(|(when,_,_,duration)|now<when+duration)}
    pub fn visible(&self,now:f64)->bool {
        self.running() || self.stop.is_some_and(|(when,_,_,duration)|now<when+duration+1.8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn wheel_waits_for_server_then_brakes_smoothly_and_ignores_duplicate_results() {
        let mut wheel=WheelAnimation::default();
        wheel.begin(10.0,0.022);wheel.update(14.0);
        assert!((wheel.angle-WHEEL_SPEED*4.0).abs()<1e-9);
        assert!(wheel.visible(1000.0)); // No local timer invents a result.
        wheel.finish(14.0,0.022,true);
        let start=wheel.angle;wheel.update(14.000001);
        assert!(((wheel.angle-start)/0.000001-WHEEL_SPEED).abs()<0.001);
        wheel.update(16.0);let a=wheel.angle;
        wheel.finish(16.0,0.022,true);wheel.update(16.0);
        assert_eq!(wheel.angle,a); // Reopening must not rewind the animation.
        wheel.update(30.0);
        assert!((-wheel.angle).rem_euclid(std::f64::consts::TAU)<0.022*std::f64::consts::TAU);
        assert!(!wheel.visible(30.0));assert_eq!(wheel.result,Some(true));
        wheel.begin(31.0,0.65);assert_eq!(wheel.result,None);assert!(wheel.running());
        wheel.finish(32.0,0.65,false);wheel.update(50.0);
        assert!((-wheel.angle).rem_euclid(std::f64::consts::TAU)>=0.65*std::f64::consts::TAU);
    }
    #[test] fn wheel_lands_in_the_server_selected_sector_without_reversing() {
        let tau = std::f64::consts::TAU;
        for from in [-100.0, -1.0, 0.0, 6.0, 1000.0] {
            for odds in [0.000001, 0.022, 0.65] {
                for won in [false, true] {
                    let to = wheel_stop_angle(from, odds, won);
                    assert!(to >= from + tau * 2.0);
                    let pointer = (-to).rem_euclid(tau);
                    assert_eq!(pointer < odds * tau, won);
                    assert_eq!(wheel_brake(from,to,0.0),from);
                    assert!((wheel_brake(from,to,1.0)-to).abs()<1e-9);
                    let mut previous=from;
                    for i in 0..=100 {let angle=wheel_brake(from,to,i as f64/100.0);assert!(angle>=previous);previous=angle;}
                }
            }
        }
    }
    #[test] fn reward_quantity_reduces_odds_proportionally() {
        assert!((chance(80.0, 2_000.0, 1) - 0.022).abs() < 1e-12);
        assert!((chance(80.0, 2_000.0, 16) - 0.001375).abs() < 1e-12);
        assert_eq!(target_count(64, 1), 1); assert_eq!(target_count(0, 64), 1);
        assert_eq!(target_count(100_000, 64), 64);
    }
    #[test] fn cheap_input_does_not_receive_a_minimum_chance_subsidy() {
        assert!((chance(0.25, 50_000.0, 64) - 0.00000004296875).abs() < 1e-16);
        for (a,b,c) in [(0.25,50_000.0,64),(80.0,2_000.0,16),(100_000.0,1.0,1)] {
            assert!(chance(a,b,c) * b * f64::from(c) <= HOUSE_EDGE * a + 1e-9);
        }
        assert_eq!(chance(f64::NAN, 1.0, 1), 0.0);
        assert_eq!(chance(1.0, 0.0, 1), 0.0);
    }
    #[test] fn attempts_expire_only_after_full_window() {
        let mut l = Limiter::default();
        assert!(l.record(0)); assert!(l.record(20)); assert!(l.record(40));
        assert!(!l.record(41)); assert_eq!(l.remaining(41), 1159);
        assert_eq!(l.remaining(1200), 0); assert!(l.record(1200));
        assert_eq!(l.remaining(1200), 20);
    }
    #[test] fn recipes_expand_without_free_unknown_ingredients() {
        let ids = ["oak_log","oak_planks","stick","diamond","diamond_block","unknown"].map(|i|format!("minecraft:{i}")).to_vec();
        let recipes = vec![
            Recipe{ingredients:vec![vec![(ids[0].clone(),1)]],result:ids[1].clone(),count:4},
            Recipe{ingredients:vec![vec![(ids[1].clone(),1)];2],result:ids[2].clone(),count:4},
            Recipe{ingredients:vec![vec![(ids[3].clone(),1)];9],result:ids[4].clone(),count:1},
            Recipe{ingredients:vec![vec![("minecraft:missing".into(),1)]],result:ids[5].clone(),count:1},
        ];
        let p=prices_for(&ids,&recipes);
        assert_eq!(p[&ids[1]],0.25); assert_eq!(p[&ids[2]],0.125);
        assert_eq!(p[&ids[4]],18_000.0); assert!(!p.contains_key(&ids[5]));
        assert!(blocked("minecraft:creeper_spawn_egg")); assert!(blocked("upgrader:upgrader"));
    }
}
