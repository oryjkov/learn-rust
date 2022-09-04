use rand::Rng;

pub mod vec3;
pub mod camera;
pub mod hit;
pub mod metal;

use vec3::*;
use camera::*;
use hit::*;
use crate::metal::*;


fn main() {
    let mut rng = rand::thread_rng();

    let aspect_ratio = 16.0 / 9.0;
    let image_width : i32 = 400;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    let max_depth = 50;

    // World
    let mut world = HittableList{objects: vec![]};
    world.objects.push(Box::new(Sphere{center: Vec3(0.0, 0.0, -1.0), radius: 0.5, material: &Lambertian{albedo: Vec3(0.1, 0.2, 0.5)}}));
    world.objects.push(Box::new(Sphere{center: Vec3(0.0, -100.5, -1.0), radius: 100.0, material: &Lambertian{albedo: Vec3(0.8, 0.8, 0.0)}}));
    world.objects.push(Box::new(Sphere{center: Vec3(-1.0, 0.0, -1.0), radius: 0.5, material: &Dielectric{ir: 1.5}}));
    world.objects.push(Box::new(Sphere{center: Vec3(-1.0, 0.0, -1.0), radius: -0.4, material: &Dielectric{ir: 1.5}}));
    world.objects.push(Box::new(Sphere{center: Vec3(1.0, 0.0, -1.0), radius: 0.5, material: &Metal{albedo: Vec3(0.8, 0.6, 0.2), fuzz: 0.0}}));
    //world.objects.push(Box::new(Sphere{center: Vec3(0.0, -100.5, -1.0), radius: 100.0}));

    // Camera
    let cam = build_camera(Vec3(-2.0, 2.0, 1.0), Vec3(0.0, 0.0, -1.0), Vec3(0.0, 1.0, 0.0), 90.0, aspect_ratio);

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("{} scan lines remaining", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    } 
}
