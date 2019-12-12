extern crate rand;
use rand::{thread_rng, prelude::ThreadRng, Rng, distributions::{Normal, Distribution}};


pub struct RDrop {
    pub x: f64,
    pub y: f64,
    pub z: u8,
}

impl RDrop {

    fn gen_pos() {
        self.x = rand.thread_rng().gen_range(-100.)
    }
}
