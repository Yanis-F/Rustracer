use egui::{Color32, ColorImage, ImageData, TextureId, Ui, Vec2};

use crate::renderer::Renderer;
use crate::scene::Scene;

pub struct Rustracer {
    scene: Scene,
    renderer: Box<Renderer>,

    render_size_dragger_value: [usize; 2],
    visualizer_texture_id: Option<TextureId>,

    renderer_dirty: bool,
}

impl Rustracer {
    pub fn new() -> Self {
        let render_size = [1280, 720];

        Self {
            scene: Scene::default(),
            renderer: Box::new(Renderer::new(render_size, Scene::default())),
            render_size_dragger_value: render_size,
            visualizer_texture_id: None,
            renderer_dirty: false,
        }
    }
}

impl eframe::App for Rustracer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::panel::SidePanel::left("Editor").show(ctx, |ui| {
            self.display_editor(ctx, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.display_visualizer(ctx, ui);
        });

        if self.renderer_dirty {
            *self.renderer = Renderer::new(self.render_size_dragger_value, self.scene.clone());
            self.renderer_dirty = false;
        }
    }
}

impl Rustracer {
    fn display_editor(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        ui.heading("Rustracer");

        // CAMERA
        ui.heading("Camera");
        ui.horizontal(|ui| {
            ui.label("Position:");
            ui.label("x:");
            self.renderer_dirty |= ui
                .add(egui::DragValue::new(&mut self.scene.camera.position[0]).speed(0.1))
                .changed();
            ui.label("y:");
            self.renderer_dirty |= ui
                .add(egui::DragValue::new(&mut self.scene.camera.position[1]).speed(0.1))
                .changed();
            ui.label("z:");
            self.renderer_dirty |= ui
                .add(egui::DragValue::new(&mut self.scene.camera.position[2]).speed(0.1))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Facing direction:");
            ui.label(" for now it's locked at {tbd}");
        });

        // SCENE OBJECTS
        // todo
    }

    fn display_visualizer(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.label("Here is the visualizer");

        ui.horizontal(|ui| {
            ui.label("Render size:");
            ui.label("width:");
            self.renderer_dirty |= ui
                .add(
                    egui::DragValue::new(&mut self.render_size_dragger_value[0])
                        .speed(1)
                        .clamp_range(0..=1920),
                )
                .changed();
            ui.label("height:");
            self.renderer_dirty |= ui
                .add(
                    egui::DragValue::new(&mut self.render_size_dragger_value[1])
                        .speed(1)
                        .clamp_range(0..=1080),
                )
                .changed();
        });

        let texture_id = self.visualizer_texture_id.get_or_insert_with(|| {
            let image_data = ImageData::Color(ColorImage::new(
                self.renderer.get_image_size(),
                Color32::GRAY,
            ));
            ctx.tex_manager().write().alloc(
                "visualizer".to_owned(),
                image_data,
                egui::TextureOptions::LINEAR,
            )
        });

        // This will get fetched every frame

        let image = (*self.renderer).get_image();
        let image_data = ImageData::Color(ColorImage {
            size: self.renderer.get_image_size(),
            pixels: image,
        });

        ctx.tex_manager().write().set(
            *texture_id,
            eframe::epaint::ImageDelta::full(image_data, egui::TextureOptions::LINEAR),
        );
        ui.image(
            *texture_id,
            Vec2::new(
                self.renderer.get_image_size()[0] as f32,
                self.renderer.get_image_size()[1] as f32,
            ),
        );
        ctx.request_repaint();
    }
}

