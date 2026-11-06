// WYR extends the frog's food sensor to affected living players.
#[derive(Clone,Debug,Default)]
struct WyrEatPlayer { uuid:Option<u128>, ticks:i32, caught:bool }
impl WyrEatPlayer {
    fn target<'a>(&self,w:&'a MobWorld)->Option<&'a super::PlayerView> {
        w.player.as_ref().filter(|p|Some(crate::lan::playerdata::ints_uuid(p.uuid))==self.uuid && p.alive && !p.creative && !p.spectator && horizon_world::wyr_shared::player_count(crate::lan::playerdata::ints_uuid(p.uuid),"frog_food")>0)
    }
}
impl Goal for WyrEatPlayer {
    goal_clone!();
    fn flags(&self)->u8 {MOVE|LOOK}
    fn every_tick(&self)->bool {true}
    fn can_use(&mut self,m:&mut Mob,cx:&mut Cx)->bool {
        self.uuid=None;
        if m.ai.panicking {return false;}
        let Some(p)=cx.w.player.as_ref().filter(|p|p.alive&&!p.creative&&!p.spectator) else{return false;};
        let uuid=crate::lan::playerdata::ints_uuid(p.uuid);
        if horizon_world::wyr_shared::player_count(uuid,"frog_food")==0 || p.position.distance_squared(m.position)>DETECTION_DISTANCE*DETECTION_DISTANCE {return false;}
        if !cx.w.line_of_sight(Vec3::new(m.position.x,m.eye_y(),m.position.z),Vec3::new(p.eye[0],p.eye[1],p.eye[2])) {return false;}
        self.uuid=Some(uuid);true
    }
    fn can_continue(&mut self,m:&mut Mob,cx:&mut Cx)->bool {self.ticks<TONGUE_TIME_OUT&&!m.ai.panicking&&self.target(cx.w).is_some()}
    fn start(&mut self,m:&mut Mob,_cx:&mut Cx) {self.ticks=0;self.caught=false;m.extra.frog.tongue_target=self.uuid.and_then(horizon_world::wyr_shared::entity_for).map(|id|id as u32);}
    fn stop(&mut self,m:&mut Mob,_cx:&mut Cx) {self.uuid=None;m.extra.frog.tongue_target=None;set_pose(m,sync::STANDING);}
    fn tick(&mut self,m:&mut Mob,cx:&mut Cx) {
        let Some(p)=self.target(cx.w) else{return;};
        let pos=p.position;let eye=p.eye;m.look_at(eye);
        if !self.caught {
            if pos.distance_squared(m.position)<EATING_DISTANCE*EATING_DISTANCE {
                self.caught=true;self.ticks=0;set_pose(m,sync::USING_TONGUE);
                cx.events.push(MobEvent::entity_sound(m,"entity.frog.tongue",6,2.0,1.0));
                let v=m.position.subtract(pos).normalize().scale(0.75);
                cx.events.push(MobEvent::SetVelocity{target:Who::Player,velocity:[v.x,v.y,v.z]});
            } else if self.ticks%10==0 {m.nav_move_to_entity(cx.w,[pos.x,pos.y,pos.z],0,2.0);}
        } else if self.ticks==6 {
            cx.events.push(MobEvent::entity_sound(m,"entity.frog.eat",6,2.0,1.0));
            super::ai::do_hurt_target(m,Who::Player,cx);
        } else if self.ticks>=10 {self.ticks=TONGUE_TIME_OUT;return;}
        self.ticks+=1;
    }
}
