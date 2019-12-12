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
use std::cmp::{min, max};

pub struct RainMaker {
    rng: ThreadRng,

    drops: HashMap<i8, Vec<RDrop>>,
}

impl EventHandler for RainMaker {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for (_, drops) in &mut self.drops {
            for d in drops {
                d.fall()
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, BLACK);

        for (z, drops) in &self.drops {
            let mut builder = MeshBuilder::new();
            for d in drops {
                let (w, h) = d.get_wh();
                builder.rectangle(
                    DrawMode::Fill(FillOptions::DEFAULT),
                    Rect { x: d.x, y: d.y, w, h },
                    WHITE,
                );
            }

            let mesh = builder.build(ctx)?;
            draw(ctx, &mesh, DrawParam::default());
        }

        present(ctx)
    }
}

impl RainMaker {
    // drops are evenly spread across layers
    pub fn new(layers: u8, n_drops: usize) -> Self {
        let drops_per_layer = max(1, n_drops / layers as usize);

        let mut rng = thread_rng();
        let drops = (0..layers).map(|u| u as i8)
            .map(|l| {
                let ds = (0..drops_per_layer)
                    .map(|_| RDrop::new(&mut rng))
                    .collect();
                (l, ds)
            }).collect();
        Self {
            rng,
            drops,
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
