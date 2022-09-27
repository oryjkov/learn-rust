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

pub fn ray_color(r: &Ray, background: &Color, world: &dyn Hittable, lights: Option<&dyn Hittable>,
    depth: i32) -> Color {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    if let Some(hr) = world.hit(r, 0.001, INFINITY) {
        let emitted = hr.material.emitted(hr.coord, &hr.p);

        if let Some((scatter_dir, color_contribution)) = hr.material.scatter(r, &hr, lights) {
            let scattered_ray = Ray {orig: hr.p, dir: scatter_dir};
            emitted + color_contribution * ray_color(&scattered_ray, background, world, lights, depth-1)
        } else {
            emitted
        }
    } else {
        background.clone()
    }
}