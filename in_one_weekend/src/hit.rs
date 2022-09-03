use std::f64::INFINITY;

use crate::vec3::*;
use crate::metal::*;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { outward_normal * (-1.0) }
    }
}
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl Hittable for Sphere<'_> {
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
        let mut hr = HitRecord { p: r.at(root), normal: (r.at(root) - self.center) / self.radius, t: root, front_face: false, material: self.material};
        let outward_normal = (hr.p - self.center) / self.radius;
        hr.set_face_normal(r, outward_normal);
        return Some(hr);
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord>  = None;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            if let Some(hr) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = hr.t;
                temp_rec = Some(hr);
            }
        }
        temp_rec
    }
}

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}
impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

pub fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    if let Some(hr) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = hr.material.scatter(r, &hr) {
            attenuation * ray_color(&scattered, world, depth-1)
        } else {
            Vec3(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = unit_vector(r.dir);
        let t = 0.5*(unit_direction.1+1.0);
        (1.0-t)*Vec3(1.0, 1.0, 1.0) + t*Vec3(0.5, 0.7, 1.0)
    }
}