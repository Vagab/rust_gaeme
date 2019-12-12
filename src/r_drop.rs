extern crate rand;

use rand::{thread_rng, prelude::ThreadRng, Rng, distributions::{Normal, Distribution}};
use crate::{WIDTH, HEIGHT, STEP};
use std::cmp::min;
use float_ord::FloatOrd;

const RD_WIDTH: f32 = 0.2;
const RD_HEIGHT: f32 = 5.;

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
            z: r.gen_range(1, 6), // shouldn't be 0
        }
    }

    pub fn get_wh(&self) -> (f32, f32) {
        let coeff = 10. * 2f32.powf(-self.z as f32).powf(0.5);
        (RD_WIDTH * coeff, RD_HEIGHT * coeff)
    }

    pub fn fall(&mut self) {
        self.y += STEP / self.z as f32;
        if self.y > HEIGHT {
            self.y = -self.get_wh().1;
        }
    }
}
