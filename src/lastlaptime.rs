#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, CentralPanel, Context};
use eframe::{App, NativeOptions};
mod telemetry;

struct LastLapTimeApp {
    local_telem: telemetry::IRacingLogging,
}

impl LastLapTimeApp {
    fn new() -> Self {
        Self {
            local_telem: telemetry::IRacingLogging::new(),
        }
    }
    fn update_telemetry(&mut self) {
        telemetry::IRacingLogging::update_telemetry(&mut self.local_telem)
    }
}

impl App for LastLapTimeApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.update_telemetry();
        CentralPanel::default().show(ctx, |ui| {
            ui.label(format!(
                "Last lap: {} seconds",
                self.local_telem.telemetry.lastlaptime
            ));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_always_on_top().with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "Last Lap Time",
        options,
        Box::new(|_cc| Ok(Box::new(LastLapTimeApp::new()))),
    )
}
