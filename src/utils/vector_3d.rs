#![allow(dead_code)]

use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Vector3 {
    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Vector3 { x, y, z }
    }

    pub const fn new_usize(x: usize, y: usize, z: usize) -> Self {
        Vector3 {
            x: x as isize,
            y: y as isize,
            z: z as isize,
        }
    }

    pub fn adjacent_points(&self) -> [Vector3; 6] {
        [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .map(|(x_d, y_d, z_d)| Vector3::new(self.x + x_d, self.y + y_d, self.z + z_d))
    }

    pub fn manhattan_distance(&self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs() + self.z.unsigned_abs()
    }

    pub fn distance_to(&self, other: Vector3) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
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
        let rhs = rhs as isize;

        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
