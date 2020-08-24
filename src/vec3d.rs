
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3d { x, y, z }
    }
    pub fn x(self) -> f64 {
        self.x
    }
    pub fn y(self) -> f64 {
        self.y
    }
    pub fn z(self) -> f64 {
        self.z
    }
    pub fn make_unit_vector(&mut self) {
        let ratio = (self.x.powi(2) + self.y.powi(2) + self.y.powi(2))
            .sqrt()
            .recip();
        self.x *= ratio;
        self.y *= ratio;
        self.z *= ratio;
    }
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn cross(self, other: Self) -> Self {
        let a = self;
        let b = other;
        Vec3d {
            x: a.y * b.z - a.z * b.y,
            y: -(a.x * b.z - a.z * b.x),
            z: a.x * b.y - a.y * b.x,
        }
    }
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
    fn length_squared(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.y.powi(2))
    }
    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Add for Vec3d {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3d {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl std::ops::Mul for Vec3d {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl std::ops::Div for Vec3d {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3d {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl std::ops::Mul<f64> for Vec3d {
    type Output = Self;
    fn mul(self, f: f64) -> Self::Output {
        Vec3d {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}
impl std::ops::Div<f64> for Vec3d {
    type Output = Self;
    fn div(self, f: f64) -> Self::Output {
        Vec3d {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}
impl std::ops::AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl std::ops::SubAssign for Vec3d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl std::ops::MulAssign for Vec3d {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl std::ops::DivAssign for Vec3d {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
impl std::ops::MulAssign<f64> for Vec3d {
    fn mul_assign(&mut self, f: f64) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
    }
}
impl std::ops::DivAssign<f64> for Vec3d {
    fn div_assign(&mut self, f: f64) {
        self.x /= f;
        self.y /= f;
        self.z /= f;
    }
}
