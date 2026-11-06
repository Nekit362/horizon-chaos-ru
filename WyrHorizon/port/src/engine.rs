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
pub fn save(_game: &Game<'_>) {}
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
pub fn server_tick(game: &mut Game<'_>, phase: horizon_client::mod_api::ServerTickPhase) {
    if !matches!(
        phase,
        horizon_client::mod_api::ServerTickPhase::ServerTickEnd { .. }
    ) {
        return;
    }
    load(game);
    advanced::nickname_tick(game);
    horizon_world::wyr_shared::set_host_uuid(game.lan.as_ref().and_then(|h|h.hub.singleplayer_owner));
    catastrophes::refresh_host(game);
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
pub fn receive_server(game: &Game<'_>, player: u128, channel: &str, bytes: &[u8]) {
    if channel != CHANNEL || bytes.len() > 256 || !game.native_on(ID) {
        return;
    }
    if let Ok(request) = serde_json::from_slice::<Request>(bytes) {
        SERVER.with(|s| {
            s.borrow_mut().intents.insert(player, request);
        });
    }
}
pub fn receive_client(game: &Game<'_>, channel: &str, bytes: &[u8]) {
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
pub fn reset_client() {
    catastrophes::clear();
    horizon_world::wyr_shared::set_client(Default::default());
    CLIENT.with(|c| *c.borrow_mut() = Client::default());
}
pub fn tick_player(game: &mut Game<'_>) {
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

pub fn damage(entity: i32, kind: &str, ty: &str, amount: &mut f32, canceled: &mut bool) {
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
pub fn knockback(attacker: i32, strength: &mut f64) {
    let p = by_entity(attacker);
    *strength *= 1.0 + f64::from(p.count("knockback"));
}
pub fn right_click(game: &mut Game<'_>, pos: [i32; 3], face: [i32; 3], canceled: &mut bool) {
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
pub fn placed(game: &mut Game<'_>) {
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
pub fn can_eat(game: &Game<'_>, stack: &ItemStack, can: &mut bool) {
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
pub fn item_damage(game: &Game<'_>, amount: &mut i32, cancel: &mut bool) {
    if has(game, "unbreakable_tools") {
        *amount = 0;
        *cancel = true;
    }
}
pub fn shield_angle(game: &Game<'_>, value: f64) -> f64 {
    if has(game, "total_shield_blocking") { 0.0 } else { value }
}
pub fn consumed(game: &mut Game<'_>, stack: &ItemStack, result: &mut Option<ItemStack>) {
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
pub fn jumped(game: &mut Game<'_>) {
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
pub fn loot(
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
pub fn respawn(game: &mut Game<'_>, id: u128) {
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

pub fn frame(game: &mut Game<'_>) {
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
pub fn key(game: &mut Game<'_>, key: &str, pressed: bool) {
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
pub fn draw(game: &Game<'_>, batch: &mut HudBatch) {
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
pub fn mouse(game: &Game<'_>, button: i32, canceled: &mut bool) {
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
pub fn hud(game: &Game<'_>, batch: &mut HudBatch) {
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
pub fn layer_pre(layer: &str, batch: &HudBatch) {
    if layer == "minecraft:crosshair" {
        CLIENT.with(|c| c.borrow_mut().crosshair_start = Some(batch.vertices.len()));
    }
}
pub fn layer_post(layer: &str, batch: &mut HudBatch) {
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
pub fn movement(forward: &mut f32, strafe: &mut f32) {
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
pub fn fov(game: &Game<'_>, degrees: &mut f32) {
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
pub fn gui_layer(layer: &str, canceled: &mut bool) {
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
pub fn health_hud(
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
pub fn command(game: &mut Game<'_>, command: &str, ran: &mut bool) {
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
pub fn listed(mods: &mut Vec<horizon_client::mod_api::NativeMod>) {
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
