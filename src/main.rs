mod app;
mod http;
use app::BrostmanApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Brostman", options, Box::new(|_cc| Box::new(BrostmanApp::default())))
}
