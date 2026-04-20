#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod wave;
mod audio;
pub use app::App;
pub use wave::SinSignal;

