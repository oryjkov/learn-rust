use crate::vec3::*;
use crate::hit::*;
use crate::ray::*;
use crate::texture::*;
use crate::material::*;
use rand::random;

pub struct DiffuseLight {
	pub emit: Box<dyn Texture>,
}

impl Material for DiffuseLight {
	fn scatter(&self, _r_in: &Ray, _hr: &HitRecord) -> Option<Color> {
		None
	}
	fn emitted(&self, coord: Vec2, p: &Point3) -> Color {
		self.emit.value(coord, p)
	}
}

pub struct Metal {
	pub albedo: Color,
	pub fuzz: f64,
}

impl Material for Metal {
	fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<Color> {
		let reflected = reflect(unit_vector(r_in.dir), hr.normal);
		let scattered_dir = reflected + self.fuzz*random_in_unit_sphere();
		if dot(scattered_dir, hr.normal) > 0.0 {
			Some(self.albedo)
		} else {
			None
		}
	}
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

pub struct Dielectric {
	pub ir: f64,  // Index of Refraction
}

impl Material for Dielectric {
	fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<Color> {
		let attenuation = Vec3(1.0, 1.0, 1.0);
		/*
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
		*/

		Some(attenuation)
	}
}