#![allow(dead_code)]

use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub const fn new_isize(x: isize, y: isize, z: isize) -> Self {
        Vector3 {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }

    pub const fn new_usize(x: usize, y: usize, z: usize) -> Self {
        Vector3 {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }

    pub fn adjacent_points(&self) -> [Vector3; 6] {
        [
            (-1.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, -1.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, -1.0),
            (0.0, 0.0, 1.0),
        ]
        .map(|(x_d, y_d, z_d)| Vector3::new(self.x + x_d, self.y + y_d, self.z + z_d))
    }

    pub fn manhattan_distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize + self.z.abs() as usize
    }

    pub fn distance_to(&self, other: Vector3) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }

    pub(crate) fn cross(&self, vel_b: Vector3) -> Vector3 {
        let a = self;
        let b = vel_b;

        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub(crate) fn normalize(&self) -> Vector3 {
        let mag = self.magnitude();
        Vector3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub(crate) fn dot(&self, normal: Vector3) -> f64 {
        self.x * normal.x + self.y * normal.y + self.z * normal.z
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x
            + self.y * self.y
            + self.z * self.z)
            .sqrt()
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<&Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: &Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<usize> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let rhs = rhs as f64;

        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<isize> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        let rhs = rhs as f64;

        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
