extern crate rand;

use rand::{thread_rng, prelude::ThreadRng, Rng, distributions::{Normal, Distribution}};


#[derive(Copy, Clone)]
pub struct UpPillar {
    x: f32,
    y: f32,
}

impl UpPillar {

    fn new() -> Self {
        Self {
            x: -100.,
            y: 0.,
        }
    }
}
