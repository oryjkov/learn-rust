use crate::vec3::*;
use crate::hit::*;
use crate::ray::*;
use crate::texture::*;
use crate::material::*;

pub struct Lambertian {
	pub albedo: Box<dyn Texture>,
}

impl Material for Lambertian {
	fn scatter(&self, _: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
		let mut scatter_direction = hr.normal + random_unit_vector();
		if scatter_direction.near_zero() {
			scatter_direction = hr.normal
		}
		Some((self.albedo.value(hr.coord, &hr.p), Ray{orig: hr.p, dir: scatter_direction}))
	}
}
