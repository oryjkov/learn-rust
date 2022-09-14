use rand::prelude::*;
use rand::thread_rng;

use crate::vec3::*;
use crate::texture::*;

const POINT_COUNT: usize = 256;

pub struct NoiseTexture {
	noise: Perlin,
	scale: f64,
}

impl NoiseTexture {
	pub fn new(scale: f64) -> NoiseTexture {
		NoiseTexture { noise: Perlin::new(), scale }
	}
}

impl Texture for NoiseTexture {
	fn value(&self, _: Vec2, p: &Point3) -> Color {
		Vec3(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale*p.2 + 10.0*self.noise.turb(&(self.scale* *p), 7)).sin())
	}
}

struct Perlin {
	ranvec: [Point3; POINT_COUNT],
	perm_x: [usize; POINT_COUNT],
	perm_y: [usize; POINT_COUNT],
	perm_z: [usize; POINT_COUNT],
}

impl Perlin {
	fn new() -> Perlin {
		let mut rng = thread_rng();

		let mut ranvec = [Vec3(0.0, 0.0, 0.0); POINT_COUNT];
		for i in 0..POINT_COUNT {
			ranvec[i] = unit_vector(random_vec3_bounds(-1.0, 1.0));
		}

		let mut perm_x: [usize; POINT_COUNT] = [0; POINT_COUNT];
		let mut perm_y: [usize; POINT_COUNT] = [0; POINT_COUNT];
		let mut perm_z: [usize; POINT_COUNT] = [0; POINT_COUNT];
		for i in 0..POINT_COUNT {
			perm_x[i] = i;
			perm_y[i] = i;
			perm_z[i] = i;
		}
		perm_x.shuffle(&mut rng);
		perm_y.shuffle(&mut rng);
		perm_z.shuffle(&mut rng);

		Perlin { ranvec, perm_x, perm_y, perm_z }
	}

	fn noise(&self, p: &Point3) -> f64 {
		let u = p.0 - p.0.floor();
		let v = p.1 - p.1.floor();
		let w = p.2 - p.2.floor();

		let i = p.0.floor() as i32;
		let j = p.1.floor() as i32;
		let k = p.2.floor() as i32;

		let mut c = [[[Vec3(0.0, 0.0, 0.0); 2]; 2]; 2];
		for di in 0..2 {
			for dj in 0..2 {
				for dk in 0..2 {
					c[di][dj][dk] = self.ranvec[(
						self.perm_x[((i + (di as i32)) & 255) as usize] ^
						self.perm_y[((j + (dj as i32)) & 255) as usize] ^
						self.perm_z[((k + (dk as i32)) & 255) as usize]
					) as usize];
				}
			}
		}
		perlin_interp(&c, u, v, w)
	}

	fn turb(&self, p: &Point3, depth: i32) -> f64 {
		let mut acc = 0.0;
		let mut temp_p = *p;
		let mut weight = 1.0;

		for _ in 0..depth {
			acc += weight * self.noise(&temp_p);
			weight *= 0.5;
			temp_p = 2.0 * temp_p;
		}
		acc.abs()
	}
}


fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
	let uu = u*u*(3.0-2.0*u);
	let vv = v*v*(3.0-2.0*v);
	let ww = w*w*(3.0-2.0*w);

	let mut acc = 0.0;
	for i in 0..2 {
		for j in 0..2 {
			for k in 0..2 {
				let weight_v = Vec3(u - i as f64, v - j as f64, w - k as f64);
				acc += ((i as f64)*uu + (1.0 - i as f64)*(1.0-uu)) *
				       ((j as f64)*vv + (1.0 - j as f64)*(1.0-vv)) *
					   ((k as f64)*ww + (1.0 - k as f64)*(1.0-ww)) *
					   dot(c[i][j][k], weight_v);
			}
		}
	}
	acc
}