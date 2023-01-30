use egui::Ui;

pub trait UiUtilsExt {
    fn horizontal_response_union(
        &mut self,
        add_contents: impl FnOnce(&mut Ui) -> Vec<egui::Response>,
    ) -> egui::Response;
    fn drag_value(&mut self, value: &mut f64, label: &str, speed: f64) -> egui::Response;
}

impl UiUtilsExt for Ui {
    fn horizontal_response_union(
        &mut self,
        add_contents: impl FnOnce(&mut Ui) -> Vec<egui::Response>,
    ) -> egui::Response {
        let ui = self;

        let inner_response = ui.horizontal(|ui| {
            let vec = add_contents(ui);
            assert!(
                !vec.is_empty(),
                "Horizontal block returned no response. Please put at least one element"
            );

            vec.into_iter().reduce(|a, b| a.union(b)).unwrap()
        });

        inner_response.response.union(inner_response.inner)
    }

    fn drag_value(&mut self, value: &mut f64, label: &str, speed: f64) -> egui::Response {
        let ui = self;
        vec![
            ui.label(label),
            ui.add(egui::DragValue::new(value).speed(speed)),
        ]
        .into_iter()
        .reduce(|a, b| a.union(b))
        .unwrap()
    }
}
