use ggez::event::{EventHandler, run};
use ggez::{Context, GameResult, ContextBuilder};
use ggez::graphics::{clear, present, WHITE, Mesh, DrawMode, FillOptions, Rect, BLACK, draw, DrawParam};
use ggez::conf::{Conf, WindowMode, FullscreenType};
use crate::{WIDTH, HEIGHT};
use rand::rngs::ThreadRng;
use rand::thread_rng;

pub struct RainMaker {
    rng: ThreadRng,

}

impl EventHandler for RainMaker {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // make it rain
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, WHITE);
        let mut  x = 0.;
        let mut y = 0.;
        let z = 0u8;

        let center_x = WIDTH / 2.;
        let center_y = HEIGHT / 2.;

//        x =

        let w = 10.;
        let h = 100.;

        let mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::DEFAULT),
            Rect { x, y, w, h },
            BLACK,
        )?;

        draw(ctx, &mesh, DrawParam::default());
        present(ctx)
    }
}

impl RainMaker {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
        }
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