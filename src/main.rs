mod r_drop;
mod event_handler;

use event_handler::RainMaker;

const WIDTH: f32 = 200.;
const HEIGHT: f32 = 100.;

fn main() {
    let mut rain_maker = RainMaker::new();
    rain_maker.run();
}
