use mint::Point2;
use std::ops::{Deref, Range};
use rand::Rng;

pub trait VectorPoint {
    fn zero() -> Self;
    fn random<R: Rng>(rng: &mut R, range_x: Range<f32>, range_y: Range<f32>) -> Self;
    fn dist(self, other: Self) -> f32;
    fn to(self, other: Self) -> Self;
    fn len(self) -> f32;
    fn scaled(self, scale: f32) -> Self;
    fn of_len(self, len: f32) -> Self;
}

impl VectorPoint for Point2<f32> {
    fn zero() -> Self {
        Self { x: 0., y: 0. }
    }

    fn random<R: Rng>(rng: &mut R, range_x: Range<f32>, range_y: Range<f32>) -> Self {
        Self {
            x: rng.gen_range(range_x.start, range_x.end),
            y: rng.gen_range(range_y.start, range_y.end),
        }
    }

    fn dist(self, other: Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).powf(0.5)
    }

    fn to(self, other: Self) -> Self {
        Self {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    fn len(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).powf(0.5)
    }

    fn scaled(self, scale: f32) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    fn of_len(self, len: f32) -> Self {
        self.scaled(len / self.len())
    }
}