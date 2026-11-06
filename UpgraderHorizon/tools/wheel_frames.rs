#![allow(dead_code)]
// Developer-only preview driver: use the exact production animation math.
#[path = "../src/core.rs"]
mod economy;
fn main() {
    let odds=economy::chance(10.0*80.0,2_000.0,1);
    for won in [false,true] {
        let mut wheel=economy::WheelAnimation::default();wheel.begin(0.0,odds);
        for frame in 0..=102 {
            let time=frame as f64/12.5;
            if time>=4.0 {wheel.finish(4.0,odds,won);}
            wheel.update(time);
            println!("{},{frame},{time},{},{},{}",u8::from(won),wheel.angle,
                u8::from(wheel.running()||wheel.braking(time)),u8::from(wheel.visible(time)));
        }
    }
}
