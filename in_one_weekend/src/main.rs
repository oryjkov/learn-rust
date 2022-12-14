use image::Frame;
use image::Rgba;
use image::codecs::gif::GifEncoder;
use image::codecs::gif::Repeat;
use std::fs::File;
use image::RgbaImage;
use rand::random;
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

pub mod vec3;
pub mod dielectric;
pub mod bvh_node;
pub mod camera;
pub mod hit;
pub mod metal;
pub mod aabb;
pub mod ray;
pub mod sphere;
pub mod texture;
pub mod perlin;
pub mod rectangle;
pub mod material;
pub mod lambertian;
pub mod pdf;

use crate::vec3::*;
use camera::*;
use hit::*;
use crate::metal::*;
use crate::ray::*;
use crate::bvh_node::*;
use crate::sphere::*;
use crate::texture::*;
use crate::perlin::*;
use crate::rectangle::*;
use crate::lambertian::*;
use crate::dielectric::*;

fn test_sphere() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let gray = Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.18, 0.18, 0.18)})};
    objects.push(Sphere::box_new(Vec3(200.0, 200.0, 200.0), 100.0, gray));

    let light = Box::new(DiffuseLight{emit: Box::new(SolidColor{color: 15.0*Vec3(1.0, 1.0, 1.0)})});
    objects.push(Box::new(XZRect{p1: Vec2(210.0, 227.0), p2: Vec2(343.0, 332.0), k: 554.0, material: light}));
    objects
}

fn cornell_box() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let green = Box::new(Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.12, 0.45, 0.15)})});
    objects.push(Box::new(YZRect{p1: Vec2(0.0, 0.0), p2: Vec2(555.0, 555.0), k: 555.0, material: green}));
    
    let red = Box::new(Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.65, 0.05, 0.05)})});
    objects.push(Box::new(YZRect{p1: Vec2(0.0, 0.0), p2: Vec2(555.0, 555.0), k: 0.0, material: red}));

    let light = Box::new(DiffuseLight{emit: Box::new(SolidColor{color: 15.0*Vec3(1.0, 1.0, 1.0)})});
    objects.push(Box::new(XZRect{p1: Vec2(213.0, 227.0), p2: Vec2(343.0, 332.0), k: 554.0, material: light}));
    let light = Box::new(DiffuseLight{emit: Box::new(SolidColor{color: 15.0*Vec3(1.0, 1.0, 1.0)})});
    objects.push(Box::new(XZRect{p1: Vec2(113.0, 127.0), p2: Vec2(243.0, 192.0), k: 554.0, material: light}));

    let white = Box::new(Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.73, 0.73, 0.73)})});
    objects.push(Box::new(XZRect{p1: Vec2(0.0, 0.0), p2: Vec2(555.0, 555.0), k: 0.0, material: white}));

    let white = Box::new(Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.73, 0.73, 0.73)})});
    objects.push(Box::new(XZRect{p1: Vec2(0.0, 0.0), p2: Vec2(555.0, 555.0), k: 555.0, material: white}));

    let white = Box::new(Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.73, 0.73, 0.73)})});
    objects.push(Box::new(XYRect{p1: Vec2(0.0, 0.0), p2: Vec2(555.0, 555.0), k: 555.0, material: white}));

    objects.push(Sphere::box_new(Vec3(200.0, 350.0, 200.0), 100.0, Metal{albedo: Vec3(1.0, 1.0, 1.0), fuzz: 0.0}));
    objects.push(Sphere::box_new(Vec3(400.0, 350.0, 200.0), 80.0, Dielectric{ir: 1.5}));

    objects
}

