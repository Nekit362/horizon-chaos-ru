mod core {
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

}
// Horizon integration. This file is embedded in the generated native bundle.
use std::{cell::RefCell, collections::{BTreeMap, VecDeque}, path::PathBuf, time::{Instant, SystemTime, UNIX_EPOCH}};
use serde_json::{Value, json};
use horizon_client::{hud::HudBatch, items::ItemStack, text::Text};
use horizon_core::random::RandomSource;
use horizon_world::{attachments::{AttachmentType, AttachmentCodec, AttachmentError, Holder}, nbt::{Compound, Tag}};
use crate::game::{Game, native_hooks::{NativeMod, NativeScreenView}};

pub const ID: &str = "upgrader_hardcore";
pub const DEVICE: &str = "upgrader:upgrader";
const CHANNEL: &str = "upgrader_hardcore:ui";

#[derive(Default)]
struct Server {
    world: PathBuf,
    queue: BTreeMap<u128, VecDeque<Value>>,
    prices: BTreeMap<String,f64>,
    catalog: Vec<Value>,
}
#[derive(Clone, Default)]
struct Player {
    granted: bool,
    limiter: core::Limiter,
    pending: Option<Pending>,
    tick: u32,
}
#[derive(Clone)]
struct Pending { target: String, count: u32, requested: u32, win: bool, ticks: u32, odds: f64, revealed: bool }
#[derive(Default)]
struct Client {
    catalog: Vec<Value>,
    queue: VecDeque<Value>,
    open: bool,
    selected: Option<usize>,
    target: String,
    count: u32,
    search: String,
    page: usize,
    message: String,
    spin: u32,
    odds: f64,
    cooldown: u64,
    origin: [f32;2],
    panel: [f32;2],
    scale: f32,
    wheel: core::WheelAnimation,
}
thread_local! {
    static SERVER: RefCell<Server> = RefCell::new(Server::default());
    static CLIENT: RefCell<Client> = RefCell::new(Client { count:1, ..Client::default() });
}

