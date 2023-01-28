
#[derive(Clone)]
pub enum SceneLight {
    Directional(light_type::Directional)
    // point
}

pub mod light_type {

	use crate::math::*;
	use crate::color::*;

	#[derive(Clone)]
	pub struct Directional {
		pub color: RGBA,
		/// /!\ not normalized
		pub direction: Vector3
	}
	impl Default for Directional {
		fn default() -> Self {
		    Self {
				color: RGBA::default(),
				direction: vec3_neg(VECTOR3_UP)
			}
		}
	}

}
