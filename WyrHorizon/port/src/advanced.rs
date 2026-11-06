use crate::game::Game;
use horizon_client::items::ItemStack;
use horizon_core::random::RandomSource;

pub fn enter_action(game:&Game<'_>) {
    super::SERVER.with(|s|s.borrow_mut().current_actor=super::local(game));
}
pub fn current_count(id:&str)->u32 {
    super::SERVER.with(|s|{let s=s.borrow();s.current_actor.and_then(|id|s.players.get(&id)).map_or(0,|p|p.count(id))})
}
pub fn fishing_table(value:&'static str)->&'static str {
    if current_count("treasure_fishing_loot")>0 {"minecraft:gameplay/fishing/treasure"}else{value}
}
pub fn tame_roll(value:bool)->bool {value || current_count("instant_taming")>0}
pub fn tame_interact(game:&mut Game<'_>,entity:i32,stack:Option<&ItemStack>,claimed:&mut bool) {
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

pub fn egg_impact(game:&mut Game<'_>,dimension:&str,kind:&str,owner:u32,at:[f64;3],canceled:bool) {
    if canceled || kind!="minecraft:egg" || !game.native_on(super::ID) {return;}
    let player=game.native_server_players().into_iter().find(|p|p.entity_id==owner as i32&&p.dimension==dimension&&p.health>0.0);
    if let Some(player)=player {
        if super::server_player(player.uuid).has("egg_ender_pearl") {
            super::queue(player.uuid,format!("tp @s {} {} {}",at[0],at[1],at[2]));
            super::queue(player.uuid,"damage @s 5 minecraft:fall".into());
        }
    }
}
pub fn barter(game:&mut Game<'_>,mob:u32,value:Vec<ItemStack>)->Vec<ItemStack> {
    if !game.native_on(super::ID) {return value;}
    if !game.mobs.iter().any(|m|m.id==mob&&m.alive()) {return value;}
    let enabled=game.native_server_players().iter().any(|p|p.dimension==game.dim.id()&&p.health>0.0&&p.game_mode!=3&&super::server_player(p.uuid).has("piglins_barter_ender_eyes"));
    if enabled {let count=1+game.world.random.next_int_bound(4).unwrap_or(0) as u32;vec![ItemStack::new("minecraft:ender_eye",count)]}else{value}
}
pub fn nickname_tick(game:&mut Game<'_>) {
    if game.native_window() {return;}
    for player in game.native_server_players() {
        let enabled=game.native_on(super::ID)&&super::server_player(player.uuid).has("mvp");
        let changed=super::SERVER.with(|s| {let mut s=s.borrow_mut();if enabled {s.mvp.insert(player.uuid)}else{s.mvp.remove(&player.uuid)}});
        if changed {let name=format!("[MVP] {}",player.name);game.native_set_nickname(player.uuid,if enabled{Some(&name)}else{None});}
    }
}
pub fn level_pre(game:&Game<'_>,dimension:&str) {
    let enabled=game.native_on(super::ID)&&game.native_server_players().iter().any(|p|p.dimension==dimension&&p.health>0.0&&p.game_mode!=3&&super::server_player(p.uuid).has("piglins_barter_ender_eyes"));
    horizon_world::wyr_shared::set_barter(enabled);
}
pub fn mounted_tame(game:&mut Game<'_>) {
    if game.native_window() || !super::has(game,"instant_taming") {return;}
    let Some(index)=game.mount_index() else{return;};
    let Some(owner)=super::local(game) else{return;};
    let m=&mut game.mobs[index];
    if horizon_client::mobs::equine::is_equine(m.species()) && !m.extra.tame {
        m.extra.tame=true;m.extra.owner=Some(horizon_client::lan::playerdata::uuid_ints(owner));m.persistent=true;m.needs_sync=true;
        let id=m.id;let kind=m.kind.id.clone();game.mob_event_particles(id,7);game.native_entity_tame(id,&kind,Some(owner));
    }
}