fn simple_light() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let noise = Box::new(NoiseTexture::new(8.0));
    objects.push(Sphere::box_new(Vec3(0.0, -1000.0, 0.0), 1000.0, Lambertian{albedo: noise}));
    //let noise = Box::new(NoiseTexture::new(4.0));
    //objects.push(Sphere::box_new(Vec3(0.0, 2.0, 0.0), 2.0, Lambertian{albedo: noise}));
    let green = Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.12, 0.85, 0.15)})};
    objects.push(Sphere::box_new(Vec3(0.0, 2.0, 0.0), 2.0, green));

    let difflight = Box::new(DiffuseLight{emit: Box::new(SolidColor{color: 0.2*Vec3(1.0, 1.0, 1.0)})});
    objects.push(Box::new(XYRect{p1: Vec2(-1.0, 1.0), p2: Vec2(1.0, 3.0), k: -2.0, material: difflight}));

    let difflight = Box::new(DiffuseLight{emit: Box::new(SolidColor{color: 1.0*Vec3(1.0, 1.0, 1.0)})});
    objects.push(Box::new(XYRect{p1: Vec2(-1.0, 1.0), p2: Vec2(1.0, 3.0), k: 2.0, material: difflight}));

    let difflight = Box::new(DiffuseLight{emit: Box::new(SolidColor{color: 1.0*Vec3(1.0, 1.0, 1.0)})});
    objects.push(Box::new(XZRect{p1: Vec2(-1.0, -1.0), p2: Vec2(1.0, 1.0), k: 4.0, material: difflight}));

    let difflight = Box::new(DiffuseLight{emit: Box::new(SolidColor{color: 4.0*Vec3(1.0, 1.0, 1.0)})});
    objects.push(Box::new(XZRect{p1: Vec2(-1.0, -1.0), p2: Vec2(1.0, 1.0), k: 0.0, material: difflight}));

    objects
}


fn earth() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let earth = ImageTexture::new("earthmap.jpg").expect("failed to load an image");
    objects.push(Sphere::box_new(Vec3(0.0, 0.0, 0.0), 2.0, Lambertian{albedo: Box::new(earth)}));

    objects
}

fn two_perlin_spheres() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let noise = Box::new(NoiseTexture::new(8.0));
    objects.push(Sphere::box_new(Vec3(0.0, -1000.0, 0.0), 1000.0, Lambertian{albedo: noise}));
    let noise = Box::new(NoiseTexture::new(4.0));
    objects.push(Sphere::box_new(Vec3(0.0, 2.0, 0.0), 2.0, Lambertian{albedo: noise}));

    objects
}

fn two_spheres() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let checker = Box::new(CheckerTexture{
        odd: Box::new(SolidColor{color: Vec3(0.2,0.3,0.1)}),
        even: Box::new(SolidColor{color: Vec3(0.9,0.9,0.9)}),
    });
    objects.push(Sphere::box_new(Vec3(0.0, -10.0, 0.0), 10.0, Lambertian{albedo: checker}));

    let checker = Box::new(CheckerTexture{
        odd: Box::new(SolidColor{color: Vec3(0.2,0.3,0.1)}),
        even: Box::new(SolidColor{color: Vec3(0.9,0.9,0.9)}),
    });
    objects.push(Sphere::box_new(Vec3(0.0, 10.0, 0.0), 10.0, Lambertian{albedo: checker}));

    objects
}

fn random_scene() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];
    let checker = Box::new(CheckerTexture{
        odd: Box::new(SolidColor{color: Vec3(0.2,0.3,0.1)}),
        even: Box::new(SolidColor{color: Vec3(0.9,0.9,0.9)}),
    });
    objects.push(Sphere::box_new(Vec3(0.0, -1000.0, 0.0), 1000.0, Lambertian{albedo: checker}));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Vec3(a as f64 + 0.9*random::<f64>(), 0.2, b as f64 + 0.9*random::<f64>());
            if (center-Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    x if x<0.8 => {
                        let color = random_vec3()*random_vec3();
                        let material = Lambertian{albedo: Box::new(SolidColor{color})};
                        objects.push(Sphere::box_new(center, 0.2, material));
                    }
                    x if x < 0.95 => {
                        let albedo = random_vec3_bounds(0.5, 1.0);
                        let fuzz = random::<f64>()*0.5;
                        let material = Metal{albedo, fuzz};
                        objects.push(Sphere::box_new(center, 0.2, material));
                    }
                    x if x>=0.95 => {
                        let material = Dielectric{ir: 1.5};
                        objects.push(Sphere::box_new(center, 0.2, material));

                    }
                    _ => {}
                }
            }
        }
    }
    objects.push(Sphere::box_new(Vec3(0.0, 1.0, 0.0), 1.0, Dielectric{ir: 1.5}));

    objects.push(Sphere::box_new(Vec3(-4.0, -1.0, 0.0), 1.0, Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.4, 0.2, 0.1)})}));
    objects.push(Sphere::box_new(Vec3(4.0, 1.0, 0.0), 1.0, Metal{albedo: Vec3(0.7, 0.6, 0.5), fuzz: 0.0}));

    // Using BVH reduces the time to render (1200 width, 50 samples/pixel) from 602s to 155s.
    objects
}

