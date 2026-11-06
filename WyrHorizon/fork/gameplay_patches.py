"""Additional original WYR effects for our isolated Horizon source build."""
from pathlib import Path

def apply(root: Path):
    def patch(file, before, after):
        path=root/file
        text=path.read_text(encoding='utf8')
        if after in text:return
        expected=2 if file.endswith('collisions.rs') else 1
        if text.count(before)!=expected:raise RuntimeError(f'{file}: expected {expected} patch anchors')
        path.write_text(text.replace(before,after,1),encoding='utf8')
    base='crates/horizon-client/src/'
    patch(base+'game/mounts.rs','        Species::Pig => 0.86875,','        Species::Pig => 0.86875,\n        Species::Enderman => 2.775,')
    patch(base+'mobs/frog.rs','    g.add(2, ShootTongue);','    g.add(2, WyrEatPlayer::default());\n    g.add(2, ShootTongue);')
    frog=root/(base+'mobs/frog.rs')
    text=frog.read_text(encoding='utf8')
    if 'struct WyrEatPlayer' not in text:
        frog.write_text(text+'\n'+(Path(__file__).parent/'frog_player_goal.rs').read_text(encoding='utf8'),encoding='utf8')
    patch(base+'motion.rs','    fn web_inside(&mut self) {','    fn web_inside(&mut self) {\n        if horizon_world::wyr_shared::current_count("no_cobweb_collision")>0 {return;}')
    patch('crates/horizon-client-render/src/collisions.rs',
          '        self.binding\n            .collision_shape_with_context(self, block.state, p, context)',
          '        if context.entity().is_some_and(|e|e.0==0) && horizon_world::wyr_shared::current_count("noclip_shovel_mineable_blocks")>0 && horizon_world::wyr_shared::shovel(block.state.0) {return Ok(VoxelShape::empty());}\n        self.binding\n            .collision_shape_with_context(self, block.state, p, context)')
    patch(base+'mobs/equine.rs','            0.225\n','            0.225 + 4.0 * f64::from(horizon_world::wyr_shared::vehicle_count(m.id,"faster_pigs"))\n')
    patch(base+'mobs/ai.rs','        m.climbing = m.horizontal_collision;',
          '        let disabled=m.ai.target==Some(Who::Player) && w.player.as_ref().is_some_and(|p|horizon_world::wyr_shared::player_count(crate::lan::playerdata::ints_uuid(p.uuid),"no_spider_climbing")>0);\n        m.climbing = m.horizontal_collision && !disabled;')
    patch(base+'game/boats.rs','        b.status = self.boat_status(&mut b);',
          '        b.status = self.boat_status(&mut b);\n        if matches!(b.status,Status::InWater|Status::UnderWater) && horizon_world::wyr_shared::vehicle_count(b.id,"sink_boats")>0 {b.status=Status::UnderFlowingWater;}')
    patch(base+'game/mounts.rs','        if !self.native_may_leave(id) {',
          '        if !self.creative && !self.spectator() && super::wyr::has(self,"enderman_carrying") && self.mobs.iter().any(|m|m.id==id&&m.alive()&&m.species()==Species::Enderman) {return;}\n        if !self.native_may_leave(id) {')
    patch(base+'game/creatures.rs','                    let result = match species {',
          '                    let result = if species==Species::Creeper && super::wyr::fork_features::creeper_flowers(self,at) {Ok(())} else {match species {')
    patch(base+'game/creatures.rs','                        _ => self.explode(at, radius),\n                    };',
          '                        _ => self.explode(at, radius),\n                    }};')
    patch(base+'game/lan_actions.rs','        let swap = self.enter_guest(host, conn, self.dim)?;',
          '        let actor=host.hub.guests.iter().find(|g|g.conn==conn).map(|g|g.uuid);\n        let _wyr_actor=horizon_world::wyr_shared::enter_actor(actor);\n        let swap = self.enter_guest(host, conn, self.dim)?;')
