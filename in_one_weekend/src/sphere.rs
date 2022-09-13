use std::f64::consts::PI;

use crate::vec3::*;
use crate::metal::*;
use crate::hit::*;
use crate::ray::*;
use crate::aabb::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = dot(oc, r.dir);
        let c = dot(oc, oc) - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;

        if discriminant<0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
		let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let mut hr = HitRecord {
 			p, normal: (r.at(root) - self.center) / self.radius, t: root, front_face: false, material: &self.material,
			coord: get_shpere_coord(outward_normal),
		};
        hr.set_face_normal(r, outward_normal);
        return Some(hr);
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(self.center + (-self.radius), self.center+self.radius))
    }
}

fn get_shpere_coord(p: Point3) -> (f64, f64) {
	// given a point p on the sphere of radius 1.0, returns the (u,v) coordinates between [0,1].
	let theta = (-p.1).acos();
	let phi = (-p.2).atan2(p.0)+PI;

	(phi/(2.0*PI), theta/PI)
}

impl Sphere {
	pub fn box_new<T: Material+'static>(center: Point3, radius: f64, material: T) -> Box<Sphere> {
    	Box::new(Sphere{center, radius, material: Box::new(material)})
	}
}