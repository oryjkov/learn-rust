use std::f64::consts::PI;

use crate::vec3::*;
use crate::hit::*;
use crate::ray::*;
use crate::texture::*;
use crate::material::*;
use crate::pdf::*;

pub struct Lambertian {
	pub albedo: Box<dyn Texture>,
}
 
// Scattering PDF evaluated for scattering into `scattered` at `hr`.
fn scattering_pdf(hr: &HitRecord, scattered_dir: &Vec3) -> f64 {
	let cosine = dot(hr.normal, unit_vector(*scattered_dir));
	if cosine > 0.0 {
		cosine / PI
	} else {
		0.0
	}
}


impl Material for Lambertian {
	fn scatter(&self, _r_in: &Ray, hr: &HitRecord, lights: Option<&dyn Hittable>)
	    -> Option<(Vec3, Color)> {
		let cos_pdf = CosinePDF{normal: &hr.normal};
		let (scattered_dir, pdf_val) = if let Some(l) = lights {
		    gen_eval(&hr.p, &cos_pdf, 0.5, &vec!(l))
		} else {
		    gen_eval(&hr.p, &cos_pdf, 1.0, &vec!())
		};

		let color_contribution = self.albedo.value(hr.coord, &hr.p) *
			scattering_pdf(&hr, &scattered_dir) *
			(1.0/pdf_val);
		Some((scattered_dir, color_contribution))
	}
}
