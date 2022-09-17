use crate::vec3::*;
use crate::hit::*;
use crate::ray::*;

pub trait Material: Sync {
	// Scatters the light. Returns None when the ray was absorbed.
	fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)>;
	// Returns color of the absorbed light.
	fn emitted(&self, _coord: Vec2, _p: &Point3) -> Color {
		Vec3(0.0, 0.0, 0.0)
	}
}