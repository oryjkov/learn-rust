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
	fn scatter(&self, _r_in: &Ray, hr: &HitRecord, lights: &HittableList)
	    -> Option<(Vec3, Color)> {
		let cos_pdf = CosinePDF{normal: &hr.normal};
		let hit_pdf = HittablePDF{hittable: lights.objects[0].as_ref(), origin: &hr.p};
		let mix_pdf = MixturePDF{pdf1: &cos_pdf, pdf2: &hit_pdf};

		let (scattered_dir, opt_val) = mix_pdf.gen();
		let pdf_val = if opt_val.is_some() {
			opt_val.unwrap()
		} else {
			mix_pdf.eval(&scattered_dir)
		};

		let color_contribution = self.albedo.value(hr.coord, &hr.p) *
			scattering_pdf(&hr, &scattered_dir) *
			(1.0/pdf_val);
		Some((scattered_dir, color_contribution))
	}
}
