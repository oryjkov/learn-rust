use crate::vec3::*;
use crate::hit::*;
use crate::ray::*;

pub trait Material: Sync {
	// Scatters the light. Returns the color contribution.
	// PDF is the probability of the scattered ray being sampled.
	// Returns None when the ray was absorbed.
	fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<Color>;

	// Returns color of the absorbed light. Only makes sense for lights.
	fn emitted(&self, coord: Vec2, p: &Point3) -> Color {
		let _ = coord;
		let _ = p;
		Vec3(0.0, 0.0, 0.0)
	}

	// Scattering PDF evaluated for `r_in` scattering into `scattered` at `hr`.
	fn scattering_pdf(&self, r_in: &Ray, hr: &HitRecord, scattered: &Ray) -> f64 {
		let _ = r_in;
		let _ = hr;
		let _ = scattered;
		0.0
	}
}