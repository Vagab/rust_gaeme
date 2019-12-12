mod r_drop;
mod rain_maker;

use rain_maker::RainMaker;

const WIDTH: f32 = 1000.;
const HEIGHT: f32 = 700.;

fn main() {
    let mut rain_maker = RainMaker::new();
    rain_maker.run();
}
