use egui::{Color32, ColorImage, ImageData, TextureId, Ui, Vec2};
use vecmath::vec3_normalized;

use crate::color::{Rgb, UiColorpickerExt};
use crate::egui_utils::UiUtilsExt;
use crate::math::{vec3, UiMathpickerExt};
use crate::renderer::Renderer;
use crate::scene::light::{
    light_type::{self, *},
    SceneLight,
};
use crate::scene::object::{
    object_type::{self, *},
    SceneObject,
};
use crate::scene::surface::{Surface, UiSurfaceEditExt};
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

        let mut default_scene = Scene::default();
        default_scene.camera.position = vec3(0.0, 1.0, 0.0);
        default_scene.ambiant = Rgb::BLACK;
        default_scene
            .lights
            .push(crate::scene::light::SceneLight::Directional(
                light_type::Directional {
                    color: crate::color::Rgb::WHITE,
                    direction: vec3(1.0, -1.0, 1.0),
                },
            ));

        default_scene
            .objects
            .push(crate::scene::object::SceneObject::Sphere(Sphere {
                center: vec3(0.0, 1.0, 2.0),
                radius: 0.5,
                surface: Surface::RUBY,
            }));

        default_scene
            .objects
            .push(crate::scene::object::SceneObject::Plane(
                object_type::Plane {
                    point: vec3(0.0, 0.0, 0.0),
                    normal: vec3(0.0, 1.0, 0.0),
                    surface: Surface::CYAN_PLASTIC,
                },
            ));

        Self {
            scene: default_scene.clone(),
            renderer: Box::new(Renderer::new(render_size, default_scene)),
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
        self.renderer_dirty |= ui
            .rustracer_vector3_edit(&mut self.scene.camera.position, "Position:", 0.1)
            .changed();
        self.renderer_dirty |= ui
            .rustracer_quaternion_edit(&mut self.scene.camera.orientation, "Facing direction:", 0.1)
            .changed();

        ui.add(egui::Separator::default().horizontal().spacing(50.0));

        ui.heading("Lights");
        ui.separator();
        self.renderer_dirty |= ui
            .rustracer_color_edit_button_rgb(&mut self.scene.ambiant, "Ambiant (one per scene):")
            .changed();
        ui.separator();

        {
            let mut light_vec = std::mem::take(&mut self.scene.lights);
            for light in &mut light_vec {
                self.display_light_editor(ui, light);
                ui.separator();
            }
            ui.label("Add light:");
            ui.horizontal_wrapped(|ui| {
                if ui.button("Directional light").clicked() {
                    light_vec.push(SceneLight::Directional(light_type::Directional::default()));
                    self.renderer_dirty = true;
                }
            });
            self.scene.lights = light_vec;
        }

        ui.add(egui::Separator::default().horizontal().spacing(50.0));

        ui.heading("Objects");
        ui.separator();
        {
            let mut object_vec = std::mem::take(&mut self.scene.objects);
            for object in &mut object_vec {
                self.display_object_editor(ui, object);
                ui.separator();
            }
            ui.label("Add object:");
            ui.horizontal_wrapped(|ui| {
                if ui.button("Sphere").clicked() {
                    object_vec.push(SceneObject::Sphere(object_type::Sphere::default()));
                    self.renderer_dirty = true;
                }
                if ui.button("Plane").clicked() {
                    object_vec.push(SceneObject::Plane(object_type::Plane::default()));
                    self.renderer_dirty = true;
                }
            });
            self.scene.objects = object_vec;
        }
    }

    fn display_light_editor(&mut self, ui: &mut Ui, light: &mut SceneLight) {
        match light {
            SceneLight::Directional(light) => self.display_light_editor_directional(ui, light),
        }
    }

    fn display_light_editor_directional(&mut self, ui: &mut Ui, light: &mut Directional) {
        ui.label("Directional light");
        self.renderer_dirty |= ui
            .rustracer_color_edit_button_rgb(&mut light.color, "Color:")
            .changed();
        self.renderer_dirty |= ui
            .rustracer_vector3_edit(&mut light.direction, "Direction:", 0.1)
            .changed();
    }

    fn display_object_editor(&mut self, ui: &mut Ui, object: &mut SceneObject) {
        match object {
            SceneObject::Sphere(sphere) => self.display_object_editor_sphere(ui, sphere),
            SceneObject::Plane(plane) => self.display_object_editor_plane(ui, plane),
        }
    }

    fn display_object_editor_sphere(&mut self, ui: &mut Ui, sphere: &mut Sphere) {
        ui.label("Sphere");
        self.renderer_dirty |= ui
            .rustracer_vector3_edit(&mut sphere.center, "Center:", 0.1)
            .changed();
        self.renderer_dirty |= ui.drag_value(&mut sphere.radius, "Radius:", 0.1, Some((0.0, 99999.0))).changed();
        self.renderer_dirty |= ui
            .rustracer_surface_edit(&mut sphere.surface, "Surface:")
            .changed();
    }

    fn display_object_editor_plane(&mut self, ui: &mut Ui, plane: &mut Plane) {
        ui.label("Plane");
        self.renderer_dirty |= ui
            .rustracer_vector3_edit(&mut plane.point, "Point:", 0.1)
            .changed();
        self.renderer_dirty |= ui
            .rustracer_vector3_edit(&mut plane.normal, "Normal:", 0.01)
            .changed();
        plane.normal = vec3_normalized(plane.normal);
        self.renderer_dirty |= ui
            .rustracer_surface_edit(&mut plane.surface, "Surface:")
            .changed();
    }

    fn display_visualizer(&mut self, ctx: &egui::Context, ui: &mut Ui) {
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
