use std::f64::consts::PI;

use rand::random;

use crate::vec3::*;
use crate::hit::*;
use crate::ray::*;
use crate::texture::*;
use crate::material::*;

pub struct Lambertian {
	pub albedo: Box<dyn Texture>,
}

// Generates a vector in the hemisphere defined by normal `n`.
fn random_cosine_vector(n: Vec3) -> (Vec3, f64) {
	let r1 = random::<f64>();
	let r2 = random::<f64>();

	let phi = 2.0*PI*r1;
	let z = (1.0-r2).sqrt();
	let y = phi.sin() * r2.sqrt();
	let x = phi.cos() * r2.sqrt();

	let w = unit_vector(n);
	let a = if w.0.abs() > 0.9 {Vec3(0.0, 1.0, 0.0)} else {Vec3(1.0,0.0,0.0)};
	let v = unit_vector(cross(w, a));
	let u = cross(w, v);
	(x*u+y*v+z*w, z)
}

impl Material for Lambertian {
	/* Scattering as done in the book.
	/* Hemispherical scattering - equal in all directions. */
	fn scatter(&self, _: &Ray, hr: &HitRecord) -> Option<(Color, Ray, f64)> {
		let scatter_direction = random_in_hemisphere(hr.normal);
		let albedo = self.albedo.value(hr.coord, &hr.p);
		let scattered_ray = Ray{orig: hr.p, dir: unit_vector(scatter_direction)};
		// Probability density of scattering in the returned direction. pdf(direction) in the book.
		let pdf = 0.5/PI;
		Some((albedo, scattered_ray, pdf))
	}

	/* Original scatter implementation from the book. The one that does scattering by bolting on
	   a random vector on a unit sphere on top of the normal. This model does not have the right pdf however.  */
	fn scatter(&self, _: &Ray, hr: &HitRecord) -> Option<(Color, Ray, f64)> {
		let mut scatter_direction = hr.normal + random_unit_vector();
		if scatter_direction.near_zero() {
			scatter_direction = hr.normal
		}
		let albedo = self.albedo.value(hr.coord, &hr.p);
		let scattered_ray = Ray{orig: hr.p, dir: unit_vector(scatter_direction)};

		//let th = dot(hr.normal, scattered_ray.dir).acos();
		//let pdf = th.cos()*th.sin()/PI;
		// pdf = cos(theta)/pi
		let pdf = dot(hr.normal, scattered_ray.dir)/PI;
		Some((albedo, scattered_ray, pdf))
	}
	*/
	fn scatter(&self, _: &Ray, hr: &HitRecord) -> Option<(Color, Ray, f64)> {
		let (scatter_direction, cos_theta) = random_cosine_vector(hr.normal);

		let scattered_ray = Ray{orig: hr.p, dir: scatter_direction};
		let albedo = self.albedo.value(hr.coord, &hr.p);
		// Normalize by the PDF of direction which is cos(theta)/PI by construction in random_cosine_vector()
		let pdf = cos_theta/PI;
		Some((albedo, scattered_ray, pdf))
	}

	fn scattering_pdf(&self, _r_in: &Ray, hr: &HitRecord, scattered: &Ray) -> f64 {
		let cosine = dot(hr.normal, unit_vector(scattered.dir));
		if cosine > 0.0 {
			cosine / PI
			//0.5 / PI
		} else {
			0.0
		}
	}

}
