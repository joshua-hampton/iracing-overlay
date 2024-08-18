#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::time::Duration;
use iracing::Connection;
use iracing::telemetry::Value;
use std::time::Instant;

struct IRacingApp {
    connection: Connection,
    speed: f32,
    last_update: Instant,
}

impl IRacingApp {
    fn new() -> Self {
        Self {
            connection: Connection::new().expect("Failed to connect to iRacing"),
            speed: 0.0,
            last_update: Instant::now(),
        }
    }

    fn update_telemetry(&mut self) {
        if self.last_update.elapsed().as_millis() >= 16 {
            if let Ok(telemetry) = self.connection.blocking().expect("Unable to start telemetry reader").sample(Duration::from_millis(16)) {
                match telemetry.get("Speed") {
                    Err(..) => (),
                    Ok(value) => match value {
                        Value::FLOAT(f) => {
                            self.speed = f*3.6;
                            ()
                        },
                        _ => (),
                    }
                }
            }
            self.last_update = Instant::now();
        }
    }
}

impl eframe::App for IRacingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_telemetry();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("iRacing Telemetry");
            ui.label(format!("Your speed: {:.2}", self.speed));
        });

        // Request a repaint
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "iRacing Telemetry",
        options,
        Box::new(|_cc| Ok(Box::new(IRacingApp::new())))
    )
}