use egui::{Color32, FontDefinitions, Galley, Pos2, Rect, Stroke, emath, epaint::{self, TextOptions}, text::Fonts, vec2};

use crate::{Signal, audio::audio};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SpectrumDisplay {
    spectrum: Vec<Signal>,
}
impl SpectrumDisplay {
    pub fn new(signals: Vec<Signal>) -> Self {
        Self { spectrum: signals }
    }
}

impl eframe::App for SpectrumDisplay {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::containers::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.request_repaint();
            let desired_size = ui.available_width() * vec2(1.0, 0.15);
            let (_id, rect) = ui.allocate_space(desired_size);

            let mut data: Vec<_> = vec![];
            let mut y_max: f32 = 0.0;
            for s in self.spectrum.iter() {
                y_max = y_max.max(s.amp());
                data.push(Pos2::new(s.freq(), s.amp()));
            }
            let x_max = data[data.len() - 1].x;

            let to_screen = emath::RectTransform::from_to(
                Rect::from_x_y_ranges(0.0..=x_max, 0.0..=y_max),
                rect,
            );

            let mut shapes = vec![];

            for p in data.iter() {
                shapes.push(epaint::Shape::LineSegment {
                    points: [to_screen * Pos2::new(p.x, 0.0), to_screen * p.clone()],
                    stroke: Stroke::new(1.0, Color32::LIGHT_GREEN),
                });
            }

            ui.painter().extend(shapes);
            ui.painter().text(
                to_screen * egui::Pos2::new(x_max, y_max), // Position
                egui::Align2::RIGHT_BOTTOM,        // Anchor
                format!("ymax: {}, xmax: {}", y_max, x_max),              // Text
                egui::FontId::proportional(16.0), // Font
                egui::Color32::LIGHT_GREEN,          // Color
            );
        });
    }
}
