// WYR: persistent positions for immediate-mode buttons, and local post-effect selection.
use std::{cell::RefCell, collections::BTreeMap, time::{Instant,SystemTime,UNIX_EPOCH}};

#[derive(Default)]
struct Buttons {
    frame:u64,
    screen:String,
    random:u64,
    positions:BTreeMap<[u32;4],([f32;2],u64)>,
}
impl Buttons {
    fn draw(&mut self,amount:u32)->i32 {
        if self.random==0 {self.random=0xd1b54a32d192ed03;}
        self.random^=self.random<<13;self.random^=self.random>>7;self.random^=self.random<<17;
        (self.random%u64::from(amount*2+1)) as i32-amount as i32
    }
}
thread_local! {
    static BUTTONS:RefCell<Buttons>=RefCell::new(Buttons::default());
    static EFFECT:RefCell<Option<(Instant,u64,usize)>>=const{RefCell::new(None)};
}
pub fn begin_screen(id:&str) {
    BUTTONS.with(|s|{let mut s=s.borrow_mut();if s.screen!=id {s.screen=id.into();s.positions.clear();}});
}
pub fn end_frame(frame:u64) {
    if horizon_world::wyr_shared::client_count("random_shaders")==0 {EFFECT.with(|s|*s.borrow_mut()=None);}
    BUTTONS.with(|s|{
        let mut s=s.borrow_mut();
        if horizon_world::wyr_shared::client_count("shaky_buttons")==0 {s.positions.clear();}
        if s.frame!=frame {s.positions.retain(|_,(_,seen)|seen.saturating_add(2)>=frame);s.frame=frame;}
    });
}
pub fn jitter(rect:[f32;4])->[f32;4] {
    let amount=horizon_world::wyr_shared::client_count("shaky_buttons").min(1000);
    if amount==0 {return rect;}
    BUTTONS.with(|s|{
        let mut s=s.borrow_mut();let key=rect.map(f32::to_bits);let frame=s.frame;
        if s.positions.get(&key).is_none_or(|(_,seen)|*seen!=frame) {
            let dx=s.draw(amount) as f32;let dy=s.draw(amount) as f32;
            let entry=s.positions.entry(key).or_insert(([0.0;2],frame));
            entry.0[0]=(entry.0[0]+dx).clamp(-1_000_000.0,1_000_000.0);
            entry.0[1]=(entry.0[1]+dy).clamp(-1_000_000.0,1_000_000.0);entry.1=frame;
        }
        let delta=s.positions[&key].0;
        [rect[0]+delta[0],rect[1]+delta[1],rect[2],rect[3]]
    })
}
// Hover/click tests and controller registration use the SAME last drawn rectangle.
// Looking it up does not draw a random number or move a widget again.
pub fn effective_rect(rect:[f32;4])->[f32;4] {
    if horizon_world::wyr_shared::client_count("shaky_buttons")==0 {return rect;}
    BUTTONS.with(|s|{
        let s=s.borrow();
        if let Some((delta,_))=s.positions.get(&rect.map(f32::to_bits)) {
            [rect[0]+delta[0],rect[1]+delta[1],rect[2],rect[3]]
        }else{rect}
    })
}
pub fn random_effect()->Option<String> {
    if horizon_world::wyr_shared::client_count("random_shaders")==0 {
        EFFECT.with(|s|*s.borrow_mut()=None);return None;
    }
    EFFECT.with(|s|{
        let mut s=s.borrow_mut();
        let current=s.get_or_insert_with(||{
            let seed=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as u64;
            (Instant::now(),seed.max(1),6)
        });
        let slot=current.0.elapsed().as_secs()/10;
        // Keep the draw fixed for this ten-second interval, even over pause/options screens.
        if current.2==6 || (current.1>>32)!=slot {
            let old=current.2;
            let mut random=(current.1&0xffff_ffff)^slot.wrapping_mul(0x9e3779b97f4a7c15);
            random^=random<<13;random^=random>>7;random^=random<<17;
            let mut mode=(random%6) as usize;
            if mode==old {mode=(mode+1)%6;}
            current.1=(slot<<32)|(random&0xffff_ffff);current.2=mode;
        }
        Some(format!("wyr:random_{}",current.2))
    })
}
