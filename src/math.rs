#![allow(dead_code)]

pub use quaternion::*;
pub use vecmath::*;

use crate::egui_utils::UiUtilsExt;

pub type Vector2 = vecmath::Vector2<f64>;
pub fn vec2(x: f64, y: f64) -> Vector2 {
    [x, y]
}

// TODO: check if I can't do that as Vector2::UP by using an extension trait to provide a custom
// `impl` block
pub const VECTOR2_UP: Vector2 = [0.0, -1.0];
pub const VECTOR2_RIGHT: Vector2 = [1.0, 0.0];
pub const VECTOR2_ZERO: Vector2 = [0.0, 0.0];
pub const VECTOR2_ONE: Vector2 = [1.0, 1.0];

pub type Vector3 = vecmath::Vector3<f64>;
pub fn vec3(x: f64, y: f64, z: f64) -> Vector3 {
    [x, y, z]
}

pub const VECTOR3_UP: Vector3 = [0.0, 1.0, 0.0];
pub const VECTOR3_RIGHT: Vector3 = [1.0, 0.0, 0.0];
pub const VECTOR3_FORWARD: Vector3 = [0.0, 0.0, 1.0];
pub const VECTOR3_ZERO: Vector3 = [0.0, 0.0, 0.0];
pub const VECTOR3_ONE: Vector3 = [1.0, 1.0, 1.0];

pub type Quaternion = quaternion::Quaternion<f64>;

pub struct Ray {
    pub origin: Vector3,
    /// The user is responsible of keeping this vector normal
    pub direction: Vector3,
}

impl Ray {
    pub fn get_position_at_distance(&self, distance: f64) -> Vector3 {
        vec3_add(self.origin, vec3_scale(self.direction, distance))
    }
}

/// Calculate the reflection direction for an incident vector
///
/// For a given incident vector I and surface normal N reflect returns the reflection direction
/// calculated as I - 2.0 * dot(N, I) * N.
///
/// N should be normalized in order to achieve the desired result.
pub fn vec3_reflect(incident: Vector3, normal: Vector3) -> Vector3 {
    assert!(
        (vec3_len(normal) - 1.0).abs() < 1e-6,
        "normal vector should be normalized"
    );

    let dot = vec3_dot(incident, normal);

    vec3_sub(incident, vec3_scale(normal, dot * 2.0))
}

pub trait UiMathpickerExt {
    fn rustracer_vector3_edit(
        &mut self,
        vector: &mut Vector3,
        label: &str,
        speed: f64,
    ) -> egui::Response;
    fn rustracer_quaternion_edit(
        &mut self,
        quaternion: &mut Quaternion,
        label: &str,
        speed: f64,
    ) -> egui::Response;
}

impl UiMathpickerExt for egui::Ui {
    fn rustracer_vector3_edit(
        &mut self,
        vector: &mut Vector3,
        label: &str,
        speed: f64,
    ) -> egui::Response {
        let ui = self;

        ui.horizontal_response_union(|ui| {
            vec![
                ui.label(label),
                ui.label("x:"),
                ui.add(egui::DragValue::new(&mut vector[0]).speed(speed)),
                ui.label("y:"),
                ui.add(egui::DragValue::new(&mut vector[1]).speed(speed)),
                ui.label("z:"),
                ui.add(egui::DragValue::new(&mut vector[2]).speed(speed)),
            ]
        })
    }

    fn rustracer_quaternion_edit(
        &mut self,
        quaternion: &mut Quaternion,
        label: &str,
        speed: f64,
    ) -> egui::Response {
        // TODO: Unity-like Euler angles

        let mut facing_direction = rotate_vector(*quaternion, VECTOR3_FORWARD);

        let response = self.rustracer_vector3_edit(&mut facing_direction, label, speed);

        *quaternion = rotation_from_to(VECTOR3_FORWARD, facing_direction);

        response
    }
}

// For the orientation picker:
