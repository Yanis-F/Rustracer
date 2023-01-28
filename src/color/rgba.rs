use egui::Color32;
use interpolation::*;

use super::Rgb;

/// Numbers are in range [0; 1]
/// alpha not premultiplied
#[derive(Clone)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<Rgba> for Color32 {
    fn from(val: Rgba) -> Self {
        Color32::from_rgba_unmultiplied(
            (val.r * 255.0) as u8,
            (val.g * 255.0) as u8,
            (val.b * 255.0) as u8,
            (val.a * 255.0) as u8,
        )
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

impl Rgba {
    pub fn to_rgb(&self, background_color: Rgb) -> Rgb {
        Rgb {
            r: background_color.r.lerp(&self.r, &self.a),
            g: background_color.g.lerp(&self.g, &self.a),
            b: background_color.b.lerp(&self.b, &self.a),
        }
    }

    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn subtractive_synthesis(a: &Rgba, b: &Rgba) -> Rgba {
        Self {
            r: a.r * b.r,
            g: a.g * b.g,
            b: a.b * b.b,
            a: a.a * b.a,
        }
    }

    pub fn additive_synthesis(a: &Rgba, b: &Rgba) -> Rgba {
        let inv_a = a.inverse();
        let inv_b = b.inverse();

        let inv_result = Self::subtractive_synthesis(&inv_a, &inv_b);
        inv_result.inverse()
    }

    pub fn inverse(&self) -> Rgba {
        Self {
            r: 1.0 - self.r,
            g: 1.0 - self.g,
            b: 1.0 - self.b,
            a: 1.0 - self.a,
        }
    }
}

impl From<Rgba> for [f32; 4] {
    fn from(val: Rgba) -> Self {
        val.to_array()
    }
}
impl From<[f32; 4]> for Rgba {
    fn from(arr: [f32; 4]) -> Self {
        Self {
            r: arr[0],
            g: arr[1],
            b: arr[2],
            a: arr[3],
        }
    }
}

#[allow(dead_code)]
impl Rgba {
    pub const BLACK: Rgba = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const DARK_GRAY: Rgba = Rgba {
        r: 0.376471,
        g: 0.376471,
        b: 0.376471,
        a: 1.0,
    };
    pub const GRAY: Rgba = Rgba {
        r: 0.627451,
        g: 0.627451,
        b: 0.627451,
        a: 1.0,
    };
    pub const LIGHT_GRAY: Rgba = Rgba {
        r: 0.862745,
        g: 0.862745,
        b: 0.862745,
        a: 1.0,
    };
    pub const WHITE: Rgba = Rgba {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BROWN: Rgba = Rgba {
        r: 0.647059,
        g: 0.164706,
        b: 0.164706,
        a: 1.0,
    };
    pub const DARK_RED: Rgba = Rgba {
        r: 0.545098,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const RED: Rgba = Rgba {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const LIGHT_RED: Rgba = Rgba {
        r: 1.0,
        g: 0.501961,
        b: 0.501961,
        a: 1.0,
    };
    pub const YELLOW: Rgba = Rgba {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const LIGHT_YELLOW: Rgba = Rgba {
        r: 1.0,
        g: 1.0,
        b: 0.878431,
        a: 1.0,
    };
    pub const KHAKI: Rgba = Rgba {
        r: 0.941176,
        g: 0.901961,
        b: 0.54902,
        a: 1.0,
    };
    pub const DARK_GREEN: Rgba = Rgba {
        r: 0.0,
        g: 0.392157,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Rgba = Rgba {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const LIGHT_GREEN: Rgba = Rgba {
        r: 0.564706,
        g: 0.933333,
        b: 0.564706,
        a: 1.0,
    };
    pub const DARK_BLUE: Rgba = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.545098,
        a: 1.0,
    };
    pub const BLUE: Rgba = Rgba {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const LIGHT_BLUE: Rgba = Rgba {
        r: 0.678431,
        g: 0.847059,
        b: 0.901961,
        a: 1.0,
    };
    pub const GOLD: Rgba = Rgba {
        r: 1.0,
        g: 0.843137,
        b: 0.0,
        a: 1.0,
    };
}