#[derive(Copy, Clone)]
struct IColor(u8, u8, u8);
type Screen = Vec<Color>;

fn render(world: &dyn Hittable, lights: &Vec<&dyn Hittable>, (image_width, image_height): (usize, usize), max_depth: i32, background: &Color, cam: &Camera) -> Screen {
    let mut screen = vec![Vec3(0.0,0.0,0.0); image_height*image_width];

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            let u = (i as f64 + random::<f64>()) / (image_width as f64 - 1.0);
            let v = (j as f64 + random::<f64>()) / (image_height as f64 - 1.0);
            let r = cam.get_ray(u, v);
            pixel_color = pixel_color + ray_color(&r, background, world, lights, max_depth);
            screen[j*image_width+i] = pixel_color;
        }
    } 

    screen
}

#[derive(Clone)]
struct View {
    look_from: Vec3,
    look_at: Vec3,
    v_up: Vec3,
    vfov_deg: f64,
    aperture: f64,
    dist_to_focus: f64,
}

struct Scene {
    aspect_ratio: f64,
    image_width: usize,
    samples_per_pixel: usize,
    background: Vec3,
    max_depth: i32,
    save_temps: usize,
}

fn build_frame(bar: &ProgressBar, world: &dyn Hittable, lights: &Vec<&dyn Hittable>, v: &View, s: &Scene) -> RgbaImage {
    let image_height: usize = ((s.image_width as f64) / s.aspect_ratio) as usize;
    let image_width = s.image_width;
    let save_temps = s.save_temps;

    let img = {
        let mut img = RgbaImage::new(s.image_width as u32, image_height as u32);

        let (tx, rx): (Sender<Screen>, Receiver<Screen>) = channel();
        let atx = Arc::new(Mutex::new(tx));

        let handler = std::thread::spawn(move || {
            let mut screen_accumulator = vec![Vec3(0.0,0.0,0.0); image_height*image_width];
            let mut cnt = 0;
            loop {
                cnt += 1;
                let tmp_r = rx.recv();
                if (save_temps > 0 && cnt % save_temps == 0) || tmp_r.is_err() {
                    for j in 0..image_height {
                        for i in 0..image_width {
                            let c: Color = (1.0/cnt as f64) * screen_accumulator[j*image_width+i];

                            // gamma correction with gamma = 2
                            let r = (256.0 * c.0.sqrt().clamp(0.0, 0.999)) as u8;
                            let g = (256.0 * c.1.sqrt().clamp(0.0, 0.999)) as u8;
                            let b = (256.0 * c.2.sqrt().clamp(0.0, 0.999)) as u8;

                            img.put_pixel(i as u32, (image_height-j-1) as u32, Rgba([r, g, b, 255]));
                        }
                    }
                    img.save("tmp.png").expect("temp save fail");
                }
                if tmp_r.is_err() {
                    break;
                }
                let new_screen = tmp_r.unwrap();
                for j in 0..image_height {
                    for i in 0..image_width {
                        let pos = j*image_width+i;
                        screen_accumulator[pos] = screen_accumulator[pos] + new_screen[pos];
                    }
                }
            }
            img
        });

        let cam = build_camera(v.look_from, v.look_at, v.v_up, v.vfov_deg, s.aspect_ratio, v.aperture, v.dist_to_focus);
        let _: Vec<_> = (0..s.samples_per_pixel).collect::<Vec<usize>>().par_iter()
            .map(|_| {
                let s = render(world, lights, (s.image_width, image_height), s.max_depth, &s.background, &cam);
                atx.lock().unwrap().send(s).expect("send failed");
                bar.inc(1);
                ()
            }).collect();
        drop(atx);

        handler.join().unwrap()
    };
    img
}

