mod r_drop;
mod rain_maker;

use rain_maker::RainMaker;

const WIDTH: f32 = 1200.;
const HEIGHT: f32 = 800.;
const STEP: f32 = 1e-5;

fn main() {
    let mut rain_maker = RainMaker::new();
    rain_maker.run();
}
