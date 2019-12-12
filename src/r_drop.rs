extern crate rand;
use rand::{thread_rng, prelude::ThreadRng, Rng, distributions::{Normal, Distribution}};
use crate::{WIDTH, HEIGHT, STEP};

pub struct RDrop {
    pub x: f32,
    pub y: f32,
    pub z: u8,
}

impl RDrop {

    fn new(r: &mut ThreadRng) -> Self {
        Self {
            x: r.gen_range(0., WIDTH),
            y: r.gen_range(0., HEIGHT),
            z: r.gen(),
        }
    }

    fn fall(&mut self) {
        self.y += STEP / self.z as f32;
    }
}
