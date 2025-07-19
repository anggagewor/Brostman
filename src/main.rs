mod app;
mod http;
use app::KirimAjaApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("KirimAja", options, Box::new(|_cc| Box::new(KirimAjaApp::default())))
}
