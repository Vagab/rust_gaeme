extern crate rand;
use rand::{thread_rng, prelude::ThreadRng, Rng, distributions::{Normal, Distribution}};
use crate::{WIDTH, HEIGHT, STEP};

#[derive(Copy, Clone)]
pub struct RDrop {
    pub x: f32,
    pub y: f32,
    pub z: i8,
}

impl RDrop {

    pub fn new(r: &mut ThreadRng) -> Self {
        Self {
            x: r.gen_range(0., WIDTH),
            y: r.gen_range(-200., HEIGHT),
            z: r.gen_range(0, 6),
        }
    }

    pub fn fall(&mut self) {
        self.y += STEP / self.z as f32;
        if self.y > WIDTH {
            self.y = -200.;
        }
    }
}
