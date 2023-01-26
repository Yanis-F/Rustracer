use crate::scene::Scene;

#[derive(Default)]
pub struct Rustracer {
	scene: Scene
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
		});


		egui::CentralPanel::default().show(ctx, |ui| {
			ui.label("Here is the visualizer");
		});
	}
}
