use crate::{
    color::{Rgb, UiColorpickerExt},
    egui_utils::UiUtilsExt,
};

#[derive(Clone, Default)]
pub struct Surface {
    ///  what color the surface reflects under ambient lighting
    /// this is usually the same as the surface's color
    pub ambiant: Rgb,

    /// The diffuse material vector defines the color of the surface under diffuse
    /// lighting. The diffuse color is (just like ambient lighting) set to the desired surface's color
    pub diffuse: Rgb,

    // The specular material vector sets the color of the specular highlight on
    // the surface (or possibly even reflect a surface-specific color).
    pub specular: Rgb,

    // shininess impacts the scattering/radius of the specular highlight.
    pub shininess: f64,
    // Transparency (affecting light color when passing through? diffuse color ?)
    // Reflection
}

#[allow(dead_code)]
impl Surface {
    // source : http://devernay.free.fr/cours/opengl/materials.html
    pub const EMERALD: Surface = Surface {
        ambiant: Rgb {
            r: 0.0215,
            g: 0.1745,
            b: 0.0215,
        },
        diffuse: Rgb {
            r: 0.07568,
            g: 0.61424,
            b: 0.07568,
        },
        specular: Rgb {
            r: 0.633,
            g: 0.727811,
            b: 0.633,
        },
        shininess: 0.6,
    };
    pub const JADE: Surface = Surface {
        ambiant: Rgb {
            r: 0.135,
            g: 0.2225,
            b: 0.1575,
        },
        diffuse: Rgb {
            r: 0.54,
            g: 0.89,
            b: 0.63,
        },
        specular: Rgb {
            r: 0.316228,
            g: 0.316228,
            b: 0.316228,
        },
        shininess: 0.1,
    };
    pub const OBSIDIAN: Surface = Surface {
        ambiant: Rgb {
            r: 0.05375,
            g: 0.05,
            b: 0.06625,
        },
        diffuse: Rgb {
            r: 0.18275,
            g: 0.17,
            b: 0.22525,
        },
        specular: Rgb {
            r: 0.332741,
            g: 0.328634,
            b: 0.346435,
        },
        shininess: 0.3,
    };
    pub const PEARL: Surface = Surface {
        ambiant: Rgb {
            r: 0.25,
            g: 0.20725,
            b: 0.20725,
        },
        diffuse: Rgb {
            r: 1.0,
            g: 0.829,
            b: 0.829,
        },
        specular: Rgb {
            r: 0.296648,
            g: 0.296648,
            b: 0.296648,
        },
        shininess: 0.088,
    };
    pub const RUBY: Surface = Surface {
        ambiant: Rgb {
            r: 0.1745,
            g: 0.01175,
            b: 0.01175,
        },
        diffuse: Rgb {
            r: 0.61424,
            g: 0.04136,
            b: 0.04136,
        },
        specular: Rgb {
            r: 0.727811,
            g: 0.626959,
            b: 0.626959,
        },
        shininess: 0.6,
    };
    pub const TURQUOISE: Surface = Surface {
        ambiant: Rgb {
            r: 0.1,
            g: 0.18725,
            b: 0.1745,
        },
        diffuse: Rgb {
            r: 0.396,
            g: 0.74151,
            b: 0.69102,
        },
        specular: Rgb {
            r: 0.297254,
            g: 0.30829,
            b: 0.306678,
        },
        shininess: 0.1,
    };
    pub const BRASS: Surface = Surface {
        ambiant: Rgb {
            r: 0.329412,
            g: 0.223529,
            b: 0.027451,
        },
        diffuse: Rgb {
            r: 0.780392,
            g: 0.568627,
            b: 0.113725,
        },
        specular: Rgb {
            r: 0.992157,
            g: 0.941176,
            b: 0.807843,
        },
        shininess: 0.21794872,
    };
    pub const BRONZE: Surface = Surface {
        ambiant: Rgb {
            r: 0.2125,
            g: 0.1275,
            b: 0.054,
        },
        diffuse: Rgb {
            r: 0.714,
            g: 0.4284,
            b: 0.18144,
        },
        specular: Rgb {
            r: 0.393548,
            g: 0.271906,
            b: 0.166721,
        },
        shininess: 0.2,
    };
    pub const CHROME: Surface = Surface {
        ambiant: Rgb {
            r: 0.25,
            g: 0.25,
            b: 0.25,
        },
        diffuse: Rgb {
            r: 0.4,
            g: 0.4,
            b: 0.4,
        },
        specular: Rgb {
            r: 0.774597,
            g: 0.774597,
            b: 0.774597,
        },
        shininess: 0.6,
    };
    pub const COPPER: Surface = Surface {
        ambiant: Rgb {
            r: 0.19125,
            g: 0.0735,
            b: 0.0225,
        },
        diffuse: Rgb {
            r: 0.7038,
            g: 0.27048,
            b: 0.0828,
        },
        specular: Rgb {
            r: 0.256777,
            g: 0.137622,
            b: 0.086014,
        },
        shininess: 0.1,
    };
    pub const GOLD: Surface = Surface {
        ambiant: Rgb {
            r: 0.24725,
            g: 0.1995,
            b: 0.0745,
        },
        diffuse: Rgb {
            r: 0.75164,
            g: 0.60648,
            b: 0.22648,
        },
        specular: Rgb {
            r: 0.628281,
            g: 0.555802,
            b: 0.366065,
        },
        shininess: 0.4,
    };
    pub const SILVER: Surface = Surface {
        ambiant: Rgb {
            r: 0.19225,
            g: 0.19225,
            b: 0.19225,
        },
        diffuse: Rgb {
            r: 0.50754,
            g: 0.50754,
            b: 0.50754,
        },
        specular: Rgb {
            r: 0.508273,
            g: 0.508273,
            b: 0.508273,
        },
        shininess: 0.4,
    };
    pub const BLACK_PLASTIC: Surface = Surface {
        ambiant: Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        },
        diffuse: Rgb {
            r: 0.01,
            g: 0.01,
            b: 0.01,
        },
        specular: Rgb {
            r: 0.50,
            g: 0.50,
            b: 0.50,
        },
        shininess: 0.25,
    };
    pub const CYAN_PLASTIC: Surface = Surface {
        ambiant: Rgb {
            r: 0.0,
            g: 0.1,
            b: 0.06,
        },
        diffuse: Rgb {
            r: 0.0,
            g: 0.5098039,
            b: 0.5098039,
        },
        specular: Rgb {
            r: 0.50196078,
            g: 0.50196078,
            b: 0.50196078,
        },
        shininess: 0.25,
    };
    pub const GREEN_PLASTIC: Surface = Surface {
        ambiant: Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        },
        diffuse: Rgb {
            r: 0.1,
            g: 0.35,
            b: 0.1,
        },
        specular: Rgb {
            r: 0.45,
            g: 0.55,
            b: 0.45,
        },
        shininess: 0.25,
    };
    pub const RED_PLASTIC: Surface = Surface {
        ambiant: Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        },
        diffuse: Rgb {
            r: 0.5,
            g: 0.0,
            b: 0.0,
        },
        specular: Rgb {
            r: 0.7,
            g: 0.6,
            b: 0.6,
        },
        shininess: 0.25,
    };
    pub const WHITE_PLASTIC: Surface = Surface {
        ambiant: Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        },
        diffuse: Rgb {
            r: 0.55,
            g: 0.55,
            b: 0.55,
        },
        specular: Rgb {
            r: 0.70,
            g: 0.70,
            b: 0.70,
        },
        shininess: 0.25,
    };
    pub const YELLOW_PLASTIC: Surface = Surface {
        ambiant: Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        },
        diffuse: Rgb {
            r: 0.5,
            g: 0.5,
            b: 0.0,
        },
        specular: Rgb {
            r: 0.60,
            g: 0.60,
            b: 0.50,
        },
        shininess: 0.25,
    };
    pub const BLACK_RUBBER: Surface = Surface {
        ambiant: Rgb {
            r: 0.02,
            g: 0.02,
            b: 0.02,
        },
        diffuse: Rgb {
            r: 0.01,
            g: 0.01,
            b: 0.01,
        },
        specular: Rgb {
            r: 0.4,
            g: 0.4,
            b: 0.4,
        },
        shininess: 0.078125,
    };
    pub const CYAN_RUBBER: Surface = Surface {
        ambiant: Rgb {
            r: 0.0,
            g: 0.05,
            b: 0.05,
        },
        diffuse: Rgb {
            r: 0.4,
            g: 0.5,
            b: 0.5,
        },
        specular: Rgb {
            r: 0.04,
            g: 0.7,
            b: 0.7,
        },
        shininess: 0.078125,
    };
    pub const GREEN_RUBBER: Surface = Surface {
        ambiant: Rgb {
            r: 0.0,
            g: 0.05,
            b: 0.0,
        },
        diffuse: Rgb {
            r: 0.4,
            g: 0.5,
            b: 0.4,
        },
        specular: Rgb {
            r: 0.04,
            g: 0.7,
            b: 0.04,
        },
        shininess: 0.078125,
    };
    pub const RED_RUBBER: Surface = Surface {
        ambiant: Rgb {
            r: 0.05,
            g: 0.0,
            b: 0.0,
        },
        diffuse: Rgb {
            r: 0.5,
            g: 0.4,
            b: 0.4,
        },
        specular: Rgb {
            r: 0.7,
            g: 0.04,
            b: 0.04,
        },
        shininess: 0.078125,
    };
    pub const WHITE_RUBBER: Surface = Surface {
        ambiant: Rgb {
            r: 0.05,
            g: 0.05,
            b: 0.05,
        },
        diffuse: Rgb {
            r: 0.5,
            g: 0.5,
            b: 0.5,
        },
        specular: Rgb {
            r: 0.7,
            g: 0.7,
            b: 0.7,
        },
        shininess: 0.078125,
    };
    pub const YELLOW_RUBBER: Surface = Surface {
        ambiant: Rgb {
            r: 0.05,
            g: 0.05,
            b: 0.0,
        },
        diffuse: Rgb {
            r: 0.5,
            g: 0.5,
            b: 0.4,
        },
        specular: Rgb {
            r: 0.7,
            g: 0.7,
            b: 0.04,
        },
        shininess: 0.078125,
    };
}

pub trait UiSurfaceEditExt {
    fn rustracer_surface_edit(&mut self, surface: &mut Surface, label: &str) -> egui::Response;
}

impl UiSurfaceEditExt for egui::Ui {
    fn rustracer_surface_edit(&mut self, surface: &mut Surface, label: &str) -> egui::Response {
        let ui = self;

        // TODO: add preset picker

        vec![
            ui.label(label),
            ui.rustracer_color_edit_button_rgb(&mut surface.ambiant, "Ambiant color:"),
            ui.rustracer_color_edit_button_rgb(&mut surface.diffuse, "Diffuse color:"),
            ui.rustracer_color_edit_button_rgb(&mut surface.specular, "Specular color:"),
            ui.drag_value(&mut surface.shininess, "Shininess:", 0.01, Some((0.0, 1.0))),
        ]
        .into_iter()
        .reduce(|a, b| a.union(b))
        .unwrap()
    }
}
