use std::{ops, f64::INFINITY};
use rand::Rng;

#[derive(Copy, Clone)]
struct Vec3 {
    e0: f64,
    e1: f64,
    e2: f64,
}
type Color = Vec3;
type Point3 = Vec3;

impl Vec3 {
    fn length(self) -> f64 {
        return (self.e0 * self.e0 + self.e1*self.e1 + self.e2*self.e2).sqrt();
    }

    fn length_squared(self) -> f64 {
        return self.e0 * self.e0 + self.e1*self.e1 + self.e2*self.e2;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3{e0: self.e0+rhs.e0, e1: self.e1 + rhs.e1, e2: self.e2 + rhs.e2,}
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3{e0: self.e0+rhs, e1: self.e1 + rhs, e2: self.e2 + rhs,}
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3{e0: self.e0-rhs.e0, e1: self.e1 - rhs.e1, e2: self.e2 - rhs.e2,}
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3{e0: rhs * self.e0, e1: rhs * self.e1, e2: rhs * self.e2}
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3{e0: self * rhs.e0, e1: self * rhs.e1, e2: self * rhs.e2}
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3{e0: self.e0/rhs, e1: self.e1/rhs, e2: self.e2/rhs}
    }
}

fn unit_vector(v: Vec3) -> Vec3 {
    v/v.length()
}
fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e0*v.e0 + u.e1*v.e1 + u.e2*v.e2
}

fn write_color(c: Color, samples_per_pixes: i32) {
    let scale = 1.0 / samples_per_pixes as f64;

    let r = c.e0 * scale;
    let g = c.e1 * scale;
    let b = c.e2 * scale;

    println!("{} {} {}",
    (256.0 * r.clamp(0.0, 0.999)) as i32,
    (256.0 * g.clamp(0.0, 0.999)) as i32,
    (256.0 * b.clamp(0.0, 0.999)) as i32,
 );
}

struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { outward_normal * (-1.0) }
    }
}
trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct Sphere {
    center: Point3,
    radius: f64,
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
        let mut hr = HitRecord { p: r.at(root), normal: (r.at(root) - self.center) / self.radius, t: root, front_face: false, };
        let outward_normal = (hr.p - self.center) / self.radius;
        hr.set_face_normal(r, outward_normal);
        return Some(hr);
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
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

struct Ray {
    orig: Point3,
    dir: Vec3,
}
impl Ray {
    fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}


fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    if let Some(hr) = world.hit(r, 0.0, INFINITY) {
        0.5 * (hr.normal + Color{e0: 1.0, e1: 1.0, e2: 1.0})
    } else {
        let unit_direction = unit_vector(r.dir);
        let t = 0.5*(unit_direction.e1+1.0);
        (1.0-t)*Color{e0: 1.0, e1: 1.0, e2: 1.0} + t*Color{e0: 0.5, e1: 0.7, e2: 1.0}
    }
}

struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray { orig: self.origin, dir: self.lower_left_corner + u*self.horizontal + v*self.vertical }
    }
}
fn build_camera() -> Camera {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3{e0: 0.0, e1: 0.0, e2: 0.0};
    let horizontal = Vec3 { e0: viewport_width, e1: 0.0, e2: 0.0 };
    let vertical = Vec3 { e0:0.0, e1: viewport_height, e2: 0.0 };

    Camera{
        origin,
        horizontal,
        vertical,
        lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vec3{e0:0.0, e1:0.0, e2:focal_length },
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let aspect_ratio = 16.0 / 9.0;
    let image_width : i32 = 400;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList{objects: vec![]};
    world.objects.push(Box::new(Sphere{center: Point3{e0: 0.0, e1: 0.0, e2: -1.0}, radius: 0.5}));
    world.objects.push(Box::new(Sphere{center: Point3{e0: 0.0, e1: -100.5, e2: -1.0}, radius: 100.0}));

    // Camera
    let cam = build_camera();

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("{} scan lines remaining", j);
        for i in 0..image_width {
            let mut pixel_color = Color{e0: 0.0, e1: 0.0, e2: 0.0 };
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    } 
}
