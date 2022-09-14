use image::RgbImage;
use image::io::Reader as ImageReader;

use crate::vec3::*;

pub trait Texture: Sync {
	// Returns a color at surface coordinates `coord`. (TODO: what is `p` then?)
	fn value(&self, coord: Vec2, p: &Point3) -> Color;
}

pub struct SolidColor {
	pub color: Color,
}

impl Texture for SolidColor {
	fn value(&self, _coord: Vec2, _p: &Point3) -> Color {
		self.color
	}
}

pub struct CheckerTexture {
	pub odd: Box<dyn Texture>,
	pub even: Box<dyn Texture>,
}

impl Texture for CheckerTexture {
	fn value(&self, coord: Vec2, p: &Point3) -> Color {
		let sines = (10.0*p.0).sin() * (10.0*p.1).sin() * (10.0*p.2).sin();
		if sines < 0.0 {
			self.odd.value(coord, p)
		} else {
			self.even.value(coord, p)
		}
	}
}

pub struct ImageTexture {
	img: RgbImage,
}

impl ImageTexture {
	pub fn new(path: &str) -> Result<ImageTexture, image::ImageError> {
    	let img = ImageReader::open(path)?.decode()?.into_rgb8();
		Ok(ImageTexture { img })
	}
}

impl Texture for ImageTexture {
	fn value(&self, coord: Vec2, _p: &Point3) -> Color {
		let rgb = self.img.get_pixel((coord.0 * (self.img.width() as f64)) as u32 , ((1.0-coord.1) * (self.img.height() as f64)) as u32);
		Vec3(rgb[0] as f64 / 256.0,
			 rgb[1] as f64 / 256.0,
			 rgb[2] as f64 / 256.0,)
	}
}