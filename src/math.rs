#![allow(dead_code)]

pub use quaternion::*;
pub use vecmath::*;

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

pub trait UiVectorpickerExt {
    fn rustracer_vector3_edit(&mut self, vector: &mut Vector3, speed: f64) -> egui::Response;
}

impl UiVectorpickerExt for egui::Ui {
    fn rustracer_vector3_edit(&mut self, vector: &mut Vector3, speed: f64) -> egui::Response {
        let inner_response = self.horizontal(|ui| {
            let first_response = ui.label("x:");
            vec![
                ui.add(egui::DragValue::new(&mut vector[0]).speed(speed)),
                ui.label("y:"),
                ui.add(egui::DragValue::new(&mut vector[1]).speed(speed)),
                ui.label("z:"),
                ui.add(egui::DragValue::new(&mut vector[2]).speed(speed)),
            ]
            .iter()
            .fold(first_response, |a, b| b.union(a))
        });

        inner_response.inner.union(inner_response.response)
    }
}

// For the orientation picker:
// ui.drag_angle(radians)
