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

pub fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    if let Some(hr) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = hr.material.scatter(r, &hr) {
            attenuation * ray_color(&scattered, world, depth-1)
        } else {
            Vec3(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = unit_vector(r.dir);
        let t = 0.5*(unit_direction.1+1.0);
        (1.0-t)*Vec3(1.0, 1.0, 1.0) + t*Vec3(0.5, 0.7, 1.0)
    }
}