struct Animation {
    num_frames: usize,
    f: fn (&View, f64) -> View,
}

use indicatif::ProgressBar;

fn main() {
    let mut s = Scene {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        max_depth: 50,
        samples_per_pixel: 36,
        background: Vec3(0.7, 0.8, 1.0),
        save_temps: 30,
    };
    let mut v = View {
        look_from: Vec3(13.0, 2.0, 3.0),
        look_at: Vec3(0.0, 0.0, 0.0),
        v_up: Vec3(0.0, 1.0, 0.0),
        dist_to_focus: 10.0,
        aperture: 0.0,
        vfov_deg: 40.0,
    };
    let mut a = Animation {
        num_frames: 1,
        f: |v, _t| { v.clone() },
    };

    // World
    let objects = match 1 {
        1 => {
            v.aperture = 0.1;
            v.vfov_deg = 20.0;
            random_scene()
        }
        2 => {
            v.vfov_deg = 20.0;
            two_spheres()
        }
        3 => {
            v.vfov_deg = 20.0;
            two_perlin_spheres()
        }
        4 => {
            v.vfov_deg = 20.0;
            earth()
        }
        5 => {
            v.look_from = Vec3(26.0, 3.0, 0.0);
            v.look_at = Vec3(0.0, 2.0, 0.0);
            v.v_up = Vec3(0.0, 1.0, 0.0);
            v.vfov_deg = 10.0;
            s.background = Vec3(0.0, 0.0, 0.0);
            s.samples_per_pixel = 800;
            simple_light()
        }
        6 => {
            s.aspect_ratio = 1.0;
            s.image_width = 600;
            s.samples_per_pixel = 1000;
            v.look_from = Vec3(278.0, 278.0, -800.0);
            v.look_at = Vec3(278.0, 278.0, 0.0);
            s.background = Vec3(0.0, 0.0, 0.0);

            cornell_box()
        }
        7 => {
            s.aspect_ratio = 1.0;
            s.image_width = 300;
            s.samples_per_pixel = 800;
            v.look_from = Vec3(278.0, 278.0, -800.0);
            v.look_at = Vec3(278.0, 278.0, 0.0);
            s.background = Vec3(0.0, 0.0, 0.0);

            a.num_frames = 100;
            a.f = |v, t| { let mut z = v.clone(); z.look_from = Vec3(2.0*278.0*t, 278.0, -800.0); z};
            cornell_box()
        }
        _ => {
            s.aspect_ratio = 1.0;
            s.image_width = 600;
            s.samples_per_pixel = 1000;
            v.look_from = Vec3(278.0, 278.0, -800.0);
            v.look_at = Vec3(278.0, 278.0, 0.0);
            s.background = Vec3(1.0, 1.0, 1.0);

            test_sphere()
        }
    };

    let world = Box::new(BVHNode::new(objects));
    // Lights refers to objects in the world, so world can not be moved as long as lights is around.
    // This is OK here since we need lights only within the world scope.
    let lights = world.pick_lights();

    let bar = ProgressBar::new((a.num_frames * s.samples_per_pixel) as u64);
    let fs = (0..a.num_frames).collect::<Vec<usize>>().par_iter().map(|frame_num| {
        build_frame(&bar, &*world, &lights, &(a.f)(&v, *frame_num as f64 / a.num_frames as f64), &s)
    }).collect::<Vec<RgbaImage>>();

    let file_out = File::create("out.gif").unwrap();
    let mut encoder = GifEncoder::new(file_out);
    encoder.set_repeat(Repeat::Infinite).expect("setting repeat failed");
    for i in 0..fs.len() {
        encoder.encode_frame(Frame::new(fs[i].clone())).expect("failed encoding");
    };
    for img in fs.into_iter().rev() {
        encoder.encode_frame(Frame::new(img)).expect("failed encoding");
    };
}
