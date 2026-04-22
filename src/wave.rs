use core::f32;
use std::f32::consts::TAU;

use egui::ecolor::Color32;
use egui::{Pos2, Rect, emath, epaint, frame, pos2, vec2};

pub const SAMPLE_LENGTH: usize = 100;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Signal {
    freq: f32,
    amp: f32,
    phase: f32,
}

impl Signal {
    pub fn new(freq: f32, amp: f32, phase: f32) -> Self {
        Self {freq, amp, phase}
    }
    pub fn points(&self) -> Vec<Pos2> {
        (0..SAMPLE_LENGTH)
            .map(|i| {
                let t = i as f32 / (SAMPLE_LENGTH as f32);
                let y = self.amp * (t * self.freq * std::f32::consts::TAU + self.phase).sin();
                pos2(t as f32, y as f32)
            })
            .collect()
    }
    pub fn freq(&self) -> f32 {self.freq}
    pub fn amp(&self) -> f32 {self.amp}
    pub fn phase(&self) -> f32 {self.phase}
}

impl Default for Signal {
    fn default() -> Self {
        Self {
            freq: 1.0,
            amp: 1.0,
            phase: 0.0,
        }
    }
}

pub struct WaveViewer {
    points: Vec<Pos2>,
    y_range: f32,
}
impl WaveViewer {
    pub fn new(points: Vec<Pos2>, y_range: f32) -> Self {
        Self { points, y_range }
    }
}
impl eframe::App for WaveViewer {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::containers::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.request_repaint();
            let desired_size = ui.available_width() * vec2(1.0, 0.15 * self.y_range);
            let (_id, rect) = ui.allocate_space(desired_size);

            let to_screen =
                emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -self.y_range..=self.y_range), rect);

            let mut shapes = vec![];

            for p in self.points.clone() {
                shapes.push(epaint::Shape::circle_filled(
                    to_screen * p,
                    2.0,
                    Color32::LIGHT_BLUE,
                ));
            }

            ui.painter().extend(shapes);
        });
    }
}

impl eframe::App for Signal {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add(egui::Slider::new(&mut self.freq, 0.0..=10.0).text("freq"));
                ui.add(egui::Slider::new(&mut self.amp, 0.0..=1.0).text("amp"));
                ui.add(egui::Slider::new(&mut self.phase, 0.0..=TAU).text("phase"));
            });
            WaveViewer::new(self.points(), 1.0).ui(ui, _frame);
        });
    }
}
