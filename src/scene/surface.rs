use crate::color::RGB;

#[derive(Clone)]
pub struct Surface {
    ///  what color the surface reflects under ambient lighting
    /// this is usually the same as the surface's color
    pub ambiant: RGB,
    // Diffuse color RGB - The diffuse material vector defines the color of the surface under diffuse
    // lighting. The diffuse color is (just like ambient lighting) set to the desired surface's color
    //
    // Specular color RGB - The specular material vector sets the color of the specular highlight on
    // the surface (or possibly even reflect a surface-specific color).
    //
    // shininess -  shininess impacts the scattering/radius of the specular highlight.
    //
    // Transparency (affecting light color when passing through?)
    //
    // Reflection
}

#[allow(dead_code)]
impl Surface {
	// source : http://devernay.free.fr/cours/opengl/materials.html
	// format : name | ambiant ambiant ambiant | diffuse diffuse diffuse | specular specular specular | shininess
	pub const EMERALD       : Surface = Surface { ambiant: RGB { r: 0.0215  , g: 0.1745  , b: 0.0215 } }; //	0.07568	0.61424	0.07568	0.633	0.727811	0.633	0.6
	pub const JADE          : Surface = Surface { ambiant: RGB { r: 0.135   , g: 0.2225  , b: 0.1575 } }; // 0.54	0.89	0.63	0.316228	0.316228	0.316228	0.1
	pub const OBSIDIAN      : Surface = Surface { ambiant: RGB { r: 0.05375 , g: 0.05    , b: 0.06625 } }; // 0.18275	0.17	0.22525	0.332741	0.328634	0.346435	0.3
	pub const PEARL         : Surface = Surface { ambiant: RGB { r: 0.25    , g: 0.20725 , b: 0.20725 } }; // 1	0.829	0.829	0.296648	0.296648	0.296648	0.088
	pub const RUBY          : Surface = Surface { ambiant: RGB { r: 0.1745  , g: 0.01175 , b: 0.01175 } }; // 0.61424	0.04136	0.04136	0.727811	0.626959	0.626959	0.6
	pub const TURQUOISE     : Surface = Surface { ambiant: RGB { r: 0.1     , g: 0.18725 , b: 0.1745 } }; // 0.396	0.74151	0.69102	0.297254	0.30829	0.306678	0.1
	pub const BRASS         : Surface = Surface { ambiant: RGB { r: 0.329412, g: 0.223529, b: 0.027451 } }; // 0.780392	0.568627	0.113725	0.992157	0.941176	0.807843	0.21794872
	pub const BRONZE        : Surface = Surface { ambiant: RGB { r: 0.2125  , g: 0.1275  , b: 0.054 } }; // 0.714	0.4284	0.18144	0.393548	0.271906	0.166721	0.2
	pub const CHROME        : Surface = Surface { ambiant: RGB { r: 0.25    , g: 0.25    , b: 0.25 } }; // 0.4	0.4	0.4	0.774597	0.774597	0.774597	0.6
	pub const COPPER        : Surface = Surface { ambiant: RGB { r: 0.19125 , g: 0.0735  , b: 0.0225 } }; // 0.7038	0.27048	0.0828	0.256777	0.137622	0.086014	0.1
	pub const GOLD          : Surface = Surface { ambiant: RGB { r: 0.24725 , g: 0.1995  , b: 0.0745 } }; // 0.75164	0.60648	0.22648	0.628281	0.555802	0.366065	0.4
	pub const SILVER        : Surface = Surface { ambiant: RGB { r: 0.19225 , g: 0.19225 , b: 0.19225 } }; // 0.50754	0.50754	0.50754	0.508273	0.508273	0.508273	0.4
	pub const BLACK_PLASTIC : Surface = Surface { ambiant: RGB { r: 0.0     , g: 0.0     , b: 0.0 } }; // 0.01	0.01	0.01	0.50	0.50	0.50	.25
	pub const CYAN_PLASTIC  : Surface = Surface { ambiant: RGB { r: 0.0     , g: 0.1     , b: 0.06 } }; // 0.0	0.50980392	0.50980392	0.50196078	0.50196078	0.50196078	.25
	pub const GREEN_PLASTIC : Surface = Surface { ambiant: RGB { r: 0.0     , g: 0.0     , b: 0.0 } }; // 0.1	0.35	0.1	0.45	0.55	0.45	.25
	pub const RED_PLASTIC   : Surface = Surface { ambiant: RGB { r: 0.0     , g: 0.0     , b: 0.0 } }; // 0.5	0.0	0.0	0.7	0.6	0.6	.25
	pub const WHITE_PLASTIC : Surface = Surface { ambiant: RGB { r: 0.0     , g: 0.0     , b: 0.0 } }; // 0.55	0.55	0.55	0.70	0.70	0.70	.25
	pub const YELLOW_PLASTIC: Surface = Surface { ambiant: RGB { r: 0.0     , g: 0.0     , b: 0.0 } }; // 0.5	0.5	0.0	0.60	0.60	0.50	.25
	pub const BLACK_RUBBER  : Surface = Surface { ambiant: RGB { r: 0.02    , g: 0.02    , b: 0.02 } }; // 0.01	0.01	0.01	0.4	0.4	0.4	.078125
	pub const CYAN_RUBBER   : Surface = Surface { ambiant: RGB { r: 0.0     , g: 0.05    , b: 0.05 } }; // 0.4	0.5	0.5	0.04	0.7	0.7	.078125
	pub const GREEN_RUBBER  : Surface = Surface { ambiant: RGB { r: 0.0     , g: 0.05    , b: 0.0 } }; // 0.4	0.5	0.4	0.04	0.7	0.04	.078125
	pub const RED_RUBBER    : Surface = Surface { ambiant: RGB { r: 0.05    , g: 0.0     , b: 0.0 } }; // 0.5	0.4	0.4	0.7	0.04	0.04	.078125
	pub const WHITE_RUBBER  : Surface = Surface { ambiant: RGB { r: 0.05    , g: 0.05    , b: 0.05 } }; // 0.5	0.5	0.5	0.7	0.7	0.7	.078125
	pub const YELLOW_RUBBER : Surface = Surface { ambiant: RGB { r: 0.05    , g: 0.05    , b: 0.0 } }; // 0.5	0.5	0.4	0.7	0.7	0.04	.078125
}
