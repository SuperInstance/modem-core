//! Bit Error Rate (BER) computation and theoretical curves.

use std::f64::consts::PI;

/// Compute the empirical BER between transmitted and received bit sequences.
pub fn ber_compute(transmitted: &[u8], received: &[u8]) -> f64 {
    assert_eq!(transmitted.len(), received.len(), "Sequences must have equal length");
    if transmitted.is_empty() {
        return 0.0;
    }
    let errors: usize = transmitted.iter().zip(received.iter())
        .filter(|(&a, &b)| a != b)
        .count();
    errors as f64 / transmitted.len() as f64
}

/// Compute BER between two symbol sequences.
pub fn ser_compute(transmitted: &[u32], received: &[u32]) -> f64 {
    assert_eq!(transmitted.len(), received.len());
    if transmitted.is_empty() {
        return 0.0;
    }
    let errors: usize = transmitted.iter().zip(received.iter())
        .filter(|(&a, &b)| a != b)
        .count();
    errors as f64 / transmitted.len() as f64
}

/// Theoretical BER for BPSK in AWGN: Q(sqrt(2·SNR))
/// Approximated using the complementary error function.
pub fn ber_theoretical_bpsk(snr_db: f64) -> f64 {
    let snr_lin = 10.0_f64.powf(snr_db / 10.0);
    q_function((2.0 * snr_lin).sqrt())
}

/// Theoretical BER for QPSK (same as BPSK per bit).
pub fn ber_theoretical_qpsk(snr_db: f64) -> f64 {
    ber_theoretical_bpsk(snr_db)
}

/// Theoretical BER for M-PSK.
pub fn ber_theoretical_mpsk(snr_db: f64, m: u32) -> f64 {
    let snr_lin = 10.0_f64.powf(snr_db / 10.0);
    let log2m = (m as f64).log2();
    // Approximate formula
    (2.0 / log2m) * q_function((2.0 * snr_lin * log2m * (PI / m as f64).sin().powi(2)).sqrt())
}

/// Q-function approximation using erfc.
fn q_function(x: f64) -> f64 {
    0.5 * erfc(x / std::f64::consts::SQRT_2)
}

/// Complementary error function approximation.
fn erfc(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    1.0 - sign * y
}

/// Generate a BER curve: list of (SNR_dB, BER) pairs.
pub fn ber_curve(snr_range_db: &[f64], ber_func: &dyn Fn(f64) -> f64) -> Vec<(f64, f64)> {
    snr_range_db.iter().map(|&snr| (snr, ber_func(snr))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ber_zero_errors() {
        let tx = [1, 0, 1, 1, 0];
        let rx = [1, 0, 1, 1, 0];
        assert!((ber_compute(&tx, &rx) - 0.0).abs() < 1e-14);
    }

    #[test]
    fn test_ber_all_errors() {
        let tx = [1, 0, 1, 0];
        let rx = [0, 1, 0, 1];
        assert!((ber_compute(&tx, &rx) - 1.0).abs() < 1e-14);
    }

    #[test]
    fn test_ber_half_errors() {
        let tx = [1, 0, 1, 0];
        let rx = [0, 0, 1, 1];
        assert!((ber_compute(&tx, &rx) - 0.5).abs() < 1e-14);
    }

    #[test]
    fn test_ber_empty() {
        assert!((ber_compute(&[], &[]) - 0.0).abs() < 1e-14);
    }

    #[test]
    fn test_ber_bpsk_decreasing() {
        // BER should decrease with increasing SNR
        let ber_0db = ber_theoretical_bpsk(0.0);
        let ber_5db = ber_theoretical_bpsk(5.0);
        let ber_10db = ber_theoretical_bpsk(10.0);
        assert!(ber_0db > ber_5db, "BER should decrease: {} > {}", ber_0db, ber_5db);
        assert!(ber_5db > ber_10db, "BER should decrease: {} > {}", ber_5db, ber_10db);
    }

    #[test]
    fn test_ber_bpsk_range() {
        let ber = ber_theoretical_bpsk(0.0);
        assert!(ber > 0.0 && ber < 1.0, "BER should be in (0,1): {}", ber);
    }

    #[test]
    fn test_ber_curve_length() {
        let snrs = [-5.0, 0.0, 5.0, 10.0, 15.0];
        let curve = ber_curve(&snrs, &ber_theoretical_bpsk);
        assert_eq!(curve.len(), 5);
    }

    #[test]
    fn test_ser_compute() {
        let tx = [0u32, 1, 2, 3];
        let rx = [0u32, 1, 2, 0];
        let ser = ser_compute(&tx, &rx);
        assert!((ser - 0.25).abs() < 1e-14);
    }
}
