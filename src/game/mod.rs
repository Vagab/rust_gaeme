use ggez::event::{EventHandler, KeyMods};
use ggez::{Context, GameError};
use ggez::input::keyboard::KeyCode;

pub mod character;

pub struct Game<B: EventHandler> {
    pub background: B,
}

impl<B: EventHandler> EventHandler for Game<B> {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.background.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.background.draw(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _repeat: bool) {
        self.background.key_down_event(ctx, key, mods, _repeat)
    }
}