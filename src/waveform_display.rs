use egui::{Color32, Pos2, Rect, Stroke, emath, epaint, vec2};

use crate::audio::audio;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AudioHandle {
    file_name: String,
    waveform: Option<Vec<Pos2>>,
}

impl AudioHandle {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            waveform: None,
        }
    }
    pub fn read_audio(&mut self) -> &Vec<Pos2> {
        if self.waveform.is_none() {
            let points = audio("assets/qwen_tts_output.wav").unwrap();
            let mut ret = Vec::with_capacity(points.len());
            for (i, y) in points.iter().enumerate() {
                let t = i as f32 / points.len() as f32;
                let y = y.abs();
                ret.push(Pos2::new(t, y));
            }
            self.waveform = Some(ret);
        }
        self.waveform.as_ref().unwrap()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WaveformDisplay {
    waveform: Vec<Pos2>,
}
impl WaveformDisplay {
    pub fn new(waveform: Vec<Pos2>) -> Self{ Self {waveform}}
}

impl eframe::App for WaveformDisplay {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::containers::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.request_repaint();
            let desired_size = ui.available_width() * vec2(1.0, 0.15);
            let (_id, rect) = ui.allocate_space(desired_size);

            let to_screen =
                emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

            let mut shapes = vec![];

            for p in &self.waveform {
                // shapes.push(epaint::Shape::circle_filled(
                //     to_screen * p.clone(),
                //     2.0,
                //     Color32::LIGHT_BLUE,
                // ));
                shapes.push(epaint::Shape::LineSegment {
                    points: [to_screen * Pos2::new(p.x, -p.y), to_screen * p.clone()],
                    stroke: Stroke::new(0.1, Color32::LIGHT_BLUE),
                });
            }

            ui.painter().extend(shapes);
        });
    }
}
