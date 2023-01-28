#[derive(Clone)]
pub enum SceneObject {
    Sphere(object_type::Sphere),
    // Cube
    // Plane
}

pub mod object_type {
	use crate::{math::*, scene::surface::Surface};

	#[derive(Clone)]
	pub struct Sphere {
		pub center: Vector3,
		pub radius: f64,
		pub surface: Surface
	}
}
