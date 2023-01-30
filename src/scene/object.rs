#[derive(Clone)]
pub enum SceneObject {
    Sphere(object_type::Sphere),
    Plane(object_type::Plane),
    // cube
}

pub mod object_type {
    use crate::{math::*, scene::surface::Surface};

    #[derive(Clone, Default)]
    pub struct Sphere {
        pub center: Vector3,
        pub radius: f64,
        pub surface: Surface,
    }
    #[derive(Clone)]
    pub struct Plane {
        pub point: Vector3,
        pub normal: Vector3,
        pub surface: Surface,
    }
    impl Default for Plane {
		fn default() -> Self {
		    Self { point: Default::default(), normal: [0.0, 1.0, 0.0], surface: Default::default() }
		}
	}
}
