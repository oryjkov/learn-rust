use std::cmp::Ordering;

use crate::vec3::*;
use crate::ray::*;

// Axis-aligned bounding box.
#[derive(Clone, PartialEq, Debug)]
pub struct AABB {
	p1: Point3,
	p2: Point3,
}

impl AABB {
	pub fn new(p1: Point3, p2: Point3) -> AABB {
		AABB{p1, p2}
	}
	pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
		let (tx1_, tx2_) = (
			(self.p1.0 - r.orig.0) / r.dir.0, 
			(self.p2.0 - r.orig.0) / r.dir.0,
		);
		let ix = (tx1_.min(tx2_).max(t_min), tx1_.max(tx2_).min(t_max));

		let (ty1_, ty2_) = (
			(self.p1.1 - r.orig.1) / r.dir.1, 
			(self.p2.1 - r.orig.1) / r.dir.1,
		);
		let iy = (ty1_.min(ty2_).max(t_min), ty1_.max(ty2_).min(t_max));

		let (tz1_, tz2_) = (
			(self.p1.2 - r.orig.2) / r.dir.2, 
			(self.p2.2 - r.orig.2) / r.dir.2,
		);
		let iz = (tz1_.min(tz2_).max(t_min), tz1_.max(tz2_).min(t_max));

		overlap(ix, iy) && overlap(ix, iz) && overlap(iy, iz)
	}
	pub fn surrounding_box(&self, other: &AABB) -> AABB {
		AABB { 
			p1: Vec3(
				self.p1.0.min(other.p1.0),
				self.p1.1.min(other.p1.1),
				self.p1.2.min(other.p1.2),
			),
			p2: Vec3(
				self.p2.0.max(other.p2.0),
				self.p2.1.max(other.p2.1),
				self.p2.2.max(other.p2.2),
			),}
	}
	pub fn compare_axis(b1: &AABB, b2: &AABB, axis: usize) -> Ordering {
		if match axis {
			0 => b1.p1.0 < b2.p1.0,
			1 => b1.p1.1 < b2.p1.1,
			_ => b1.p1.2 < b2.p1.2,
		} {
			Ordering::Less
		} else {
			Ordering::Greater
		}
	}
}

#[test]
fn surrounding_box_test() {
	let b1 = AABB::new(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 1.0, 1.0));
	let b2 = AABB::new(Vec3(2.0, 2.0, 2.0), Vec3(3.0, 3.0, 3.0));

	assert_eq!(b1.surrounding_box(&b2), AABB{p1: Vec3(0.0, 0.0, 0.0), p2: Vec3(3.0, 3.0, 3.0)});
	assert_eq!(b1.surrounding_box(&b2), b2.surrounding_box(&b1));
}

fn overlap(i1: (f64, f64), i2: (f64, f64)) -> bool {
	!(i1.1 < i2.0 || i2.1 < i1.0)
}
#[test]
fn overlap_test() {
	assert!(!overlap((0.0, 1.0), (2.0, 3.0)));
	assert!(overlap((0.0, 4.0), (2.0, 3.0)));
	assert!(!overlap((4.0, 5.0), (2.0, 3.0)));
	assert!(overlap((0.0, 5.0), (2.0, 3.0)));
	assert!(overlap((2.5, 5.0), (2.0, 3.0)));
}

#[cfg(test)]
#[test]
fn intersect_test() {
    use std::f64::{NEG_INFINITY, INFINITY};

	let bb = AABB{p1: Vec3(0.0, 0.0, 0.0), p2: Vec3(1.0, 1.0, 1.0)};

	assert!(bb.hit(&Ray{orig: Vec3(0.0, 0.0, 0.0), dir: Vec3(1.0, 1.0, 1.0)}, NEG_INFINITY, INFINITY));
	assert!(!bb.hit(&Ray{orig: Vec3(2.0, 0.0, 0.0), dir: Vec3(1.0, 1.0, 1.0)}, NEG_INFINITY, INFINITY));
	assert!(bb.hit(&Ray{orig: Vec3(2.0, 2.0, 2.0), dir: Vec3(1.0, 1.0, 1.0)}, NEG_INFINITY, INFINITY));

	assert!(!bb.hit(&Ray{orig: Vec3(0.0, 0.0, 0.0), dir: Vec3(1.0, 0.0, 0.0)}, NEG_INFINITY, INFINITY));
	assert!(!bb.hit(&Ray{orig: Vec3(0.0, 1.0, 0.0), dir: Vec3(1.0, 0.0, 0.0)}, NEG_INFINITY, INFINITY));

	assert!(!bb.hit(&Ray{orig: Vec3(0.0, 0.0, 0.0), dir: Vec3(1.0, 1.0, 1.0)}, 10.0, INFINITY));
	assert!(!bb.hit(&Ray{orig: Vec3(0.0, 0.0, 0.0), dir: Vec3(1.0, 1.0, 1.0)}, NEG_INFINITY, -0.1));
}