extern crate self as horizon_world;
pub mod wyr_shared {
    use std::cell::Cell;
    thread_local!{pub static SHAKY:Cell<u32>=const{Cell::new(0)};pub static RANDOM:Cell<u32>=const{Cell::new(0)};}
    pub fn client_count(id:&str)->u32 {match id {"shaky_buttons"=>SHAKY.get(),"random_shaders"=>RANDOM.get(),_=>0}}
}
mod visuals { include!("render_helpers.rs"); }
#[test]
fn geometry_matches_hits_and_moves_once_per_frame() {
    wyr_shared::SHAKY.set(5);visuals::begin_screen("inventory");visuals::end_frame(1);
    let base=[10.0,20.0,100.0,20.0];let drawn=visuals::jitter(base);
    assert_eq!(drawn,visuals::jitter(base));assert_eq!(drawn,visuals::effective_rect(base));
    assert!((drawn[0]-base[0]).abs()<=5.0);assert!((drawn[1]-base[1]).abs()<=5.0);
    visuals::end_frame(2);let next=visuals::jitter(base);
    assert!((next[0]-drawn[0]).abs()<=5.0);assert!((next[1]-drawn[1]).abs()<=5.0);
    assert_eq!(&next[2..],&base[2..]);
}
#[test]
fn disabled_restores_original_geometry_and_clears_shader() {
    wyr_shared::SHAKY.set(4);visuals::begin_screen("a");visuals::end_frame(1);
    let rect=[2.0,3.0,50.0,20.0];visuals::jitter(rect);
    wyr_shared::SHAKY.set(0);visuals::end_frame(2);
    assert_eq!(rect,visuals::effective_rect(rect));assert_eq!(rect,visuals::jitter(rect));
    wyr_shared::RANDOM.set(1);let first=visuals::random_effect().unwrap();
    assert!(first.starts_with("wyr:random_"));assert_eq!(Some(first),visuals::random_effect());
    wyr_shared::RANDOM.set(0);assert_eq!(None,visuals::random_effect());
}
