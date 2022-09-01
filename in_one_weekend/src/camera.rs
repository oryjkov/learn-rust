use crate::vec3::*;
use crate::hit::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray { orig: self.origin, dir: self.lower_left_corner + u*self.horizontal + v*self.vertical }
    }
}

pub fn build_camera() -> Camera {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3 (viewport_width, 0.0, 0.0 );
    let vertical = Vec3 (0.0, viewport_height, 0.0 );

    Camera{
        origin,
        horizontal,
        vertical,
        lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vec3(0.0, 0.0, focal_length),
    }
}