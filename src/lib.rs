#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod wave;
mod audio;
#[macro_use]
mod utils;
mod waveform_display;
mod spectrum_display;
mod analyze;
pub use app::App;
pub use wave::Signal;

