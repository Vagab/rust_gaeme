#[macro_use] extern crate derive_deref;
#[macro_use] extern crate derive_more;

mod backgrounds;
mod vector_point;
mod game;

use backgrounds::rain_maker::RainMaker;
use ggez::event::run;
use ggez::conf::{WindowMode, FullscreenType};
use ggez::{ContextBuilder, conf};
use crate::game::Game;
use crate::game::character::Character;

const WIDTH: f32 = 1500.;
const HEIGHT: f32 = 900.;

fn main() {
    let wm = WindowMode {
        width: WIDTH,
        height: HEIGHT,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.,
        min_height: 0.,
        max_width: 0.,
        max_height: 0.,
        resizable: false
    };
    let (ref mut ctx, ref mut event_loop)
        = ContextBuilder::new("game", "author")
        .conf(conf::Conf::new())
        .window_mode(wm)
        .build()
        .unwrap();


    let mut game = Game {
        background: RainMaker::new(1000),
        character: Character::new(),
        step: 7.,
    };
    run(ctx, event_loop, &mut game).expect("crashed");
}
