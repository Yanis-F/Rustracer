use egui::Color32;

use crate::scene::Scene;

#[derive(Default)]
pub struct Renderer {
	size: [usize; 2],
	scene: Scene,

	data: Vec<Color32>
}

impl Renderer {
	pub fn new(size: [usize; 2], scene: Scene) -> Renderer {
		Renderer { 
			size,
			scene,
			data: vec![Color32::GRAY; size[0] * size[1]]
		}
	}

	/// Returns an RGB image of `self.size` size
	pub fn get_image(&self) -> Vec<Color32> {
		self.data.clone()
	}
}


