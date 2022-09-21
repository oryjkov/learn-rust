use crate::vec3::*;
use crate::hit::*;
use crate::ray::*;
use crate::texture::*;
use crate::material::*;

pub struct DiffuseLight {
	pub emit: Box<dyn Texture>,
}

impl Material for DiffuseLight {
	fn emitted(&self, coord: Vec2, p: &Point3) -> Color {
		self.emit.value(coord, p)
	}
}

pub struct Metal {
	pub albedo: Color,
	pub fuzz: f64,
}

impl Material for Metal {
	fn scatter(&self, r_in: &Ray, hr: &HitRecord, _lights: &HittableList)
	    -> Option<(Vec3, Color)> {
		let reflected = reflect(unit_vector(r_in.dir), hr.normal);
		let scattered_dir = reflected + self.fuzz*random_in_unit_sphere();
		if dot(scattered_dir, hr.normal) > 0.0 {
			Some((scattered_dir, self.albedo))
		} else {
			None
		}
	}
}

