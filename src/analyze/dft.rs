use std::f64::consts::TAU;

use crate::{Signal, analyze::Vec2};

pub fn dft(samples: &[f64], sample_rate: usize) -> Vec<Signal> {
    let freq_count = samples.len() / 2 + 1;
    let sample_size: f64 = samples.len() as f64;
    println!("{}", sample_size);
    let mut spectrum = Vec::with_capacity(freq_count);
    let freq_step = sample_rate as f64 / sample_size;

    for freq_index in 0..freq_count {
        let mut sample_sum = Vec2::default();
        for (i, sample) in samples.iter().enumerate() {
            let angle = i as f64 / sample_size * TAU * (freq_index as f64);
            let point = Vec2::new(angle.cos(), angle.sin());
            sample_sum += point * (*sample);
        }

        let center = sample_sum / sample_size;

        let is_0 = freq_index == 0;
        let is_nyquist_freq = freq_index == freq_count - 1 && samples.len() % 2 == 0;
        let amp_scale = if is_0 || is_nyquist_freq { 1. } else { 2. };
        let amp = center.magnitude() * amp_scale;
        let freq = freq_index as f64 * freq_step;
        spectrum.push(Signal::new(freq, amp, center.phase()));
    }

    spectrum
}

#[cfg(test)]
mod tests {
    use crate::wave::SAMPLE_LENGTH;

    use super::*;

    #[test]
    fn test_dft() {
        let signal = Signal::new(3., 1., 0.);
        let signal2 = Signal::new(5., 2., 0.);
        let samples: Vec<_> = signal
            .points()
            .iter()
            .zip(signal2.points().iter())
            .map(|(pos1, pos2)| pos1.y as f64 + pos2.y as f64)
            .collect();
        let res = dft(&samples, SAMPLE_LENGTH);
        for s in res.iter() {
            println!("{:?}", s);
        }
    }
}
