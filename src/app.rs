use eframe::egui;

pub struct BrostmanApp {
    pub url: String,
    pub method: String,
    pub response: String,
}

impl Default for BrostmanApp {
    fn default() -> Self {
        Self {
            url: String::from("https://jsonplaceholder.typicode.com/posts/1"),
            method: "GET".to_string(),
            response: "".to_string(),
        }
    }
}

impl eframe::App for BrostmanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.url);
                ui.text_edit_singleline(&mut self.method);
                if ui.button("Send").clicked() {
                    self.response = crate::http::send_request(&self.url, &self.method);
                }
            });

            ui.label("Response:");
            ui.text_edit_multiline(&mut self.response);
        });
    }
}
