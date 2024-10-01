#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iracing::telemetry::Value;
use iracing::Connection;
use std::time::{Duration, Instant};

pub struct IRacingTelemetry {
    pub speed: f32,
    pub lastlaptime: f32,
}

pub struct IRacingLogging {
    connection: Connection,
    last_update: Instant,
    pub telemetry: IRacingTelemetry,
}

impl IRacingLogging {
    pub fn new() -> Self {
        Self {
            connection: Connection::new().expect("Failed to connect to iRacing"),
            last_update: Instant::now(),
            telemetry: IRacingTelemetry {
                speed: 0.0,
                lastlaptime: 0.0,
            },
        }
    }

    pub fn update_telemetry(&mut self) {
        //println!("Checking");
        if self.last_update.elapsed().as_millis() >= 16 {
            //println!("updating");
            //self.telemetry.speed += 1.;
            //self.telemetry.lastlaptime += 0.5;
            if let Ok(data) = self.connection.blocking().expect("Unable to start telemetry reader").sample(Duration::from_millis(16)) {
                match data.get("Speed") {
                    Err(..) => (),
                    Ok(value) => match value {
                        Value::FLOAT(f) => {
                            self.telemetry.speed = f;
                            ()
                        },
                        _ => (),
                    }
                }
                match data.get("LapLastLapTime") {
                    Err(..) => (),
                    Ok(value) => match value {
                        Value::FLOAT(f) => {
                            self.telemetry.lastlaptime = f;
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