fn now() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs().saturating_mul(20) }
fn animation_time()->f64 {
    static START:std::sync::OnceLock<Instant>=std::sync::OnceLock::new();
    START.get_or_init(Instant::now).elapsed().as_secs_f64()
}
fn owns(game: &Game<'_>) -> bool { game.inventory.iter().flatten().chain(game.offhand.iter()).any(|s| &*s.item == DEVICE && s.count > 0) }
fn send(game: &mut Game<'_>, uuid: u128, value: Value) {
    if let Ok(bytes) = serde_json::to_vec(&value) { let _ = game.native_net_send_to_player(uuid,CHANNEL,&bytes); }
}
fn notify(game: &mut Game<'_>, uuid:u128, text:&str) {
    let _ = game.native_tell(uuid,Text::literal(text));
    send(game,uuid,json!({"message":text}));
}
fn encode_player(p:&Player)->Result<Compound,AttachmentError> {
    let pending=p.pending.as_ref().map(|v|json!({"target":v.target,"count":v.count,"requested":v.requested,"win":v.win,"ticks":v.ticks,"odds":v.odds,"revealed":v.revealed}));
    let json=json!({"granted":p.granted,"attempts":p.limiter.attempts,"pending":pending}).to_string();
    Ok([("json".into(),Tag::String(json.as_str().into()))].into())
}
fn decode_player(c:&Compound)->Result<Player,AttachmentError> {
    let Some(Tag::String(text))=c.get(&"json".into()) else {return Err(AttachmentError::Decode);};
    let p:Value=serde_json::from_str(&text.to_string_lossy()).map_err(|_|AttachmentError::Decode)?;
    let attempts=p["attempts"].as_array().map(|a|a.iter().filter_map(Value::as_u64).take(3).collect()).unwrap_or_default();
    let x=&p["pending"];
    let count=x["count"].as_u64().unwrap_or(1).clamp(1,64) as u32;
    let pending=if x.is_object() {Some(Pending{target:x["target"].as_str().ok_or(AttachmentError::Decode)?.into(),count,requested:x["requested"].as_u64().unwrap_or(u64::from(count)).clamp(1,64) as u32,win:x["win"].as_bool().unwrap_or(false),ticks:x["ticks"].as_u64().unwrap_or(1).min(80) as u32,odds:x["odds"].as_f64().unwrap_or(0.0),revealed:x["revealed"].as_bool().unwrap_or(false)})}else{None};
    Ok(Player{granted:p["granted"].as_bool().unwrap_or(false),limiter:core::Limiter{attempts},pending,..Player::default()})
}
// These values save in the SAME player snapshot as the inventory: a restart cannot
// roll the inventory back while keeping a newer prize, attempt or starter-item flag.
static PLAYER_STATE:AttachmentType<Player>=AttachmentType{
    namespace:ID,name:"player",default:Player::default,
    codec:Some(AttachmentCodec{encode:encode_player,decode:decode_player}),
    copy_on_death:true,sync:false,
};

fn build_catalog(game:&mut Game<'_>, server:&mut Server) {
    let Some(tables)=game.native_recipes() else { return; };
    let ids:Vec<String>=tables.items.iter().map(|i|i.id.clone()).collect();
    let mut recipes=Vec::new();
    for r in &tables.recipes {
        if r.slots.iter().any(|slot|!slot.is_empty() && slot.iter().all(|s|!s.components.is_empty())) {continue;}
        for result in &r.results {
            if !result.components.is_empty() {continue;}
            let Some(item)=tables.items.get(result.item as usize) else {continue;};
            let ingredients=r.slots.iter().map(|slot|slot.iter().filter(|s|s.components.is_empty()).filter_map(|s|tables.items.get(s.item as usize).map(|i|(i.id.clone(),s.count.max(1)))).collect()).collect();
            recipes.push(core::Recipe{ingredients,result:item.id.clone(),count:result.count});
        }
    }
    server.prices=core::prices_for(&ids,&recipes);
    // Optional world-local overrides; the server owns the prices. Non-positive means disabled.
    if let Ok(bytes)=std::fs::read(server.world.join("upgrader-prices.json")) {
        if let Ok(Value::Object(overrides))=serde_json::from_slice::<Value>(&bytes) {
            for (id,value) in overrides {
                if let Some(value)=value.as_f64().filter(|v|v.is_finite()) {
                    if core::blocked(&id) || value<=0.0 {server.prices.remove(&id);} else if ids.contains(&id) {server.prices.insert(id,value);}
                }
            }
        }
    }
    server.catalog=tables.items.iter().filter_map(|i|server.prices.get(&i.id).map(|value| {
        let max=game.res.items().get(&i.id).map_or(1,|v|v.max_stack);
        json!({"id":i.id,"name":i.name,"price":value,"max":max})
    })).collect();
    server.catalog.sort_by(|a,b|a["name"].as_str().cmp(&b["name"].as_str()));
    let report=server.world.join("upgrader-effective-prices.json");
    if let Ok(bytes)=serde_json::to_vec_pretty(&server.prices) {let _=std::fs::write(report,bytes);}
}
fn send_catalog(game:&mut Game<'_>, uuid:u128, server:&Server) {
    send(game,uuid,json!({"catalog_reset":true}));
    for chunk in server.catalog.chunks(100) {send(game,uuid,json!({"catalog":chunk}));}
}

pub fn tick(game:&mut Game<'_>) {
    if !game.native_on(ID) || game.remote.is_some() || game.player.is_none() {return;}
    let uuid=game.native_acting_id();
    let Some(world)=game.native_world_dir() else {return;};
    SERVER.with(|state| {
        let mut server=state.borrow_mut();
        if server.world!=world {*server=Server{world,..Server::default()};}
        if server.catalog.is_empty() {build_catalog(game,&mut server);}
        let Ok(mut player)=game.native_attachments(Holder::Player(uuid)).and_then(|a|a.get(&PLAYER_STATE)) else {return;};
        player.tick=player.tick.wrapping_add(1);
        if !player.granted && game.res.items().get(DEVICE).is_some() {
            if game.native_give_items(DEVICE,1)==1 {
                player.granted=true;
                notify(game,uuid,"Вы получили Апгрейдер. ПКМ с предметом или клавиша U — открыть.");
            }
        }
        if let Some(mut pending)=player.pending.take() {
            pending.ticks=pending.ticks.saturating_sub(1);
            if pending.ticks==0 {
                if !pending.revealed {
                    send(game,uuid,json!({"result":pending.win,"spin":0,"odds":pending.odds,"target":pending.target,"count":pending.requested}));
                    pending.revealed=true;
                }
                if pending.win {
                    let rest=game.native_give_stack(uuid,ItemStack::new(&pending.target,pending.count));
                    if let Some(rest)=rest {
                        // The payout remains owed until there is room; never discard an earned reward.
                        pending.count=rest.count;player.pending=Some(pending);
                        if player.tick%40==0 {notify(game,uuid,"Выигрыш ожидает выдачи: освободите место в инвентаре.");}
                    } else {
                        notify(game,uuid,"УСПЕХ! Выигрыш выдан в инвентарь.");
                    }
                } else {
                    notify(game,uuid,"ПРОВАЛ. Предметы ставки потеряны.");
                }
            } else {
                if pending.ticks%10==0 {send(game,uuid,json!({"spin":pending.ticks,"odds":pending.odds}));}
                player.pending=Some(pending);
            }
        }
        let requests=server.queue.remove(&uuid).unwrap_or_default();
        for request in requests.into_iter().take(4) {
            match request["action"].as_str().unwrap_or("") {
                "open" => {
                    if owns(game) {
                        send_catalog(game,uuid,&server);
                        let mut snapshot=json!({"open":true,"cooldown":player.limiter.remaining(now()),"spin":0});
                        if let Some(pending)=&player.pending {
                            snapshot["spin"]=pending.ticks.into();snapshot["odds"]=pending.odds.into();
                            snapshot["target"]=pending.target.as_str().into();snapshot["count"]=pending.requested.into();
                            if pending.revealed {snapshot["result"]=pending.win.into();}
                        }
                        send(game,uuid,snapshot);
                    } else {notify(game,uuid,"Чтобы открыть станцию, нужен предмет «Апгрейдер».");}
                }
                "roll" => {
                    if !owns(game) || player.pending.is_some() {continue;}
                    let wait=player.limiter.remaining(now());
                    if wait>0 {send(game,uuid,json!({"cooldown":wait,"message":format!("Лимит попыток. Подождите {} с.",wait.div_ceil(20))}));continue;}
                    let Some(slot)=request["slot"].as_u64().filter(|v|*v<36).map(|v|v as usize) else {continue;};
                    let Some(input)=game.inventory.get(slot).and_then(Option::as_ref).cloned() else {continue;};
                    // Components are not priced. Reject them instead of laundering enchantments,
                    // filled containers, renamed objects or damaged tools through a plain-item price.
                    if input.count==0 || input.extra.is_some() || input.damage!=0 {notify(game,uuid,"Для ставки нужен обычный неповреждённый предмет без компонентов.");continue;}
                    let Some(input_price)=server.prices.get(&*input.item).copied() else {notify(game,uuid,"Этот предмет нельзя использовать как ставку.");continue;};
                    let Some(target)=request["target"].as_str() else {continue;};
                    let Some(target_price)=server.prices.get(target).copied() else {continue;};
                    let Some(max)=game.res.items().get(target).map(|i|i.max_stack) else {continue;};
                    let count=core::target_count(request["count"].as_u64().unwrap_or(1).min(u64::from(u32::MAX)) as u32,max);
                    let odds=core::chance(input_price*f64::from(input.count),target_price,count);
                    if odds<=0.0 || !player.limiter.record(now()) {continue;}
                    let win=game.world.random.next_double()<odds;
                    // Consume the exact stack in the authoritative slot before recording a pending result.
                    game.inventory[slot]=None;
                    player.pending=Some(Pending{target:target.into(),count,requested:count,win,ticks:core::SPIN_TICKS,odds,revealed:false});
                    send(game,uuid,json!({"spin":core::SPIN_TICKS,"odds":odds,"target":target,"count":count,"cooldown":player.limiter.remaining(now()),"message":"Ставка принята. Результат через 4 секунды."}));
                }
                _=>{}
            }
        }
        if let Ok(attachments)=game.native_attachments(Holder::Player(uuid)) {
            if let Err(error)=attachments.set(&PLAYER_STATE,player) {log::error!("Upgrader player state: {error:?}");}
        }
    });
}

pub fn from_client(player:u128,channel:&str,data:&[u8]) {
    if channel!=CHANNEL || data.len()>1024 {return;}
    let Ok(value)=serde_json::from_slice::<Value>(data) else {return;};
    SERVER.with(|state|{
        let mut server=state.borrow_mut();
        let queue=server.queue.entry(player).or_default();
        if value["action"]=="open" && queue.iter().any(|v|v["action"]=="open") {return;}
        if queue.len()<4 {queue.push_back(value);}
    });
}
pub fn reload() {
    SERVER.with(|s|{let mut s=s.borrow_mut();s.catalog.clear();s.prices.clear();});
}
pub fn from_server(channel:&str,data:&[u8]) {
    if channel!=CHANNEL {return;}
    let Ok(value)=serde_json::from_slice::<Value>(data) else {return;};
    CLIENT.with(|state|{
        let mut c=state.borrow_mut();
        if value["catalog_reset"].as_bool()==Some(true) {c.catalog.clear();}
        if let Some(items)=value["catalog"].as_array() {if c.catalog.len()<4096 {c.catalog.extend(items.iter().cloned());}}
        if value["open"].as_bool()==Some(true) {c.open=true;}
        if let Some(v)=value["message"].as_str() {c.message=v.into();}
        if let Some(v)=value["odds"].as_f64() {c.odds=v;}
        if let Some(v)=value["target"].as_str() {c.target=v.into();}
        if let Some(v)=value["count"].as_u64() {c.count=v.clamp(1,64) as u32;}
        if let Some(v)=value["spin"].as_u64() {
            if v>0 && !c.wheel.running() {let odds=c.odds;c.wheel.begin(animation_time(),odds);}
            c.spin=v.min(80) as u32;
        }
        if let Some(v)=value["cooldown"].as_u64() {c.cooldown=v;}
        if let Some(v)=value["result"].as_bool() {
            c.message=if v{"УСПЕХ! Выигрыш получен."}else{"ПРОВАЛ. Ставка проиграна."}.into();
            let odds=c.odds;c.wheel.finish(animation_time(),odds,v);
        }
    });
}
pub fn frame(game:&mut Game<'_>) {
    if !game.native_on(ID) || !game.native_window() {return;}
    if game.player.is_none() {
        CLIENT.with(|s|*s.borrow_mut()=Client{count:1,..Client::default()});return;
    }
    let (open,queue)=CLIENT.with(|state|{
        let mut c=state.borrow_mut();
        c.wheel.update(animation_time());
        (std::mem::take(&mut c.open),std::mem::take(&mut c.queue))
    });
    if open {let _=game.native_mod_screen_open(ID);}
    for request in queue {
        if let Ok(data)=serde_json::to_vec(&request) {
            if let Err(error)=game.native_net_send_to_server(CHANNEL,&data) {
                log::debug!("Upgrader request: {error}");
                game.native_chat("Апгрейдер: сервер ещё не готов или на нём не установлен этот мод.");
            }
        }
    }
}
pub fn key(game:&mut Game<'_>,key:&str,pressed:bool) {
    if !pressed || !game.native_on(ID) || !game.native_window() || !game.native_playing() {return;}
    if key==game.native_mod_key(ID,"open","key.keyboard.u") {
        CLIENT.with(|s|s.borrow_mut().queue.push_back(json!({"action":"open"})));
    }
}
pub fn use_device(game:&mut Game<'_>,stack:&ItemStack,claimed:&mut bool) {
    if *claimed || &*stack.item!=DEVICE || !game.native_on(ID) {return;}
    *claimed=true;
    if game.remote.is_none() {
        let uuid=game.native_acting_id();
        from_client(uuid,CHANNEL,b"{\"action\":\"open\"}");
    } else {CLIENT.with(|s|s.borrow_mut().queue.push_back(json!({"action":"open"})));}
}

fn cut(game:&Game<'_>,text:&str,width:f32)->String {
    let mut s=text.to_owned();
    while game.native_text_width(&s)>width && !s.is_empty() {s.pop();}
    if s!=text {while game.native_text_width(&(s.clone()+"…"))>width && !s.is_empty() {s.pop();}s.push('…');}
    s
}
fn button(game:&Game<'_>,batch:&mut HudBatch,rect:[f32;4],text:&str,active:bool) {
    let scale=((batch.width-8.0)/390.0).min((batch.height-8.0)/260.0).clamp(0.1,1.0);
    let ox=(batch.width-390.0*scale)/2.0;let oy=(batch.height-260.0*scale)/2.0;
    let [mx,my]=game.native_cursor();let mx=(mx-ox)/scale+ox;let my=(my-oy)/scale+oy;
    let hover=mx>=rect[0] && mx<rect[0]+rect[2] && my>=rect[1] && my<rect[1]+rect[3];
    game.native_fill(batch,rect,if !active {0xff29323b}else if hover {0xff506677}else{0xff394b5d});
    game.native_text(batch,&cut(game,text,rect[2]-6.0),[rect[0]+3.0,rect[1]+5.0],if active {0xffffff}else{0x88929b},false);
}
fn visible(c:&Client)->Vec<&Value> {
    let query=c.search.to_lowercase();
    c.catalog.iter().filter(|i|query.is_empty() || i["id"].as_str().unwrap_or("").contains(&query) || i["name"].as_str().unwrap_or("").to_lowercase().contains(&query)).collect()
}
fn wheel_visible(c:&Client)->bool {
    c.spin>0 || c.wheel.visible(animation_time())
}
fn ring(game:&Game<'_>,batch:&mut HudBatch,center:[f32;2],inner:f32,outer:f32,start:f64,sweep:f64,color:u32) {
    let steps=((sweep.abs()/std::f64::consts::TAU*160.0).ceil() as usize).max(1);
    let point=|r:f32,a:f64|[center[0]+r*a.cos() as f32,center[1]+r*a.sin() as f32];
    for i in 0..steps {
        let a=start+sweep*i as f64/steps as f64;let b=start+sweep*(i+1) as f64/steps as f64;
        game.native_quad(batch,[point(inner,a),point(outer,a),point(outer,b),point(inner,b)],color);
    }
}
fn draw_wheel(game:&Game<'_>,batch:&mut HudBatch,c:&mut Client,x:f32,y:f32) {
    if !wheel_visible(c) {return;}
    let now=animation_time();c.wheel.update(now);
    let moving=c.wheel.running() || c.wheel.braking(now);
    game.native_fill(batch,[x+3.0,y+23.0,384.0,219.0],0xf5131c27);
    game.native_text(batch,if moving{"КОЛЕСО КРУТИТСЯ…"}else if c.wheel.result==Some(true){"ВЫИГРЫШ!"}else{"СТАВКА ПРОИГРАНА"},[x+20.0,y+34.0],0xffdc99,true);
    let center=[x+111.0,y+139.0];let tau=std::f64::consts::TAU;let top=-std::f64::consts::FRAC_PI_2;
    ring(game,batch,center,0.0,80.0,0.0,tau,0xffe5b866);
    ring(game,batch,center,0.0,76.5,0.0,tau,0xff101820);
    for i in 0..16 {ring(game,batch,center,47.0,73.0,top+c.wheel.angle+tau*i as f64/16.0,tau/16.0,if i%2==0{0xff9f3948}else{0xff6c293d});}
    ring(game,batch,center,47.0,73.0,top+c.wheel.angle,tau*c.odds.clamp(0.0,1.0),0xff36c397);
    ring(game,batch,center,44.5,47.0,0.0,tau,0xffe5b866);
    let chance=if c.odds>0.0 && c.odds*100.0<0.00001 {"<0.00001%".into()}else{format!("{:.5}%",c.odds*100.0)};
    game.native_text(batch,&chance,[center[0]-game.native_text_width(&chance)/2.0,center[1]-4.0],0xffffff,true);
    game.native_quad(batch,[[center[0]-7.0,center[1]-86.0],[center[0]+7.0,center[1]-86.0],[center[0],center[1]-66.0],[center[0],center[1]-66.0]],0xffffd77e);
    let rx=x+205.0;
    game.native_text(batch,"Зелёный сектор",[rx,y+88.0],0x65e6ba,false);
    game.native_text(batch,"— выигрыш",[rx,y+101.0],0xb9cbd7,false);
    game.native_text(batch,"Красный сектор",[rx,y+123.0],0xff8395,false);
    game.native_text(batch,"— проигрыш",[rx,y+136.0],0xb9cbd7,false);
    game.native_item(batch,&c.target,[rx+40.0,y+160.0],28.0);
    game.native_text(batch,&format!("× {}",c.count),[rx+78.0,y+171.0],0xffffff,true);
    game.native_text(batch,if moving{"Ждём остановки колеса"}else{"Результат подтверждён сервером"},[x+20.0,y+225.0],0xb9cbd7,false);
}
pub fn draw(game:&Game<'_>,batch:&mut HudBatch) {
    if !game.native_on(ID) || game.native_mod_screen()!=Some(ID) {return;}
    CLIENT.with(|state| {
        let mut c=state.borrow_mut();
        let scale=((batch.width-8.0)/390.0).min((batch.height-8.0)/260.0).clamp(0.1,1.0);
        let (w,h)=(390.0,260.0);
        let x=(batch.width-w*scale)/2.0;let y=(batch.height-h*scale)/2.0;
        c.origin=[x,y];c.panel=[w,h];c.scale=scale;
        game.native_fill(batch,[0.0,0.0,batch.width as f32,batch.height as f32],0xb5000000);
        let start=batch.vertices.len();
        game.native_fill(batch,[x,y,w,h],0xff17232e);
        game.native_text(batch,"АПГРЕЙДЕР · ХАРДКОР",[x+10.0,y+9.0],0xffc36b,true);
        game.native_text(batch,"Ставка: весь выбранный стак",[x+10.0,y+28.0],0xbccad5,false);
        for slot in 0..36 {
            let sx=x+10.0+(slot%9) as f32*18.0;let sy=y+44.0+(slot/9) as f32*18.0;
            game.native_fill(batch,[sx,sy,17.0,17.0],if c.selected==Some(slot){0xffbc863e}else{0xff2d3a46});
            if let Some(stack)=game.inventory.get(slot).and_then(Option::as_ref) {
                game.native_item_stack(batch,stack,[sx+1.0,sy+1.0],15.0);
                if stack.count>1 {game.native_text(batch,&stack.count.to_string(),[sx+6.0,sy+9.0],0xffffff,true);}
            }
        }
        let input=c.selected.and_then(|s|game.inventory.get(s)).and_then(Option::as_ref);
        let input_price=input.and_then(|s|c.catalog.iter().find(|v|v["id"].as_str()==Some(&*s.item)).and_then(|v|v["price"].as_f64()).map(|v|v*f64::from(s.count))).unwrap_or(0.0);
        game.native_text(batch,&format!("Ставка: {:.2}",input_price),[x+10.0,y+120.0],0xcdd8df,false);
        let target=c.catalog.iter().find(|v|v["id"].as_str()==Some(&c.target));
        let target_price=target.and_then(|v|v["price"].as_f64()).unwrap_or(0.0);
        let target_name=target.and_then(|v|v["name"].as_str()).unwrap_or("Выберите выигрыш справа");
        game.native_text(batch,&cut(game,target_name,175.0),[x+10.0,y+135.0],0xffdc99,false);
        button(game,batch,[x+10.0,y+151.0,24.0,19.0],"−",c.spin==0);
        button(game,batch,[x+38.0,y+151.0,24.0,19.0],"+",c.spin==0);
        button(game,batch,[x+66.0,y+151.0,35.0,19.0],"MAX",c.spin==0);
        game.native_text(batch,&format!("Кол-во: {}",c.count),[x+107.0,y+157.0],0xffffff,false);
        let odds=core::chance(input_price,target_price,c.count);
        let chance=if odds>0.0 && odds*100.0<0.00001 {"Шанс: <0.00001%".to_owned()}else{format!("Шанс: {:.5}%",odds*100.0)};
        game.native_text(batch,&chance,[x+10.0,y+179.0],0x7edbc7,false);
        let active=c.spin==0 && input_price>0.0 && target_price>0.0;
        button(game,batch,[x+10.0,y+194.0,172.0,22.0],if c.spin>0{"КРУТИМ…"}else{"РИСКНУТЬ СТАВКОЙ"},active);
        game.native_text(batch,&cut(game,&c.message,w-20.0),[x+10.0,y+h-14.0],0xffc36b,false);
        let rx=x+198.0;let rw=w-208.0;
        game.native_text(batch,&cut(game,&format!("Поиск: {}_",c.search),rw),[rx,y+28.0],0xcdd8df,false);
        let entries=visible(&c);let pages=entries.len().div_ceil(10).max(1);let page=c.page.min(pages-1);
        for (row,v) in entries.iter().skip(page*10).take(10).enumerate() {
            let sy=y+44.0+row as f32*17.0;let id=v["id"].as_str().unwrap_or("");
            game.native_fill(batch,[rx,sy,rw,16.0],if id==c.target{0xff5a482f}else{0xff263643});
            game.native_item(batch,id,[rx+1.0,sy+1.0],14.0);
            game.native_text(batch,&cut(game,v["name"].as_str().unwrap_or(id),rw-19.0),[rx+18.0,sy+4.0],0xe3ebef,false);
        }
        button(game,batch,[rx,y+218.0,22.0,18.0],"<",true);
        game.native_text(batch,&format!("{}/{}",page+1,pages),[rx+27.0,y+223.0],0xffffff,false);
        button(game,batch,[rx+rw-22.0,y+218.0,22.0,18.0],">",true);
        draw_wheel(game,batch,&mut c,x,y);
        batch.scale_since(start,x,y,scale,scale);
    });
}
pub fn mouse(game:&Game<'_>,screen:&NativeScreenView<'_>,button:i32,canceled:&mut bool) {
    if !game.native_on(ID) || game.native_mod_screen()!=Some(ID) || button!=0 {return;}
    *canceled=true;
    CLIENT.with(|state|{
        let mut c=state.borrow_mut();let [x,y]=c.origin;let scale=c.scale.max(0.1);let px=(screen.mouse[0]-x)/scale;let py=(screen.mouse[1]-y)/scale;
        if wheel_visible(&c) {return;}
        if (10.0..172.0).contains(&px) && (44.0..116.0).contains(&py) && c.spin==0 {
            c.selected=Some(((py-44.0)/18.0) as usize*9+((px-10.0)/18.0) as usize);return;
        }
        if (151.0..170.0).contains(&py) && c.spin==0 {
            let max=c.catalog.iter().find(|v|v["id"].as_str()==Some(&c.target)).and_then(|v|v["max"].as_u64()).unwrap_or(64) as u32;
            let max=core::target_count(64,max);
            if (10.0..34.0).contains(&px) {c.count=c.count.saturating_sub(1).max(1);}
            if (38.0..62.0).contains(&px) {c.count=(c.count+1).min(max);}
            if (66.0..101.0).contains(&px) {c.count=max;}
        }
        if (10.0..182.0).contains(&px) && (194.0..216.0).contains(&py) && c.spin==0 {
            if let Some(slot)=c.selected {
                let request=json!({"action":"roll","slot":slot,"target":c.target,"count":c.count});c.queue.push_back(request);
            }
        }
        if px>=198.0 && px<c.panel[0]-10.0 {
            let entries=visible(&c);let pages=entries.len().div_ceil(10).max(1);let page=c.page.min(pages-1);
            if (44.0..214.0).contains(&py) && c.spin==0 {
                if let Some(v)=entries.get(page*10+((py-44.0)/17.0) as usize) {
                    let id=v["id"].as_str().unwrap_or("").to_owned();c.target=id;c.count=1;
                }
            } else if (218.0..236.0).contains(&py) {
                if px<220.0 {c.page=page.saturating_sub(1);} else if px>=c.panel[0]-32.0 {c.page=(page+1).min(pages-1);}
            }
        }
    });
}
pub fn typed(game:&Game<'_>,c:char,canceled:&mut bool) {
    if game.native_mod_screen()!=Some(ID) || !game.native_on(ID) {return;}
    *canceled=true;CLIENT.with(|s|{let mut s=s.borrow_mut();if s.search.chars().count()<48 && !c.is_control(){s.search.push(c);s.page=0;}});
}
pub fn screen_key(game:&Game<'_>,key:&str,canceled:&mut bool) {
    if game.native_mod_screen()!=Some(ID) || !game.native_on(ID) {return;}
    if key=="key.keyboard.backspace" { *canceled=true;CLIENT.with(|s|{let mut s=s.borrow_mut();s.search.pop();s.page=0;}); }
}
pub fn mods(mods:&mut Vec<NativeMod>) {
    mods.push(NativeMod{id:ID.into(),title:"Апгрейдер: Хардкор 0.1.0".into(),description:"Ставьте предметы ради выбранного выигрыша. Предмет выдаётся при первом входе.".into(),keys:vec![["open".into(),"Открыть Апгрейдер".into(),"key.keyboard.u".into()]]});
}
