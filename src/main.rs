#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod util;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960., 540.]),
        ..Default::default()
    };
    eframe::run_native(
        "iRacing Overlays",
        options,
        Box::new(|_cc| Ok(Box::new(app::MainApp::new()))),
    )
}
