use rand::random;
use crate::material::*;
use crate::vec3::*;
use crate::ray::*;
use crate::hit::*;

pub struct Dielectric {
	pub ir: f64,  // Index of Refraction
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
	let mut cos_theta = dot(-1.0 * uv, n);
	if cos_theta > 1.0 {
		cos_theta = 1.0;
	}
	let r_out_perp = etai_over_etat * (uv + cos_theta*n);
	let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
	r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
	// Schlick's approximation for reflectance
	let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
	r0 = r0*r0;
	r0 + (1.0-r0) * (1.0-cosine).powi(5)
}

impl Material for Dielectric {
	fn scatter(&self, r_in: &Ray, hr: &HitRecord, _lights: &HittableList)
	    -> Option<(Vec3, Color)> {
		let attenuation = Vec3(1.0, 1.0, 1.0);
		let refraction_ratio = if hr.front_face {1.0/self.ir} else {self.ir};
		let unit_direction = unit_vector(r_in.dir);

		let mut cos_theta = dot(-1.0 * unit_direction, hr.normal);
		cos_theta = if cos_theta > 1.0 {1.0} else {cos_theta};
		let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

		let cannot_refract = refraction_ratio * sin_theta > 1.0;
		let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random::<f64>() {
			reflect(unit_direction, hr.normal)
		} else {
			refract(unit_direction, hr.normal, refraction_ratio)
		};
		Some((direction, attenuation))
	}
}