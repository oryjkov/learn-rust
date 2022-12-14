use crate::vec3::*;
use crate::material::*;
use crate::aabb::*;
use crate::ray::*;
use rand::random;

pub struct HitRecord<'a> {
    // Point where the hit happened.
    pub p: Point3,
    // Normal vector to the surface at point `p`.
    pub normal: Vec3,
    pub material: &'a Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
    pub coord: Vec2,
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
	fn gen_random_point(&self, origin: &Vec3) -> Vec3 {
        let _ = origin;
        Vec3(0.0, 0.0, 0.0)
    }
    fn pdf_eval(&self, origin: &Vec3, dir: &Vec3) -> f64 {
        let _ = origin;
        let _ = dir;
        0.0
    }
    // Used to collect all lights that are behind this Hittable.
    fn pick_lights(&self) -> Vec<&dyn Hittable> { vec![] }
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
    fn pdf_eval(&self, origin: &Vec3, dir: &Vec3) -> f64 {
        let w = 1.0/self.objects.len() as f64;
        self.objects.iter().map(|x| {w*x.pdf_eval(origin, dir)} ).sum()
    }
    fn gen_random_point(&self, origin: &Vec3) -> Vec3 {
        let n = random::<usize>() % self.objects.len();
        self.objects[n].gen_random_point(origin)
    }
}
