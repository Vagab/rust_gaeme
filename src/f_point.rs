extern crate mint;
extern crate rand;

use mint::Point2;
use std::ops::{Deref, Add, AddAssign, DerefMut, Sub};
use rand::Rng;
use std::f32::consts::PI;
use std::iter::Sum;
use crate::{WIDTH, HEIGHT};

pub const CENTER: FPoint = FPoint(Point2 { x: WIDTH / 2., y: HEIGHT / 2. });

#[derive(Copy, Clone, Deref, DerefMut, Into, Debug)]
pub struct FPoint(pub Point2<f32>);

impl Add for FPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(Point2 { x: self.x + rhs.x, y: self.y + rhs.y })
    }
}

impl AddAssign for FPoint {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for FPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(Point2 { x: self.x - rhs.x, y: self.y - rhs.y })
    }
}

impl FPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Point2 { x, y })
    }

    pub fn zero() -> Self {
        Self::new(0., 0.)
    }

    // generate a random point on the unit circle
    pub fn unit_rand() -> Self {
        let direction = rand::thread_rng().gen::<f32>() * 2. * PI;
        Self::new(direction.cos(), direction.sin())
    }

    pub fn length(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).powf(0.5)
    }

    pub fn with_length(self, length: f32) -> Self {
        self.times(length / self.length())
    }

    pub fn times(self, fac: f32) -> Self {
        Self(Point2 { x: self.0.x * fac, y: self.0.y * fac })
    }

    pub fn distance(self, rhs: Self) -> f32 {
        ((self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2)).powf(0.5)
    }

    pub fn max(&mut self, length: f32) {
        let current = self.length();
        if current > length {
            *self = self.times(length / current)
        }
    }

    pub fn unit(self) -> Self {
        let length = self.length();
        Self(Point2 { x: self.x / length, y: self.y / length })
    }
}

impl Sum for FPoint {
    fn sum<I: Iterator<Item=FPoint>>(iter: I) -> Self {
        let mut sum = FPoint::zero();
        for point in iter {
            sum += point
        }
        sum
    }
}

impl PartialEq for FPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
