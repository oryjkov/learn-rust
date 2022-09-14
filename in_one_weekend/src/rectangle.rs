use crate::aabb::*;
use crate::metal::*;
use crate::ray::*;
use crate::vec3::*;
use crate::hit::*;

static EPS: f64 = 1e-4;

pub struct XYRect {
	pub material: Box<dyn Material>,
	// Two corners.
	pub p1: Vec2,
	pub p2: Vec2,
	// Z-coordinate.
	pub k: f64,
}

impl Hittable for XYRect {
	fn bounding_box(&self) -> Option<AABB> {
		Some(AABB::new(Vec3(self.p1.0, self.p1.1, self.k-EPS), Vec3(self.p2.0, self.p2.1, self.k+EPS)))
	}
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let t = (self.k - r.orig.2)/r.dir.2;
		if t<t_min || t>t_max {
			return None;
		}
		let v = r.orig + t * r.dir;
		if v.0 < self.p1.0 || v.0 > self.p2.0 || v.1 < self.p1.1 || v.1 > self.p2.1 {
			return None
		}
		let mut hr = HitRecord {
			p: v,
			normal: Vec3(0.0, 0.0, 1.0),
			t,
			material: &self.material,
			front_face: false,
			coord: Vec2(
				(v.0-self.p1.0)/(self.p2.0-self.p1.0),
				(v.1-self.p1.1)/(self.p2.1-self.p1.1),
			)
		};
		hr.set_face_normal(r, hr.normal);
		Some(hr)
	}
}