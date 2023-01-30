use egui::Color32;

/// Numbers are in range [0; 1]
#[derive(Clone, Copy)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub fn to_array(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    pub fn subtractive_synthesis(a: &Rgb, b: &Rgb) -> Rgb {
        Self {
            r: a.r * b.r,
            g: a.g * b.g,
            b: a.b * b.b,
        }
    }

    pub fn additive_synthesis(a: &Rgb, b: &Rgb) -> Rgb {
        let inv_a = a.inverse();
        let inv_b = b.inverse();

        let inv_result = Self::subtractive_synthesis(&inv_a, &inv_b);
        inv_result.inverse()
    }

    pub fn inverse(&self) -> Rgb {
        Self {
            r: 1.0 - self.r,
            g: 1.0 - self.g,
            b: 1.0 - self.b,
        }
    }

    /// factor must be in range [0; 1]
    pub fn scale(&self, factor: f32) -> Rgb {
        assert!(
            (0.0..=1.0).contains(&factor),
            "scaling RGB color by {}. It should be in the range [0; 1]",
            factor
        );
        Self {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
        }
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Self::WHITE
    }
}

impl From<Rgb> for Color32 {
    fn from(val: Rgb) -> Self {
        Color32::from_rgb(
            (val.r * 255.0) as u8,
            (val.g * 255.0) as u8,
            (val.b * 255.0) as u8,
        )
    }
}

impl From<Rgb> for [f32; 3] {
    fn from(val: Rgb) -> Self {
        val.to_array()
    }
}
impl From<[f32; 3]> for Rgb {
    fn from(arr: [f32; 3]) -> Self {
        Rgb {
            r: arr[0],
            g: arr[1],
            b: arr[2],
        }
    }
}

#[allow(dead_code)]
impl Rgb {
    pub const BLACK: Rgb = Rgb {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const DARK_GRAY: Rgb = Rgb {
        r: 0.376471,
        g: 0.376471,
        b: 0.376471,
    };
    pub const GRAY: Rgb = Rgb {
        r: 0.627451,
        g: 0.627451,
        b: 0.627451,
    };
    pub const LIGHT_GRAY: Rgb = Rgb {
        r: 0.862745,
        g: 0.862745,
        b: 0.862745,
    };
    pub const WHITE: Rgb = Rgb {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    pub const BROWN: Rgb = Rgb {
        r: 0.647059,
        g: 0.164706,
        b: 0.164706,
    };
    pub const DARK_RED: Rgb = Rgb {
        r: 0.545098,
        g: 0.0,
        b: 0.0,
    };
    pub const RED: Rgb = Rgb {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };
    pub const LIGHT_RED: Rgb = Rgb {
        r: 1.0,
        g: 0.501961,
        b: 0.501961,
    };
    pub const YELLOW: Rgb = Rgb {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    };
    pub const LIGHT_YELLOW: Rgb = Rgb {
        r: 1.0,
        g: 1.0,
        b: 0.878431,
    };
    pub const KHAKI: Rgb = Rgb {
        r: 0.941176,
        g: 0.901961,
        b: 0.54902,
    };
    pub const DARK_GREEN: Rgb = Rgb {
        r: 0.0,
        g: 0.392157,
        b: 0.0,
    };
    pub const GREEN: Rgb = Rgb {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };
    pub const LIGHT_GREEN: Rgb = Rgb {
        r: 0.564706,
        g: 0.933333,
        b: 0.564706,
    };
    pub const DARK_BLUE: Rgb = Rgb {
        r: 0.0,
        g: 0.0,
        b: 0.545098,
    };
    pub const BLUE: Rgb = Rgb {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };
    pub const LIGHT_BLUE: Rgb = Rgb {
        r: 0.678431,
        g: 0.847059,
        b: 0.901961,
    };
    pub const GOLD: Rgb = Rgb {
        r: 1.0,
        g: 0.843137,
        b: 0.0,
    };
}
