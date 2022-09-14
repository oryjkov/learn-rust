use rand::random;

use rayon::prelude::*;

pub mod vec3;
pub mod bvh_node;
pub mod camera;
pub mod hit;
pub mod metal;
pub mod aabb;
pub mod ray;
pub mod sphere;
pub mod texture;
pub mod perlin;

use vec3::*;
use camera::*;
use hit::*;
use crate::metal::*;
use crate::ray::*;
use crate::bvh_node::*;
use crate::sphere::*;
use crate::texture::*;
use crate::perlin::*;

fn earth() -> HittableList {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let earth = ImageTexture::new("earthmap.jpg").expect("failed to load an image");
    objects.push(Sphere::box_new(Vec3(0.0, 0.0, 0.0), 2.0, Lambertian{albedo: Box::new(earth)}));

    // Using BVH reduces the time to render (1200 width, 50 samples/pixel) from 602s to 155s.
    let bvh = BVHNode::new(objects);
    let mut world = HittableList{objects: vec![]};
    world.objects.push(Box::new(bvh));
    world
}

fn two_perlin_spheres() -> HittableList {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let noise = Box::new(NoiseTexture::new(8.0));
    objects.push(Sphere::box_new(Vec3(0.0, -1000.0, 0.0), 1000.0, Lambertian{albedo: noise}));
    let noise = Box::new(NoiseTexture::new(4.0));
    objects.push(Sphere::box_new(Vec3(0.0, 2.0, 0.0), 2.0, Lambertian{albedo: noise}));

    // Using BVH reduces the time to render (1200 width, 50 samples/pixel) from 602s to 155s.
    let bvh = BVHNode::new(objects);
    let mut world = HittableList{objects: vec![]};
    world.objects.push(Box::new(bvh));
    world
}

fn two_spheres() -> HittableList {
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

    // Using BVH reduces the time to render (1200 width, 50 samples/pixel) from 602s to 155s.
    let bvh = BVHNode::new(objects);
    let mut world = HittableList{objects: vec![]};
    world.objects.push(Box::new(bvh));
    world
}

fn random_scene() -> HittableList {
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
    let bvh = BVHNode::new(objects);
    let mut world = HittableList{objects: vec![]};
    world.objects.push(Box::new(bvh));
    world
}

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH : usize = 1200;
const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
const MAX_DEPTH: i32 = 50;
const SAMPLES_PER_PIXEL: usize = 50;

#[derive(Copy, Clone)]
struct IColor(u8, u8, u8);
type Screen = [IColor; IMAGE_HEIGHT*IMAGE_WIDTH];

fn set_color(screen: &mut Screen, (row, col): (usize, usize), c: Color, samples_per_pixes: i32) {
    let scale = 1.0 / samples_per_pixes as f64;

    // gamma correction with gamma = 2
    let r = (c.0 * scale).sqrt();
    let g = (c.1 * scale).sqrt();
    let b = (c.2 * scale).sqrt();

    screen[row*IMAGE_WIDTH+col] = IColor(
    (256.0 * r.clamp(0.0, 0.999)) as u8,
    (256.0 * g.clamp(0.0, 0.999)) as u8,
    (256.0 * b.clamp(0.0, 0.999)) as u8,
    );
}

fn render(world: &Box<dyn Hittable>, cam: &Camera, samples_per_pixel: i32) -> Box<Screen> {
    let mut screen = Box::new([IColor(0,0,0); IMAGE_HEIGHT*IMAGE_WIDTH]);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &**world, MAX_DEPTH);
            }
            set_color(&mut screen, (j, i), pixel_color, samples_per_pixel);
        }
    } 

    screen
}

fn blend(c1: IColor, c2: IColor) -> IColor {
    IColor(
        ((c1.0 as i16 + c2.0 as i16)/2) as u8,
        ((c1.1 as i16 + c2.1 as i16)/2) as u8,
        ((c1.2 as i16 + c2.2 as i16)/2) as u8,)
}

fn main() {
    // World
    let wp: Box<dyn Hittable> = match 0 {
        1 => Box::new(random_scene()),
        2 => Box::new(two_spheres()),
        3 => Box::new(two_perlin_spheres()),
        _ => Box::new(earth()),
    };

    // Camera
    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let v_up = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = build_camera(look_from, look_at, v_up, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    if let Some(screen) = (0..SAMPLES_PER_PIXEL).collect::<Vec<usize>>().par_iter()
        .map(|n| {
            eprintln!("iteration {} started", n);
            render(&wp, &cam, 1)
        })
        .reduce_with(|mut s1, s2| {
        for j in 0..IMAGE_HEIGHT {
            for i in 0..IMAGE_WIDTH {
                let pos = j*IMAGE_WIDTH+i;
                s1[pos] = blend(s1[pos], s2[pos]);
            }
        }
        s1
    }) {
        print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
        for j in (0..IMAGE_HEIGHT).rev() {
            for i in 0..IMAGE_WIDTH {
                let ic = screen[j*IMAGE_WIDTH+i];
                println!("{} {} {}", ic.0, ic.1, ic.2);
            }
        }
    }
}
