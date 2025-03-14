mod app;
mod utils;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "EGUI App",
        options,
        Box::new(|_cc| Box::new(app::MyApp::default())),
    )
}