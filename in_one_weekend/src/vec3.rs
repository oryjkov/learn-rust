use std::ops;
use rand::random;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2(pub f64, pub f64);

impl Vec3 {
    pub fn length(self) -> f64 {
        return (self.0 * self.0 + self.1*self.1 + self.2*self.2).sqrt();
    }

    pub fn length_squared(self) -> f64 {
        return self.0 * self.0 + self.1*self.1 + self.2*self.2;
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        (self.0.abs()<s) && (self.1.abs() < s) && (self.2.abs() < s)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0+rhs.0, self.1 + rhs.1, self.2 + rhs.2,)
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3(self.0+rhs, self.1 + rhs, self.2 + rhs,)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0-rhs.0, self.1 - rhs.1, self.2 - rhs.2,)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 * self.0, rhs.1 * self.1, rhs.2 * self.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(rhs * self.0, rhs * self.1, rhs * self.2)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0/rhs, self.1/rhs, self.2/rhs)
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v/v.length()
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.0*v.0 + u.1*v.1 + u.2*v.2
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(
        u.1*v.2 - u.2*v.1,
        u.2*v.0 - u.0*v.2,
        u.0*v.1 - u.1*v.0)
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3(
            random::<f64>() * 2.0 - 1.0,
            random::<f64>() * 2.0 - 1.0,
            random::<f64>() * 2.0 - 1.0,
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
	}
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3(random::<f64>()*2.0-1.0, random::<f64>()*2.0 - 1.0, 0.0);
        if p.length_squared()>1.0 {
            continue
        }
        return p;
    }
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let v = random_in_unit_sphere();
    if dot(v, normal) >= 0.0 {
        v
    } else {
        -1.0 * v
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn random_vec3() -> Vec3 {
    Vec3(random::<f64>(),random::<f64>(),random::<f64>(),)
}

pub fn random_vec3_bounds(min: f64, max: f64) -> Vec3 {
    let d = max - min;
    Vec3(random::<f64>()*d + min,random::<f64>()*d+min,random::<f64>()*d+min,)
}
