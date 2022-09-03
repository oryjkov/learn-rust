use crate::vec3::*;
use crate::hit::*;

pub trait Material {
	fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
	pub albedo: Color,
}

impl Material for Lambertian {
	fn scatter(&self, _: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
		let mut scatter_direction = hr.normal + random_unit_vector();
		if scatter_direction.near_zero() {
			scatter_direction = hr.normal
		}
		Some((self.albedo, Ray{orig: hr.p, dir: scatter_direction}))
	}
}

pub struct Metal {
	pub albedo: Color,
}

impl Material for Metal {
	fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
		let reflected = reflect(unit_vector(r_in.dir), hr.normal);
		let scattered = Ray{orig: hr.p, dir: reflected};
		if dot(scattered.dir, hr.normal) > 0.0 {
			Some((self.albedo, scattered))
		} else {
			None
		}
	}
}