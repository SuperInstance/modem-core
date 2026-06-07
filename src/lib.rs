//! # Modem Core
//!
//! Digital modulation and demodulation: ASK, FSK, PSK, QAM
//! with constellation diagrams and BER analysis.

pub mod ask;
pub mod fsk;
pub mod psk;
pub mod qam;
pub mod ber;

pub use ask::{ask_modulate, ask_demodulate};
pub use fsk::{fsk_modulate, fsk_demodulate};
pub use psk::{psk_modulate, psk_demodulate, ConstellationPoint};
pub use qam::{qam_modulate, qam_demodulate};
pub use ber::{ber_compute, ber_theoretical_bpsk, ber_theoretical_qpsk};

/// Simple pseudo-random number generator for noise generation.
pub fn noise_generator(n: usize, amplitude: f64, seed: u64) -> Vec<f64> {
    let mut state = seed;
    (0..n).map(|_| {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let x = ((state >> 33) as i64 as f64) / (1i64 << 31) as f64;
        x * amplitude
    }).collect()
}

/// Compute Euclidean distance between two complex numbers.
pub fn complex_distance(re1: f64, im1: f64, re2: f64, im2: f64) -> f64 {
    ((re1 - re2) * (re1 - re2) + (im1 - im2) * (im1 - im2)).sqrt()
}
