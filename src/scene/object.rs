use crate::math::*;
use crate::raytracer::raycast::RaycastHit;

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
}
