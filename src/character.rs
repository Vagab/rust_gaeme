use crate::gravity_affected::GravityAffected;

pub struct Character {
    pub x: f32;
    pub y: f32;
}

impl GravityAffected for Character {
    fn fall(&mut self, step: f32) {

        self.y += step;
        if self.y > HEIGHT {
            step = 0;
        } else if self.y < y_start {
            step = 0;
        }
    }
}

impl Character {

    pub fn new() -> Self {
        Self {
            x: WIDTH/2.;
            y: HEIGHT/2.;
        }
    }

}
