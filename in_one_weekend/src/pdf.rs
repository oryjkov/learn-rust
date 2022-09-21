use crate::vec3::*;
use crate::hit::*;

use std::f64::consts::PI;
use rand::random;

pub trait PDF {
	// Evaluates the PDF for the given direction.
	fn eval(&self, v: &Vec3) -> f64;
	// Generates a random Vector according to the pdf.
	fn gen(&self) -> (Vec3, Option<f64>);
}

pub struct MixturePDF<'a> {
	pub pdf1: &'a dyn PDF,
	pub pdf2: &'a dyn PDF,
}

impl <'a> PDF for MixturePDF<'a> {
	fn eval(&self, v: &Vec3) -> f64 {
		0.5*self.pdf1.eval(&v) + 0.5*self.pdf2.eval(&v)
	}

	fn gen(&self) -> (Vec3, Option<f64>) {
		if random::<f64>() < 0.5 {
			self.pdf1.gen()
		} else {
			self.pdf2.gen()
		}
	}
}

pub struct HittablePDF<'a> {
	pub hittable: &'a dyn Hittable,
	// Origin relative to which the random vector is generated. 
	pub origin: &'a Vec3,
}

impl <'a> PDF for HittablePDF<'a> {
	fn eval(&self, dir: &Vec3) -> f64 {
		self.hittable.pdf_eval(self.origin, dir)
	}
	fn gen(&self) -> (Vec3, Option<f64>) {
		let v = self.hittable.gen_random_point(self.origin);
		(unit_vector(v), None)
	}
}

pub struct CosinePDF<'a> {
	pub normal: &'a Vec3,
}

// Generates a vector in the hemisphere defined by normal `normal`.
fn random_cosine_vector(normal: &Vec3) -> (Vec3, f64) {
	let r1 = random::<f64>();
	let r2 = random::<f64>();

	let phi = 2.0*PI*r1;
	let z = (1.0-r2).sqrt();
	let y = phi.sin() * r2.sqrt();
	let x = phi.cos() * r2.sqrt();

	let w = unit_vector(*normal);
	let a = if w.0.abs() > 0.9 {Vec3(0.0, 1.0, 0.0)} else {Vec3(1.0,0.0,0.0)};
	let v = unit_vector(cross(w, a));
	let u = cross(w, v);
	(x*u+y*v+z*w, z)
}

impl <'a> PDF for CosinePDF<'a> {
	fn eval(&self, v: &Vec3) -> f64 {
		let cos_theta = dot(*self.normal, *v);
		if cos_theta > 0.0 {
			cos_theta/PI
		} else {
			0.0
		}
	}
	fn gen(&self) -> (Vec3, Option<f64>) {
		let rcv = random_cosine_vector(self.normal);
		(rcv.0, Some(rcv.1))
	}
}