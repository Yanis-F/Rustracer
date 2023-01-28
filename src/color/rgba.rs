use egui::Color32;
use interpolation::*;

use super::RGB;

/// Numbers are in range [0; 1]
/// alpha not premultiplied
#[derive(Clone)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Into<Color32> for RGBA {
    fn into(self) -> Color32 {
		Color32::from_rgba_unmultiplied((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8, (self.a * 255.0) as u8)
    }
}

impl Default for RGBA {
    fn default() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
}

impl RGBA {
	pub fn to_rgb(&self, background_color: RGB) -> RGB {
		RGB { 
			r: background_color.r.lerp(&self.r, &self.a),
			g: background_color.g.lerp(&self.g, &self.a),
			b: background_color.b.lerp(&self.b, &self.a), 
		}
	}

	pub fn to_array(&self) -> [f32; 4] {
		[ self.r, self.g, self.b, self.a ]
	}

	pub fn subtractive_synthesis(a: &RGBA, b: &RGBA) -> RGBA {
		Self { 
			r: a.r * b.r,
			g: a.g * b.g,
			b: a.b * b.b,
			a: a.a * b.a, 
		}
	}

	pub fn additive_synthesis(a: &RGBA, b: &RGBA) -> RGBA {
		let inv_a = a.inverse();
		let inv_b = b.inverse();

		let inv_result = Self::subtractive_synthesis(&inv_a, &inv_b);
		inv_result.inverse()
	}

	pub fn inverse(&self) -> RGBA {
		Self { 
			r: 1.0 - self.r,
			g: 1.0 - self.g,
			b: 1.0 - self.b,
			a: 1.0 - self.a, 
		}
	}
}

impl Into<[f32; 4]> for RGBA {
	fn into(self) -> [f32; 4] {
		self.to_array()
	}
}
impl From<[f32; 4]> for RGBA {
	fn from(arr: [f32; 4]) -> Self {
		Self { r: arr[0], g: arr[1], b: arr[2], a: arr[3] }
	}
}


