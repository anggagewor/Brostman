mod brostman_app;
mod http;
use eframe::{self, Theme};


fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        follow_system_theme: true,
        default_theme: Theme::Light,
        ..Default::default()
    };

    eframe::run_native(
        "Brostman",
        native_options,
        Box::new(|_cc| Box::new(brostman_app::BrostmanApp::default())),
    )
}
