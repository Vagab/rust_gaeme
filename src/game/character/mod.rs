pub mod gravity_affected;

use crate::{WIDTH, HEIGHT};

pub struct Character {
    pub x: f32,
    pub y: f32,
}

impl Character {
    pub fn new() -> Self {
        Self {
            x: WIDTH / 2.,
            y: HEIGHT / 2.,
        }
    }
}