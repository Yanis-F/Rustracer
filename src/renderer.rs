use egui::Color32;

use crate::scene::Scene;
use rand::Rng;

#[derive(Default)]
pub struct Renderer {
    size: [usize; 2],
    scene: Scene,

    data: Vec<Color32>,
}

impl Renderer {
    pub fn new(size: [usize; 2], scene: Scene) -> Renderer {
        Renderer {
            size,
            scene,
            data: vec![Color32::GRAY; size[0] * size[1]],
        }
    }

    /// Returns an RGB image of `self.size` size
    pub fn get_image(&self) -> Vec<Color32> {
        let size = self.size[0] * self.size[1];
        let mut result = Vec::with_capacity(size);

        let mut rng = rand::thread_rng();

        for _ in 0..size {
            result.push(Color32::from_rgb(rng.gen(), rng.gen(), rng.gen()));
        }

        result
    }
}
