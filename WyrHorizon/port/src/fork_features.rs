use crate::game::Game;
use horizon_client::items::ItemStack;
use horizon_core::random::RandomSource;

pub fn tick(game:&mut Game<'_>) {
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
pub fn creeper_flowers(game:&mut Game<'_>,at:[f64;3])->bool {
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
pub fn prepare_shovel(game:&Game<'_>) {
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
