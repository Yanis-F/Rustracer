use egui::Color32;

/// Numbers are in range [0; 1]
#[derive(Clone)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Into<Color32> for RGB {
    fn into(self) -> Color32 {
		Color32::from_rgb((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8)
    }
}

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


///
///Numbers are in range [0; 1]
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
