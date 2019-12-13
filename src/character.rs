use crate::gravity_affected::GravityAffected;
use crate::{WIDTH, HEIGHT};


#[derive(Copy, Clone)]
pub struct Character {
    pub x: f32,
    pub y: f32,
}

impl GravityAffected for Character {
    fn fall(&mut self, step: f32) {
        if self.y > HEIGHT || self.y < 0. {
            return
        }
        self.y += step;
    }
}

impl Character {

    pub fn new() -> Self {
        Self {
            x: 100f32,
            y: HEIGHT/2.,
        }
    }

}
