use crate::vec3::*;
use crate::metal::*;
use crate::aabb::*;
use crate::ray::*;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
    pub coord: (f64, f64),
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { outward_normal * (-1.0) }
    }
}
pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
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
    fn bounding_box(&self) -> Option<AABB> {
        if self.objects.len() <= 0 { return None }
        let mut bb;
        if let Some(x) = self.objects[0].bounding_box() {
            bb = x;
        } else {
            return None;
        }
        for obj in &self.objects[1..] {
            if let Some(b2) = obj.bounding_box() {
                bb = bb.surrounding_box(&b2);
            } else {
                return None
            }
        }
        Some(bb)
    }
}
