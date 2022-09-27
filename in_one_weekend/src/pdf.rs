use crate::vec3::*;
use crate::hit::*;

use std::f64::consts::PI;
use rand::random;

pub trait PDF {
	// Evaluates the PDF for the given direction.
	fn eval(&self, v: &Vec3) -> f64;
	// Generates a random Vector according to the pdf.
	fn gen(&self) -> Vec3;
}

pub fn gen_eval(origin: &Vec3, cos_pdf: &CosinePDF, w0: f64, pdfs: &[&dyn Hittable]) -> (Vec3, f64) {
	let mut w = 0.0;
	let r = random::<f64>();
	let v = if r <= w0 {
		cos_pdf.gen()
	} else {
		w = (1.0-w0)/(pdfs.len() as f64);
		let idx = ((r - w0) / w).trunc() as usize;
		unit_vector(pdfs[idx].gen_random_point(origin))
	};
	let gen = w0*cos_pdf.eval(&v) + pdfs.iter().map(|pdf| {pdf.pdf_eval(origin, &v) * w}).sum::<f64>();
	(v, gen)
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
	fn gen(&self) -> Vec3 {
		let rcv = random_cosine_vector(self.normal);
		rcv.0
	}
}