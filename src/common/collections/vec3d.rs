use crate::common::error::CommonError;
use std::ops::{Add, AddAssign, Index, IndexMut, Neg, Sub, SubAssign};
use std::str::FromStr;

pub trait Vec3d<T> {
    fn new(x: T, y: T, z: T) -> Self;
    fn norm_l1(&self) -> f64;
    fn norm_l2(&self) -> f64;
    fn distance_l2(&self, other: &Self) -> f64;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct Vec3di {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Add for Vec3di {
    type Output = Vec3di;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for &Vec3di {
    type Output = Vec3di;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3di {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3di {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3di {
    type Output = Vec3di;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for &Vec3di {
    type Output = Vec3di;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3di {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl SubAssign for Vec3di {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3di {
    type Output = Vec3di;

    fn neg(self) -> Self::Output {
        Vec3di {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3d<i64> for Vec3di {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Vec3di { x, y, z }
    }

    fn norm_l1(&self) -> f64 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as f64
    }

    fn norm_l2(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f64).sqrt()
    }

    fn distance_l2(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

impl Index<usize> for Vec3di {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

impl IndexMut<usize> for Vec3di {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds: {}", index),
        }
    }
}

impl FromStr for Vec3di {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x = iter
            .next()
            .ok_or(CommonError::Parse(
                "Malformed input for while parsing Vec3di.",
            ))?
            .parse()?;
        let y = iter
            .next()
            .ok_or(CommonError::Parse(
                "Malformed input for while parsing Vec3di.",
            ))?
            .parse()?;
        let z = iter
            .next()
            .ok_or(CommonError::Parse(
                "Malformed input for while parsing Vec3di.",
            ))?
            .parse()?;
        Ok(Vec3di::new(x, y, z))
    }
}
