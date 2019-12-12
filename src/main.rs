#[macro_use] extern crate derive_deref;
#[macro_use] extern crate derive_more;

mod r_drop;
mod rain_maker;
mod f_point;

use rain_maker::RainMaker;

const WIDTH: f32 = 1200.;
const HEIGHT: f32 = 800.;
const STEP: f32 = 1.;

fn main() {
    let mut rain_maker = RainMaker::new();
    rain_maker.generate();
    rain_maker.run();
}
