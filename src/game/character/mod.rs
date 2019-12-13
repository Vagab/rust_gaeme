pub mod gravity_affected;

use crate::{WIDTH, HEIGHT};
use crate::game::character::gravity_affected::GravityAffected;

pub struct Character {
    pub x: f32,
    pub y: f32,
}

impl GravityAffected for Character {
    fn fall(&mut self, step: f32) {
        if self.y > HEIGHT || self.y < 0. { return }
        self.y += step;
    }
}

impl Character {
    pub fn new() -> Self {
        Self {
            x: 100.,
            y: HEIGHT / 2.,
        }
    }
}
