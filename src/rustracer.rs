use egui::{ColorImage, Color32, ImageData, Vec2};

use crate::scene::Scene;
use crate::renderer::Renderer;

#[derive(Default)]
pub struct Rustracer {
	scene: Scene,
	renderer: Renderer,
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

			// This will get fetched every frame
			let image_data: ColorImage = ColorImage::new([32, 32], Color32::BLUE);
			let image_data = ImageData::Color(image_data);

			// let texture_handle = ctx.load_texture("visualizer", image_data, );

			let future_image = image_data.clone();

			let texture_id = ctx.tex_manager().write().alloc("visualizer".to_owned(), image_data, egui::TextureOptions::LINEAR);
			ctx.tex_manager().write().set(texture_id, eframe::epaint::ImageDelta::full(future_image, egui::TextureOptions::LINEAR));

			ui.image(texture_id, Vec2::new(32.0, 32.0));
		});
	}
}
