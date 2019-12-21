use ggez::event::{EventHandler, KeyMods};
use ggez::{Context, GameError};
use ggez::input::keyboard::KeyCode;
use ggez::graphics::{MeshBuilder, draw, DrawMode, FillOptions, Rect, BLACK, DrawParam, clear, WHITE, present, Color};
use crate::HEIGHT;
use crate::game::character::Character;
use mint::Point2;

pub mod character;

pub struct Game<B: EventHandler> {
    pub background: B,
    pub character: Character,
    pub step: f32,
}

impl<B: EventHandler> EventHandler for Game<B> {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.background.update(ctx)?;
        self.character.fall(self.step);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        clear(ctx, WHITE);

        self.background.draw(ctx)?;

        let mut builder = MeshBuilder::new();

        builder.rectangle(
            DrawMode::Fill(FillOptions::DEFAULT),
            Rect {
                x: self.character.x,
                y: self.character.y,
                w: 100.,
                h: 100.,
            },
            Color::new(1., 1., 0., 1.),
        );

        let mesh = builder.build(ctx)?;
        draw(ctx, &mesh, DrawParam::default())?;
        present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _repeat: bool) {
        self.background.key_down_event(ctx, key, mods, _repeat)
    }
}