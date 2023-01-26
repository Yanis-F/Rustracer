use eframe::egui;

mod types;
mod rustracer;
mod scene;
mod renderer;

fn main() {
	// Log to stdout (if you run with `RUST_LOG=debug`).
	tracing_subscriber::fmt::init();

	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(1600.0, 900.0)),
		..Default::default()
	};
	
	eframe::run_native(
		"My egui App",
		options,
		Box::new(|_cc| Box::new(rustracer::Rustracer::default())),
	);
}
