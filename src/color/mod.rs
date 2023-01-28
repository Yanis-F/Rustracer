mod rgb;
pub use rgb::*;

mod rgba;
pub use rgba::*;

pub trait UiColorpickerExt {
    fn rustracer_color_edit_button_rgb(&mut self, rgb: &mut Rgb) -> egui::Response;
    fn rustracer_color_edit_button_rgba(&mut self, rgba: &mut Rgba) -> egui::Response;
}

impl UiColorpickerExt for egui::Ui {
    fn rustracer_color_edit_button_rgb(&mut self, rgb: &mut Rgb) -> egui::Response {
        let mut arr = (*rgb).to_array();
        let response = self.color_edit_button_rgb(&mut arr);
        *rgb = arr.into();

        response
    }

    fn rustracer_color_edit_button_rgba(&mut self, rgba: &mut Rgba) -> egui::Response {
        let mut arr = (*rgba).to_array();
        let response = self.color_edit_button_rgba_unmultiplied(&mut arr);
        *rgba = arr.into();

        response
    }
}
