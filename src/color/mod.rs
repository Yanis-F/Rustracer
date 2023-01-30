mod rgb;
pub use rgb::*;

mod rgba;
pub use rgba::*;

pub trait UiColorpickerExt {
    fn rustracer_color_edit_button_rgb(&mut self, rgb: &mut Rgb, label: &str) -> egui::Response;
    fn rustracer_color_edit_button_rgba(&mut self, rgba: &mut Rgba, label: &str) -> egui::Response;
}

impl UiColorpickerExt for egui::Ui {
    fn rustracer_color_edit_button_rgb(&mut self, rgb: &mut Rgb, label: &str) -> egui::Response {
        let ui = self;
        let mut arr = (*rgb).to_array();

        let inner_response = ui.horizontal(|ui| {
            vec![ui.label(label), ui.color_edit_button_rgb(&mut arr)]
                .into_iter()
                .reduce(|a, b| a.union(b))
                .unwrap()
        });
        let response = inner_response.response.union(inner_response.inner);
        *rgb = arr.into();

        response
    }

    fn rustracer_color_edit_button_rgba(&mut self, rgba: &mut Rgba, label: &str) -> egui::Response {
        let ui = self;
        let mut arr = (*rgba).to_array();

        let inner_response = ui.horizontal(|ui| {
            vec![
                ui.label(label),
                ui.color_edit_button_rgba_unmultiplied(&mut arr),
            ]
            .into_iter()
            .reduce(|a, b| a.union(b))
            .unwrap()
        });
        let response = inner_response.response.union(inner_response.inner);
        *rgba = arr.into();

        response
    }
}
