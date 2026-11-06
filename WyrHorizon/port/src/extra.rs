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
pub fn interact_entity(
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
pub fn toggle_sneak(game:&mut Game<'_>,sneaking:bool) {
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
pub fn right_click_block(game:&mut Game<'_>,pos:[i32;3],canceled:&mut bool) {
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
pub fn loot_drops(game:&mut Game<'_>,context:&horizon_client::mod_api::NativeLootDrops<'_>) {
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
pub fn use_item_start(game:&mut Game<'_>,stack:&ItemStack,canceled:&mut bool) {
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
pub fn player_tick(game:&mut Game<'_>) {
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
