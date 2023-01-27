use egui::Color32;

use crate::{scene::Scene, math::*};



pub fn raytracer(scene: &Scene, ray: &Ray) -> Color32 {
	let c = vec3_scale(ray.direction, 127.0);
	let c = vec3_add(c, vec3(127.0, 127.0, 127.0));
	println!("ray is {},{},{}, color is {},{},{}", ray.direction[0], ray.direction[1], ray.direction[2], c[0], c[1], c[2]);
	Color32::from_rgb(c[0] as u8, c[1] as u8, c[2] as u8)
}
