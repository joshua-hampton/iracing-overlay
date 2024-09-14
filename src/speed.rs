#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, CentralPanel, Context};
use eframe::{App, NativeOptions};
use util::SpeedUnits;
mod telemetry;
mod util;

struct SpeedApp {
    local_telem: telemetry::IRacingLogging,
    units: util::SpeedUnits,
}

impl SpeedApp {
    fn new() -> Self {
        let config: util::WindowsConfig = confy::load("iracing-overlays", None).unwrap_or_default();
        let units: util::SpeedUnits = config.speed_config.units;
        Self {
            local_telem: telemetry::IRacingLogging::new(),
            units,
        }
    }
    fn update_telemetry(&mut self) {
        telemetry::IRacingLogging::update_telemetry(&mut self.local_telem)
    }
}

impl App for SpeedApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.update_telemetry();
        CentralPanel::default().show(ctx, |ui| {
            let (unitstring, multiplier) = match self.units {
                SpeedUnits::Metrespersecond => ("m/s", 1.),
                SpeedUnits::Milesperhour => ("mph", 3600./1609.),
                SpeedUnits::Kilometresperhour => ("kph", 3.6),
            };
            ui.label(format!(
                "{:.0} {}",
                self.local_telem.telemetry.speed * multiplier, unitstring
            ))
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
        "Speed",
        options,
        Box::new(|_cc| Ok(Box::new(SpeedApp::new()))),
    )
}
