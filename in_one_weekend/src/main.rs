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
pub mod rectangle;

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

fn cornell_box() -> HittableList {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let green = Box::new(Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.12, 0.45, 0.15)})});
    objects.push(Box::new(YZRect{p1: Vec2(0.0, 0.0), p2: Vec2(555.0, 555.0), k: 555.0, material: green}));
    
    let red = Box::new(Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.65, 0.05, 0.05)})});
    objects.push(Box::new(YZRect{p1: Vec2(0.0, 0.0), p2: Vec2(555.0, 555.0), k: 0.0, material: red}));

    let light = Box::new(DiffuseLight{emit: Box::new(SolidColor{color: 15.0*Vec3(1.0, 1.0, 1.0)})});
    objects.push(Box::new(XZRect{p1: Vec2(213.0, 227.0), p2: Vec2(343.0, 332.0), k: 554.0, material: light}));

    let white = Box::new(Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.73, 0.73, 0.73)})});
    objects.push(Box::new(XZRect{p1: Vec2(0.0, 0.0), p2: Vec2(555.0, 555.0), k: 0.0, material: white}));

    let white = Box::new(Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.73, 0.73, 0.73)})});
    objects.push(Box::new(XZRect{p1: Vec2(0.0, 0.0), p2: Vec2(555.0, 555.0), k: 555.0, material: white}));

    let white = Box::new(Lambertian{albedo: Box::new(SolidColor{color: Vec3(0.73, 0.73, 0.73)})});
    objects.push(Box::new(XYRect{p1: Vec2(0.0, 0.0), p2: Vec2(555.0, 555.0), k: 555.0, material: white}));

    let bvh = BVHNode::new(objects);
    let mut world = HittableList{objects: vec![]};
    world.objects.push(Box::new(bvh));
    world
}

fn simple_light() -> HittableList {
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

    /*
    let difflight = DiffuseLight{emit: Box::new(SolidColor{color: Vec3(1.0, 1.0, 1.0)})};
    objects.push(Sphere::box_new(Vec3(0.0, 7.0, 0.0), 2.0, difflight));
    */

    let bvh = BVHNode::new(objects);
    let mut world = HittableList{objects: vec![]};
    world.objects.push(Box::new(bvh));
    world
}


fn earth() -> HittableList {
    let mut objects: Vec<Box<dyn Hittable>> = vec![];

    let earth = ImageTexture::new("earthmap.jpg").expect("failed to load an image");
    objects.push(Sphere::box_new(Vec3(0.0, 0.0, 0.0), 2.0, Lambertian{albedo: Box::new(earth)}));

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

#[derive(Copy, Clone)]
struct IColor(u8, u8, u8);
type Screen = Vec<IColor>;

fn set_color(screen: &mut Screen, image_width: usize, (row, col): (usize, usize), c: Color, samples_per_pixes: i32) {
    let scale = 1.0 / samples_per_pixes as f64;

    // gamma correction with gamma = 2
    let r = (c.0 * scale).sqrt();
    let g = (c.1 * scale).sqrt();
    let b = (c.2 * scale).sqrt();

    screen[row*image_width+col] = IColor(
    (256.0 * r.clamp(0.0, 0.999)) as u8,
    (256.0 * g.clamp(0.0, 0.999)) as u8,
    (256.0 * b.clamp(0.0, 0.999)) as u8,
    );
}

fn render(world: &Box<dyn Hittable>, (image_width, image_height): (usize, usize), max_depth: i32, background: &Color, cam: &Camera, samples_per_pixel: i32) -> Screen {
    let mut screen = vec![IColor(0,0,0); image_height*image_width];

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + random::<f64>()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, background, &**world, max_depth);
            }
            set_color(&mut screen, image_width, (j, i), pixel_color, samples_per_pixel);
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
    let mut aspect_ratio: f64 = 3.0 / 2.0;
    let mut image_width : usize = 1200;
    let max_depth: i32 = 50;

    // Camera
    let mut look_from = Vec3(13.0, 2.0, 3.0);
    let mut look_at = Vec3(0.0, 0.0, 0.0);
    let mut v_up = Vec3(0.0, 1.0, 0.0);
    let mut dist_to_focus = 10.0;
    let aperture = 0.1;
    let mut samples_per_pixel = 36;
    let mut background = Vec3(0.7, 0.8, 1.0);

    // World
    let wp: Box<dyn Hittable> = Box::new(match 6 {
        1 => random_scene(),
        2 => two_spheres(),
        3 => two_perlin_spheres(),
        4 => earth(),
        5 => {
            look_from = Vec3(26.0, 3.0, 0.0);
            look_at = Vec3(0.0, 2.0, 0.0);
            v_up = Vec3(0.0, 1.0, 0.0);
            background = Vec3(0.0, 0.0, 0.0);
            dist_to_focus = 20.0;
            simple_light()
        }
        _ => {
            /* world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = color(0,0,0);
            lookfrom = point3(278, 278, -800);
            lookat = point3(278, 278, 0);
            vfov = 40.0; */
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 24;
            look_from = Vec3(278.0, 278.0, -800.0);
            look_at = Vec3(278.0, 278.0, 0.0);
            background = Vec3(0.0, 0.0, 0.0);
            dist_to_focus = 40.0;
            cornell_box()
        }
    });

    let image_height: usize = ((image_width as f64) / aspect_ratio) as usize;

    let cam = build_camera(look_from, look_at, v_up, 20.0, aspect_ratio, aperture, dist_to_focus);

    if let Some(screen) = (0..samples_per_pixel).collect::<Vec<usize>>().par_iter()
        .map(|n| {
            eprintln!("iteration {} started", n);
            render(&wp, (image_width, image_height), max_depth, &background, &cam, 1)
        })
        .reduce_with(|mut s1, s2| {
        for j in 0..image_height {
            for i in 0..image_width {
                let pos = j*image_width+i;
                s1[pos] = blend(s1[pos], s2[pos]);
            }
        }
        s1
    }) {
        print!("P3\n{} {}\n255\n", image_width, image_height);
        for j in (0..image_height).rev() {
            for i in 0..image_width {
                let ic = screen[j*image_width+i];
                println!("{} {} {}", ic.0, ic.1, ic.2);
            }
        }
    }
}
