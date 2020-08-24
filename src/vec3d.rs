use num::Float;
use std::ops::Add;


#[derive(Clone, Copy, Debug)]
struct Vec3d<F: Float> {
    x: F,
    y: F,
    z: F,
}

impl<F: Float + std::ops::MulAssign> Vec3d<F> {
    fn make_unit_vector(&mut self) {
        let ratio = (self.x.powi(2) + self.y.powi(2) + self.y.powi(2))
            .sqrt()
            .recip();
        self.x *= ratio;
        self.y *= ratio;
        self.z *= ratio;
    }
    fn dot(self, other: Vec3d<F>) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn cross(self, other: Vec3d<F>) -> Vec3d<F> {
        let a = self;
        let b = other;
        Vec3d {
            x: a.y * b.z - a.z * b.y,
            y: -(a.x * b.z - a.z * b.x),
            z: a.x * b.y - a.y * b.x,
        }
    }
    fn unit_vector(self) -> Vec3d<F>{
        self / self.length()
    }
    fn length_squared(self) -> F{
        (self.x.powi(2) + self.y.powi(2) + self.y.powi(2))
    }
    fn length(self) -> F {
        self.length_squared().sqrt()
    }
}

impl<F: Float> Add for Vec3d<F> {
    type Output = Vec3d<F>;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<F: Float> std::ops::Sub for Vec3d<F> {
    type Output = Vec3d<F>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<F: Float> std::ops::Mul for Vec3d<F> {
    type Output = Vec3d<F>;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl<F: Float> std::ops::Div for Vec3d<F> {
    type Output = Vec3d<F>;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3d {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl<F: Float> std::ops::Mul<F> for Vec3d<F> {
    type Output = Vec3d<F>;
    fn mul(self, f: F) -> Self::Output {
        Vec3d {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}
impl<F: Float> std::ops::Div<F> for Vec3d<F> {
    type Output = Vec3d<F>;
    fn div(self, f: F) -> Self::Output {
        Vec3d {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}
impl<F: Float + std::ops::AddAssign> std::ops::AddAssign for Vec3d<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl<F: Float + std::ops::SubAssign> std::ops::SubAssign for Vec3d<F> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl<F: Float + std::ops::MulAssign> std::ops::MulAssign for Vec3d<F> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
impl<F: Float + std::ops::DivAssign> std::ops::DivAssign for Vec3d<F> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
impl<F: Float + std::ops::MulAssign> std::ops::MulAssign<F> for Vec3d<F> {
    fn mul_assign(&mut self, f: F) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
    }
}
impl<F: Float + std::ops::DivAssign> std::ops::DivAssign<F> for Vec3d<F> {
    fn div_assign(&mut self, f: F) {
        self.x /= f;
        self.y /= f;
        self.z /= f;
    }
}
