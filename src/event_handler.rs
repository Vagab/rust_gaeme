use ggez::event::{EventHandler, run};
use ggez::{Context, GameResult, ContextBuilder};
use ggez::graphics::{clear, present, WHITE};
use ggez::conf::{Conf, WindowMode, FullscreenType};
use crate::{WIDTH, HEIGHT};

pub struct RainMaker; // add rain

impl EventHandler for RainMaker {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // make it rain
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, WHITE);
        // draw shit
        present(ctx)
    }
}

impl RainMaker {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) -> GameResult<()> {
        let (ref mut ctx, ref mut event_loop)
            = ContextBuilder::new("gæme", "")
                .conf(Conf::new())
                .window_mode(WindowMode {
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
                })
                .build()
                .unwrap();

        run(ctx, event_loop, self)
    }
}
