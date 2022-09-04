use std::f64::consts::PI;

use crate::vec3::*;
use crate::hit::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray { orig: self.origin, dir: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin }
    }
}

fn degrees_to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn build_camera(
    look_from: Point3,
    look_at: Point3,
    v_up: Vec3,
    vfov_deg: f64,
    aspect_ratio: f64) -> Camera {
    let theta = degrees_to_radians(vfov_deg);
    let h = (theta/2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;
    
    let w = unit_vector(look_from - look_at);
    let u = unit_vector(cross(v_up, w));
    let v = cross(w, u);

    let origin = look_from;
    let horizontal = viewport_width*u;
    let vertical = viewport_height*v;
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

    Camera{
        origin,
        horizontal,
        vertical,
        lower_left_corner
    }
}