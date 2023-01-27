use crate::math::*;


use super::surface::Surface;

#[derive(Clone)]
pub enum SceneObject {
    Sphere(Sphere),
    // Cube
    // Plane
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub surface: Surface
}
