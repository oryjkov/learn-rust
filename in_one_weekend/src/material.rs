use crate::vec3::*;
use crate::hit::*;
use crate::ray::*;

pub trait Material: Sync {
	// Scatters the light. Returns the scattering direction and the color
	// contribution of this scattering.
	// Returns None when the ray was absorbed.
	fn scatter(&self, r_in: &Ray, hr: &HitRecord, lights: Option<&dyn Hittable>)
	    -> Option<(Vec3, Color)> {
		let _ = r_in;
		let _ = hr;
		let _ = lights;
		None
	}

	// Returns color of the absorbed light. Only makes sense for lights.
	fn emitted(&self, coord: Vec2, p: &Point3) -> Color {
		let _ = coord;
		let _ = p;
		Vec3(0.0, 0.0, 0.0)
	}

	fn is_light(&self) -> bool { return false; }
}