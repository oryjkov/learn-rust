use std::f64::consts::PI;

use crate::vec3::*;
use crate::hit::*;
use crate::ray::*;
use crate::texture::*;
use crate::material::*;

pub struct Lambertian {
	pub albedo: Box<dyn Texture>,
}

impl Material for Lambertian {
	fn scatter(&self, _: &Ray, hr: &HitRecord) -> Option<Color> {
		Some(self.albedo.value(hr.coord, &hr.p))
	}

	fn scattering_pdf(&self, _r_in: &Ray, hr: &HitRecord, scattered: &Ray) -> f64 {
		let cosine = dot(hr.normal, unit_vector(scattered.dir));
		if cosine > 0.0 {
			cosine / PI
			//0.5 / PI
		} else {
			0.0
		}
	}

}
