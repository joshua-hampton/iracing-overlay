#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use core::panic;
use eframe::egui;
use std::process::Command;

use crate::util::{
    toggle, HomeConfig, LaspLapTimeConfig, Overlays, SpeedConfig, SpeedUnits, WindowProcesses,
    WindowsConfig,
};

pub struct MainApp {
    menu_option: Overlays,
    config: WindowsConfig,
    windows: WindowProcesses,
}

impl MainApp {
    pub fn new() -> Self {
        let config: WindowsConfig = confy::load("iracing-overlays", None).unwrap_or_default();
        let windows: WindowProcesses = WindowProcesses::default();
        Self {
            menu_option: Overlays::Home,
            config,
            windows,
        }
    }
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.vertical_centered(|ui| ui.heading("iRacing Overlays"));
            ui.add_space(5.0);
        });
    }

    fn render_left_side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let home_btn = ui.add(egui::Button::new("Home"));
                if home_btn.clicked() {
                    self.menu_option = Overlays::Home;
                }

                let speed_btn = ui.add(egui::Button::new("Speed"));
                if speed_btn.clicked() {
                    self.menu_option = Overlays::Speed;
                }

                let lastlapbtn = ui.add(egui::Button::new("Last Lap Time"));
                if lastlapbtn.clicked() {
                    self.menu_option = Overlays::LastLapTime;
                }
            });
        });
    }

    fn render_main(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| match self.menu_option {
            Overlays::Home => self.render_home_options(ui),
            Overlays::Speed => self.render_speed_options(ui),
            Overlays::LastLapTime => self.render_lastlaptime_options(ui),
        });
    }

    fn render_footer(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.);
                ui.add(egui::Hyperlink::from_label_and_url(
                    "iRacing Overlays",
                    "https://github.com/joshua-hampton/iracing-overlay",
                ));
                ui.add_space(10.);
            })
        });
    }

    fn render_home_options(&mut self, ui: &mut egui::Ui) {
        ui.label("Home");

        ui.horizontal(|ui| {
            ui.label("Font size");
            ui.add(egui::DragValue::new(&mut self.config.home_config.font_size).range(6.0..=40.0).speed(0.5));
        });

        ui.horizontal(|ui| {
            ui.label("Pick background colour");
            ui.color_edit_button_srgba(&mut self.config.home_config.bg_colour);
        });

        ui.horizontal(|ui| {
            ui.label("Pick font colour");
            ui.color_edit_button_srgba(&mut self.config.home_config.font_colour);
        });
    }

    fn render_speed_options(&mut self, ui: &mut egui::Ui) {
        ui.label("Speed");

        ui.horizontal(|ui| {
            ui.label(format!("Show window: {}", self.config.speed_config.display));
            let display_toggle = ui.add(toggle(&mut self.config.speed_config.display));
            if display_toggle.clicked() {
                self.save_config();
                self.manage_speed_overlay();
            }
        });

        ui.horizontal(|ui| {
            ui.label("Font size");
            ui.add(egui::DragValue::new(&mut self.config.speed_config.font_size).range(6.0..=40.0).speed(0.5));
        });

        let mut units_enum = self.config.speed_config.units.clone();
        ui.horizontal(|ui| {
            ui.label("Units:");
            let unit_select_ms = ui.radio_value(
                &mut units_enum,
                SpeedUnits::Metrespersecond,
                "metres per second",
            );
            let unit_select_mph =
                ui.radio_value(&mut units_enum, SpeedUnits::Milesperhour, "miles per hour");
            let unit_select_kph = ui.radio_value(
                &mut units_enum,
                SpeedUnits::Kilometresperhour,
                "kilometres per hour",
            );
            if unit_select_ms.clicked() {
                self.config.speed_config.units = SpeedUnits::Metrespersecond;
            }
            if unit_select_mph.clicked() {
                self.config.speed_config.units = SpeedUnits::Milesperhour;
            }
            if unit_select_kph.clicked() {
                self.config.speed_config.units = SpeedUnits::Kilometresperhour;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Pick background colour");
            ui.color_edit_button_srgba(&mut self.config.speed_config.overlay_bgcolour);
        });

        ui.horizontal(|ui| {
            ui.label("Pick font colour");
            ui.color_edit_button_srgba(&mut self.config.speed_config.overlay_fontcolour);
        });
    }

    fn manage_speed_overlay(&mut self) {
        if self.config.speed_config.display && self.windows.speed.is_none() {
            self.windows.speed = Some(
                Command::new(r"C:\Program Files (x86)\iRacing Overlays\speed")
                    .spawn()
                    .expect("Failed to launch speed overlay"),
            );
        } else if !self.config.speed_config.display && self.windows.speed.is_some() {
            let Some(ref mut child) = self.windows.speed else {
                panic!("Oops")
            };
            let _ = child.kill();
            self.windows.speed = None;
        }
    }

    fn render_lastlaptime_options(&mut self, ui: &mut egui::Ui) {
        ui.label("Last lap time");

        ui.horizontal(|ui| {
          ui.label(format!(
              "Show window: {}",
              self.config.lastlaptime_config.display
          ));
          let display_toggle = ui.add(toggle(&mut self.config.lastlaptime_config.display));

          if display_toggle.clicked() {
              self.save_config();
              self.manage_lastlaptime_overlay();
          }
        });

        ui.horizontal(|ui| {
            ui.label("Font size");
            ui.add(egui::DragValue::new(&mut self.config.lastlaptime_config.font_size).range(6.0..=40.0).speed(0.5));
        });

        ui.horizontal(|ui| {
            ui.label("Pick background colour");
            ui.color_edit_button_srgba(&mut self.config.lastlaptime_config.overlay_bgcolour);
        });

        ui.horizontal(|ui| {
            ui.label("Pick font colour");
            ui.color_edit_button_srgba(&mut self.config.lastlaptime_config.overlay_fontcolour);
        });
    }

    fn manage_lastlaptime_overlay(&mut self) {
        if self.config.lastlaptime_config.display && self.windows.lastlaptime.is_none() {
            self.windows.lastlaptime = Some(
                Command::new(r"C:\Program Files (x86)\iRacing Overlays\lastlaptime")
                    .spawn()
                    .expect("Failed to launch Last Lap Time overlay"),
            );
        } else if !self.config.lastlaptime_config.display && self.windows.lastlaptime.is_some() {
            let Some(ref mut child) = self.windows.lastlaptime else {
                panic!("Oops")
            };
            let _ = child.kill();
            self.windows.lastlaptime = None;
        }
    }

    fn save_config(&self) {
        let _ = confy::store(
            "iracing-overlays",
            None,
            WindowsConfig {
                home_config: HomeConfig {
                    font_size: self.config.home_config.font_size,
                    bg_colour: self.config.home_config.bg_colour,
                    font_colour: self.config.home_config.font_colour,
                },
                speed_config: SpeedConfig {
                    display: self.config.speed_config.display,
                    font_size: self.config.speed_config.font_size,
                    units: self.config.speed_config.units.clone(),
                    overlay_bgcolour: self.config.speed_config.overlay_bgcolour,
                    overlay_fontcolour: self.config.speed_config.overlay_fontcolour,
                },
                lastlaptime_config: LaspLapTimeConfig {
                    display: self.config.lastlaptime_config.display,
                    font_size: self.config.lastlaptime_config.font_size,
                    overlay_bgcolour: self.config.lastlaptime_config.overlay_bgcolour,
                    overlay_fontcolour: self.config.lastlaptime_config.overlay_fontcolour,
                },
            },
        );
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (
                egui::TextStyle::Heading,
                egui::FontId::proportional(self.config.home_config.font_size * 2.),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::proportional(self.config.home_config.font_size),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::proportional(self.config.home_config.font_size),
            ),
        ]
        .into();
        ctx.set_style(style);

        let mut visuals = egui::Visuals::default();
        visuals.panel_fill = self.config.home_config.bg_colour;
        visuals.override_text_color = Some(self.config.home_config.font_colour);
        ctx.set_visuals(visuals);

        ctx.request_repaint();
        self.render_top_panel(ctx);
        self.render_left_side_panel(ctx);
        self.render_footer(ctx);
        self.render_main(ctx);

        self.manage_speed_overlay();
        self.manage_lastlaptime_overlay();
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Some(ref mut child) = self.windows.speed {
            let _ = child.kill();
        }
        if let Some(ref mut child) = self.windows.lastlaptime {
            let _ = child.kill();
        }
    }
}
