use std::cmp::Ordering;

use rand::random;

use crate::aabb::*;
use crate::vec3::*;
use crate::hit::*;
use crate::ray::*;

// Bounded volume hierarchy.
pub struct BVHNode {
	bbox: AABB,
	child0: Box<dyn Hittable>,
	child1: Option<Box<dyn Hittable>>,
}

impl Hittable for BVHNode {
	fn bounding_box(&self) -> Option<AABB> {
		Some(self.bbox.clone())
	}
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		if !self.bbox.hit(r, t_min, t_max) {
			return None;
		}

		let mut first_hit_at = t_max;
		let mut hr: Option<HitRecord> = None;
		// Try the left child. Then try the right child, but only take right if it was hit before the left child.
		if let Some(h_left) = self.child0.hit(r, t_min, t_max) {
			first_hit_at = h_left.t;
			hr = Some(h_left);
		}
		if let Some(right_child) = &self.child1 {
			if let Some(h_right) = right_child.hit(r, t_min, first_hit_at) {
				hr = Some(h_right);
			}
		}
		hr
	}
}

impl BVHNode {
	pub fn new(mut objs: Vec<Box<dyn Hittable>>) -> BVHNode {
		let axis = random::<usize>() % 3;
		let comparator = |h1: &Box<dyn Hittable>, h2: &Box<dyn Hittable>| {
			if let (Some(b1), Some(b2)) = (h1.bounding_box(), h2.bounding_box()) {
				AABB::compare_axis(&b1, &b2, axis)
			} else {
				Ordering::Greater
			}
		};

		let object_span = objs.len();
		let (child0, child1) = match object_span {
			1 => {
				(objs.pop().unwrap(), None)
			}	
			2 => {
				let a = objs.pop().unwrap();
				let b = objs.pop().unwrap();

				if comparator(&a, &b) == Ordering::Less {
					(a, Some(b))
				} else {
					(b, Some(a))
				}
			}
			_ => {
				objs.sort_unstable_by(comparator);
				let mid = object_span/2;
				let sr = objs.split_off(mid);
				let x = Box::new(BVHNode::new(objs)) as Box<dyn Hittable>;
				let y = Box::new(BVHNode::new(sr)) as Box<dyn Hittable>;
				(x, Some(y))
			}
		};
		let rb = if child1.is_none() {child0.bounding_box()} else {child1.as_ref().unwrap().bounding_box()};
		if let (Some(b1), Some(b2)) = (child0.bounding_box(), rb) {
			BVHNode{ bbox: b1.surrounding_box(&b2), child0, child1}
		} else {
			BVHNode{ bbox: AABB::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0)), child0, child1 }
		}
	}
}