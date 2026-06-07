//! Frequency Shift Keying (FSK) modulation and demodulation.

/// FSK modulate a bit sequence.
///
/// # Arguments
/// * `bits` - Input bits (0 or 1)
/// * `samples_per_symbol` - Samples per symbol period
/// * `freq0` - Frequency for bit 0
/// * `freq1` - Frequency for bit 1
pub fn fsk_modulate(bits: &[u8], samples_per_symbol: usize, freq0: f64, freq1: f64) -> Vec<f64> {
    let mut signal = Vec::with_capacity(bits.len() * samples_per_symbol);
    let mut phase: f64 = 0.0;

    for &bit in bits {
        let freq = if bit == 1 { freq1 } else { freq0 };
        for i in 0..samples_per_symbol {
            let _t = i as f64 / samples_per_symbol as f64;
            let sample = phase.sin();
            phase += 2.0_f64 * std::f64::consts::PI * freq / samples_per_symbol as f64;
            signal.push(sample);
        }
    }

    signal
}

/// FSK demodulate using frequency discrimination.
///
/// Compares energy at each frequency to determine the transmitted bit.
pub fn fsk_demodulate(signal: &[f64], samples_per_symbol: usize, freq0: f64, freq1: f64) -> Vec<u8> {
    let n_symbols = signal.len() / samples_per_symbol;
    let mut bits = Vec::with_capacity(n_symbols);

    for sym in 0..n_symbols {
        let start = sym * samples_per_symbol;
        let end = start + samples_per_symbol;

        // Correlate with each frequency
        let mut corr0 = 0.0;
        let mut corr1 = 0.0;
        for (idx, &sig_val) in signal.iter().enumerate().take(end).skip(start) {
            let t = (idx - start) as f64 / samples_per_symbol as f64;
            corr0 += sig_val * (2.0 * std::f64::consts::PI * freq0 * t).sin();
            corr1 += sig_val * (2.0 * std::f64::consts::PI * freq1 * t).sin();
        }

        bits.push(if corr1.abs() > corr0.abs() { 1 } else { 0 });
    }

    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fsk_modulate_length() {
        let bits = [1, 0, 1];
        let signal = fsk_modulate(&bits, 100, 2.0, 6.0);
        assert_eq!(signal.len(), 300);
    }

    #[test]
    fn test_fsk_roundtrip() {
        let bits = [1, 0, 1, 0, 1, 1, 0, 0];
        let signal = fsk_modulate(&bits, 100, 2.0, 6.0);
        let demod = fsk_demodulate(&signal, 100, 2.0, 6.0);
        assert_eq!(demod, bits);
    }

    #[test]
    fn test_fsk_different_frequencies() {
        let signal0 = fsk_modulate(&[0], 200, 2.0, 8.0);
        let signal1 = fsk_modulate(&[1], 200, 2.0, 8.0);
        // The two signals should be different
        let diff: f64 = signal0.iter().zip(signal1.iter()).map(|(a, b)| (a - b) * (a - b)).sum();
        assert!(diff > 1.0, "FSK signals should differ for different bits");
    }

    #[test]
    fn test_fsk_all_zeros() {
        let bits = [0, 0, 0, 0];
        let signal = fsk_modulate(&bits, 100, 2.0, 6.0);
        let demod = fsk_demodulate(&signal, 100, 2.0, 6.0);
        assert_eq!(demod, bits);
    }

    #[test]
    fn test_fsk_all_ones() {
        let bits = [1, 1, 1, 1];
        let signal = fsk_modulate(&bits, 100, 2.0, 6.0);
        let demod = fsk_demodulate(&signal, 100, 2.0, 6.0);
        assert_eq!(demod, bits);
    }
}
