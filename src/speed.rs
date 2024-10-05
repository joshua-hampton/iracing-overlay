#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, CentralPanel, Context};
use eframe::{App, NativeOptions};
use util::SpeedUnits;
mod telemetry;
mod util;

struct SpeedApp {
    font_size: f32,
    local_telem: telemetry::IRacingLogging,
    overlay_bgcolour: egui::Color32,
    overlay_fontcolour: egui::Color32,
    units: util::SpeedUnits,
}

impl SpeedApp {
    fn new() -> Self {
        let config: util::WindowsConfig = confy::load("iracing-overlays", None).unwrap_or_default();
        let font_size: f32 = config.speed_config.font_size;
        let units: util::SpeedUnits = config.speed_config.units;
        let overlay_bgcolour: egui::Color32 = config.speed_config.overlay_bgcolour;
        let overlay_fontcolour: egui::Color32 = config.speed_config.overlay_fontcolour;
        Self {
            font_size,
            local_telem: telemetry::IRacingLogging::new(),
            overlay_bgcolour,
            overlay_fontcolour,
            units,
        }
    }
    fn update_telemetry(&mut self) {
        telemetry::IRacingLogging::update_telemetry(&mut self.local_telem)
    }
}

impl App for SpeedApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (
                egui::TextStyle::Body,
                egui::FontId::proportional(self.font_size),
            ),
        ]
        .into();
        ctx.set_style(style);

        let mut visuals = egui::Visuals::default();
        visuals.panel_fill = self.overlay_bgcolour;
        visuals.override_text_color = Some(self.overlay_fontcolour);
        ctx.set_visuals(visuals);

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
