use std::f64::INFINITY;

use crate::pdf::*;
use crate::vec3::*;
use crate::hit::*;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}
impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

pub fn ray_color(r: &Ray, background: &Color, world: &dyn Hittable, lights: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    if let Some(hr) = world.hit(r, 0.001, INFINITY) {
        let emitted = hr.material.emitted(hr.coord, &hr.p);

        if let Some(albedo) = hr.material.scatter(r, &hr) {
            let cos_pdf = CosinePDF{normal: &hr.normal};
            let hit_pdf = HittablePDF{hittable: lights.objects[0].as_ref(), origin: &hr.p};
            let mix_pdf = MixturePDF{pdf1: &cos_pdf, pdf2: &hit_pdf};

            let (scattered_dir, opt_val) = mix_pdf.gen();
            let pdf_val = if opt_val.is_some() {
                opt_val.unwrap()
            } else {
                mix_pdf.eval(&scattered_dir)
            };
            let scattered1 = Ray { orig: hr.p, dir: scattered_dir };
            emitted + albedo *
             hr.material.scattering_pdf(r, &hr, &scattered1) *
             ray_color(&scattered1, background, world, lights, depth-1) *
             (1.0/pdf_val)
        } else {
            emitted
        }
    } else {
        background.clone()
    }
}