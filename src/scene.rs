use crate::types::*;

#[derive(Default)]
pub struct Scene {
	pub camera: Camera,
}


#[derive(Default)]
pub struct Camera {
	pub position: Vec3,
	pub orientation: Quaternion,
}
