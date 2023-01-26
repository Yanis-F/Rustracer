use eframe::epaint::ImageDelta;
use egui::{ColorImage, Color32, ImageData, Vec2, TextureId};

use crate::scene::Scene;
use crate::renderer::Renderer;

pub struct Rustracer {
	scene: Scene,
	renderer: Box<Renderer>,

	render_size: [usize; 2],
	visualizer_texture_id: Option<TextureId>,
}

impl Rustracer {
	pub fn new() -> Self {
		let render_size = [64, 64];

		Self {
			scene: Scene::default(),
			renderer: Box::new(Renderer::new(render_size, Scene::default())),
			render_size,
			visualizer_texture_id: None,
		}
	}
}

impl eframe::App for Rustracer {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::panel::SidePanel::left("Editor").show(ctx, |ui| {
			ui.heading("Rustracer");

			// CAMERA
			ui.heading("Camera");
			ui.horizontal(|ui| {
				ui.label("Position:");
				ui.label("x:");
				ui.add(egui::DragValue::new(&mut self.scene.camera.position.x).speed(0.1));
				ui.label("y:");
				ui.add(egui::DragValue::new(&mut self.scene.camera.position.y).speed(0.1));
				ui.label("z:");
				ui.add(egui::DragValue::new(&mut self.scene.camera.position.z).speed(0.1));
			});
			ui.horizontal(|ui| {
				ui.label("Facing direction:");
				ui.label(" for now it's locked at {tbd}");
			});


			// SCENE OBJECTS
			// todo
		});


		egui::CentralPanel::default().show(ctx, |ui| {
			ui.label("Here is the visualizer");

			ui.horizontal(|ui| {
				ui.label("Render size:");
				// ui.add(egui::DragValue::new(&mut self.render_size.x).speed(1).prefix("width:").clamp_range(0..=1920));
				// ui.add(egui::DragValue::new(&mut self.render_size.y).speed(1).prefix("height:").clamp_range(0..=1080));
			});

			let texture_id = self.visualizer_texture_id.get_or_insert_with(||{
				let image_data = ImageData::Color(ColorImage::new(self.render_size, Color32::GRAY));
				ctx.tex_manager().write().alloc("visualizer".to_owned(), image_data, egui::TextureOptions::LINEAR)
			});

			// This will get fetched every frame

			let image = (*self.renderer).get_image();
			let image_data = ImageData::Color(ColorImage { size: self.render_size, pixels: image });

			ctx.tex_manager().write().set(*texture_id, eframe::epaint::ImageDelta::full(image_data, egui::TextureOptions::LINEAR));
			ui.image(*texture_id, Vec2::new(self.render_size[0] as f32, self.render_size[1] as f32));
		});
	}
}
