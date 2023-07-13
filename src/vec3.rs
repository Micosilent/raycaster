use std::ops::{Add, Div, Mul, Sub};
#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn as_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

//Overloads
impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vector3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
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
impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
pub enum DivError {
    DivByZeroError,
}

impl Div<f64> for Vector3 {
    type Output = Result<Vector3, DivError>;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            return Err(DivError::DivByZeroError);
        }

        Ok(Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        })
    }
}
