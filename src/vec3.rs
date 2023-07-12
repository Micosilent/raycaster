use std::ops::{Add, Div, Sub};

pub struct Vector3 {
    x: usize,
    y: usize,
    z: usize,
}

impl Vector3 {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
    pub fn as_array(&self) -> [usize; 3] {
        [self.x, self.y, self.z]
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, right_hand_side: Vector3) -> Vector3 {
        Self {
            x: self.x + right_hand_side.x,
            y: self.y + right_hand_side.y,
            z: self.z + right_hand_side.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
pub enum DivError {
    DivByZeroError,
}

impl Div<usize> for Vector3 {
    type Output = Result<Vector3, DivError>;

    fn div(self, rhs: usize) -> Self::Output {
        if rhs == 0 {
            return Err(DivError::DivByZeroError);
        }

        Ok(Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        })
    }
}
