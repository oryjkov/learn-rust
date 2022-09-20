use crate::vec3::*;
use crate::hit::*;
use crate::ray::*;

pub trait Material: Sync {
	// Scatters the light. Returns the color contribution, scattered ray and scattered ray PDF.
	// PDF is the probability of the scattered ray being sampled.
	// Returns None when the ray was absorbed.
	fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray, f64)>;
	// Returns color of the absorbed light.
	fn emitted(&self, _coord: Vec2, _p: &Point3) -> Color {
		Vec3(0.0, 0.0, 0.0)
	}

	// Scattering PDF evaluated for `r_in` scattering into `scattered` at `hr`.
	fn scattering_pdf(&self, _r_in: &Ray, _hr: &HitRecord, _scattered: &Ray) -> f64 {
		0.0
	}
}