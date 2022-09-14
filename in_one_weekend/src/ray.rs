use std::f64::INFINITY;

use crate::vec3::*;
use crate::hit::*;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}
impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

pub fn ray_color(r: &Ray, background: &Color, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    if let Some(hr) = world.hit(r, 0.001, INFINITY) {
        let emitted = hr.material.emitted(hr.coord, &hr.p);

        if let Some((attenuation, scattered)) = hr.material.scatter(r, &hr) {
            emitted + attenuation * ray_color(&scattered, background, world, depth-1)
        } else {
            emitted
        }
    } else {
        background.clone()
    }
}