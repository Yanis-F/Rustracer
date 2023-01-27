use crate::color::RGB;

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
