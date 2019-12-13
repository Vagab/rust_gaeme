#[macro_use] extern crate derive_deref;
#[macro_use] extern crate derive_more;

mod r_drop;
mod rain_maker;
mod f_point;

use rain_maker::RainMaker;

const WIDTH: f32 = 1500.;
const HEIGHT: f32 = 900.;
//const STEP: f32 = 7.;

fn main() {
    let mut rain_maker = RainMaker::new(10000);
    rain_maker.run().expect("something went wrong");
}
