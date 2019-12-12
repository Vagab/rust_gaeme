use ggez::event::{EventHandler, run};
use ggez::{Context, GameResult, ContextBuilder};
use ggez::graphics::{clear, present, WHITE, Mesh, DrawMode, FillOptions, Rect, BLACK, draw, DrawParam, MeshBuilder};
use ggez::conf::{Conf, WindowMode, FullscreenType};
use crate::{WIDTH, HEIGHT};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use crate::r_drop::RDrop;
use std::f32::consts::E;
use std::f32;
use std::collections::HashMap;
use ggez::graphics::spritebatch::SpriteBatch;

pub struct RainMaker {
    rng: ThreadRng,

    drops: Vec<RDrop>,
}

impl EventHandler for RainMaker {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for drop in &mut self.drops {
            drop.fall()
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, WHITE);

        let mut map = HashMap::new();

        for &d in &self.drops {
            map.entry(d.z).or_insert(vec![]).push(d)
        }

        for (z, drops) in map {

            let mut builder = MeshBuilder::new();
            for d in drops {
                let (w, h) = d.get_wh();
                builder.rectangle(
                    DrawMode::Fill(FillOptions::DEFAULT),
                    Rect { x: d.x, y: d.y, w, h },
                    BLACK,
                );
            }

            let mesh = builder.build(ctx)?;
            draw(ctx, &mesh, DrawParam::default());
        }

        present(ctx)
    }
}

impl RainMaker {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),

            drops: vec![],
        }
    }

    pub fn generate(&mut self) {
        for _ in 0..10000 {
            self.drops.push(RDrop::new(&mut self.rng));
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
