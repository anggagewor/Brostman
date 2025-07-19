use eframe::egui::{self, ComboBox, TopBottomPanel, menu, Context, ViewportCommand};
use crate::http::{HttpResponse};
use serde_json::{Value as JsonValue};

#[derive(PartialEq, Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
        }
    }

    pub fn all() -> Vec<HttpMethod> {
        vec![Self::GET, Self::POST, Self::PUT, Self::DELETE, Self::PATCH]
    }
}

pub struct BrostmanApp {
    pub url: String,
    pub method: HttpMethod,
    pub response: String,
    pub show_about: bool,
}

impl Default for BrostmanApp {
    fn default() -> Self {
        Self {
            url: String::from("https://jsonplaceholder.typicode.com/posts/1"),
            method: HttpMethod::GET,
            response: String::new(),
            show_about: false,
        }
    }
}

impl eframe::App for BrostmanApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // ⛳ Menu Bar
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close); // ✅ ganti ini
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About Brostman").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });

        // 🪪 About Dialog
        if self.show_about {
            egui::Window::new("About Brostman")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("🚀 Brostman");
                    ui.label("Versi belajar Rust dengan studi kasus mirip Postman.");
                    ui.label("Dibuat oleh Angga Purnama ❤️");
                    if ui.button("Close").clicked() {
                        self.show_about = false;
                    }
                });
        }

        // 🧱 Panel utama
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ComboBox::from_label("Method")
                    .selected_text(self.method.as_str())
                    .show_ui(ui, |ui| {
                        for m in HttpMethod::all() {
                            ui.selectable_value(&mut self.method, m, m.as_str());
                        }
                    });

                ui.text_edit_singleline(&mut self.url);

                if ui.button("Send").clicked() {
                    let response: HttpResponse = crate::http::send_request(&self.url, self.method.as_str());

                    // Cek konten dan format kalo json
                    if let Some(ct) = response.content_type {
                        if ct.contains("application/json") {
                            if let Ok(json) = serde_json::from_str::<JsonValue>(&response.body) {
                                if let Ok(pretty) = serde_json::to_string_pretty(&json) {
                                    self.response = format!("Status: {}\nContent-Type: {}\n\n{}", response.status, ct, pretty);
                                    return;
                                }
                            }
                        }
                        // fallback non-JSON
                        self.response = format!("Status: {}\nContent-Type: {}\n\n{}", response.status, ct, response.body);
                    } else {
                        self.response = format!("Status: {}\n\n{}", response.status, response.body);
                    }
                }
            });

            ui.separator();
            ui.label("Response:");
            ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.response).desired_rows(30));
        });
    }
}
