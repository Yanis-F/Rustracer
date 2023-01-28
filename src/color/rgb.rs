use egui::Color32;

/// Numbers are in range [0; 1]
#[derive(Clone)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RGB {
	pub fn to_array(&self) -> [f32; 3] {
		[self.r, self.g, self.b]
	}

	pub fn subtractive_synthesis(a: &RGB, b: &RGB) -> RGB {
		Self { 
			r: a.r * b.r,
			g: a.g * b.g,
			b: a.b * b.b,
		}
	}

	pub fn additive_synthesis(a: &RGB, b: &RGB) -> RGB {
		let inv_a = a.inverse();
		let inv_b = b.inverse();

		let inv_result = Self::subtractive_synthesis(&inv_a, &inv_b);
		inv_result.inverse()
	}

	pub fn inverse(&self) -> RGB {
		Self { 
			r: 1.0 - self.r,
			g: 1.0 - self.g,
			b: 1.0 - self.b,
		}
	}
}

impl Into<Color32> for RGB {
    fn into(self) -> Color32 {
		Color32::from_rgb((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8)
    }
}

impl Into<[f32; 3]> for RGB {
	fn into(self) -> [f32; 3] {
		self.to_array()
	}
}
impl From<[f32; 3]> for RGB {
	fn from(arr: [f32; 3]) -> Self {
		RGB { r: arr[0], g: arr[1], b: arr[2] }
	}
}

#[allow(dead_code)]
impl RGB {
    pub const BLACK: RGB        = RGB { r: 0.0      , g: 0.0      , b: 0.0      } ;
    pub const DARK_GRAY: RGB    = RGB { r: 0.376471 , g: 0.376471 , b: 0.376471 } ;
    pub const GRAY: RGB         = RGB { r: 0.627451 , g: 0.627451 , b: 0.627451 } ;
    pub const LIGHT_GRAY: RGB   = RGB { r: 0.862745 , g: 0.862745 , b: 0.862745 } ;
    pub const WHITE: RGB        = RGB { r: 1.0      , g: 1.0      , b: 1.0      } ;
    pub const BROWN: RGB        = RGB { r: 0.647059 , g: 0.164706 , b: 0.164706 } ;
    pub const DARK_RED: RGB     = RGB { r: 0.545098 , g: 0.0      , b: 0.0      } ;
    pub const RED: RGB          = RGB { r: 1.0      , g: 0.0      , b: 0.0      } ;
    pub const LIGHT_RED: RGB    = RGB { r: 1.0      , g: 0.501961 , b: 0.501961 } ;
    pub const YELLOW: RGB       = RGB { r: 1.0      , g: 1.0      , b: 0.0      } ;
    pub const LIGHT_YELLOW: RGB = RGB { r: 1.0      , g: 1.0      , b: 0.878431 } ;
    pub const KHAKI: RGB        = RGB { r: 0.941176 , g: 0.901961 , b: 0.54902  } ;
    pub const DARK_GREEN: RGB   = RGB { r: 0.0      , g: 0.392157 , b: 0.0      } ;
    pub const GREEN: RGB        = RGB { r: 0.0      , g: 1.0      , b: 0.0      } ;
    pub const LIGHT_GREEN: RGB  = RGB { r: 0.564706 , g: 0.933333 , b: 0.564706 } ;
    pub const DARK_BLUE: RGB    = RGB { r: 0.0      , g: 0.0      , b: 0.545098 } ;
    pub const BLUE: RGB         = RGB { r: 0.0      , g: 0.0      , b: 1.0      } ;
    pub const LIGHT_BLUE: RGB   = RGB { r: 0.678431 , g: 0.847059 , b: 0.901961 } ;
    pub const GOLD: RGB         = RGB { r: 1.0      , g: 0.843137 , b: 0.0      } ;
}
