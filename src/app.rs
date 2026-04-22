use egui::{Pos2, frame};
use symphonia::core::audio::AudioBuffer;

use crate::{
    Signal, analyze::dft::dft, spectrum_display::SpectrumDisplay, wave::{self, SAMPLE_LENGTH, WaveViewer}, waveform_display::{AudioHandle, WaveformDisplay}
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    wave1: Signal,
    wave2: Signal,
    wave3: Signal,
    audio: AudioHandle,
    wd: Option<WaveformDisplay>,
    specd: Option<SpectrumDisplay>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            wave1: Default::default(),
            wave2: Default::default(),
            wave3: Default::default(),
            audio: AudioHandle::new("assets/qwen_tts_output.wav".into()),
            wd: None,
            specd: None
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.heading("sin wave drawing");
            //
            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            // });
            //
            // let mut points = self.wave1.points();
            // let points2 = self.wave2.points();
            // let points3 = self.wave3.points();
            // for i in 0..SAMPLE_LENGTH {
            //     points[i] = Pos2::new(
            //         points[i].x + points2[i].x + points3[i].x,
            //         points[i].y + points2[i].y + points3[i].y,
            //     );
            // }
            //
            // WaveViewer::new(points, 3.0).ui(ui, frame);
            // ui.separator();
            //
            // self.wave1.ui(ui, frame);
            // ui.separator();
            //
            // self.wave2.ui(ui, frame);
            // ui.separator();
            //
            // self.wave3.ui(ui, frame);
            // ui.separator();
            //

            if self.wd.is_none() {
                let audio_data = self.audio.read_audio();
                self.wd = Some(WaveformDisplay::new(audio_data.clone()));
                let data: Vec<_> = audio_data.iter().map(|p| p.y).collect();
                self.specd = Some(SpectrumDisplay::new(dft(&data, 44100)))
            }

            if let Some(display) = &mut self.wd {
                display.ui(ui, frame);
            }
            if let Some(display) = &mut self.specd {
                display.ui(ui, frame);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
