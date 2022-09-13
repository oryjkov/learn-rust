use crate::vec3::*;

pub trait Texture: Sync {
	fn value(&self, coord: (f64, f64), p: &Point3) -> Color;
}

pub struct SolidColor {
	pub color: Color,
}

impl Texture for SolidColor {
	fn value(&self, _coord: (f64, f64), _p: &Point3) -> Color {
		self.color
	}
}

pub struct CheckerTexture {
	pub odd: Box<dyn Texture>,
	pub even: Box<dyn Texture>,
}

impl Texture for CheckerTexture {
	fn value(&self, coord: (f64, f64), p: &Point3) -> Color {
		let sines = (10.0*p.0).sin() * (10.0*p.1).sin() * (10.0*p.2).sin();
		if sines < 0.0 {
			self.odd.value(coord, p)
		} else {
			self.even.value(coord, p)
		}
	}
}