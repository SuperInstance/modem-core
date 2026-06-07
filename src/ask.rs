//! Amplitude Shift Keying (ASK) modulation and demodulation.

/// ASK modulate a bit sequence.
///
/// # Arguments
/// * `bits` - Input bits (0 or 1)
/// * `samples_per_symbol` - Number of samples per symbol period
/// * `amplitude` - Carrier amplitude for bit '1'
///
/// Returns the modulated signal.
pub fn ask_modulate(bits: &[u8], samples_per_symbol: usize, amplitude: f64) -> Vec<f64> {
    let mut signal = Vec::with_capacity(bits.len() * samples_per_symbol);
    for &bit in bits {
        let a = if bit == 1 { amplitude } else { 0.0 };
        for i in 0..samples_per_symbol {
            let t = i as f64 / samples_per_symbol as f64;
            signal.push(a * (2.0 * std::f64::consts::PI * 4.0 * t).sin());
        }
    }
    signal
}

/// ASK demodulate a signal using envelope detection.
///
/// Returns the demodulated bit sequence.
pub fn ask_demodulate(signal: &[f64], samples_per_symbol: usize, threshold: f64) -> Vec<u8> {
    let n_symbols = signal.len() / samples_per_symbol;
    let mut bits = Vec::with_capacity(n_symbols);

    for sym in 0..n_symbols {
        let start = sym * samples_per_symbol;
        let end = start + samples_per_symbol;
        // Envelope detection: average absolute value
        let energy: f64 = signal[start..end].iter().map(|x| x * x).sum::<f64>() / samples_per_symbol as f64;
        bits.push(if energy > threshold { 1 } else { 0 });
    }

    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ask_modulate_length() {
        let bits = [1, 0, 1, 1, 0];
        let signal = ask_modulate(&bits, 10, 1.0);
        assert_eq!(signal.len(), 50);
    }

    #[test]
    fn test_ask_bit0_is_silent() {
        let bits = [0];
        let signal = ask_modulate(&bits, 10, 1.0);
        for &s in &signal {
            assert!(s.abs() < 1e-14, "Bit 0 should produce silence");
        }
    }

    #[test]
    fn test_ask_bit1_has_energy() {
        let bits = [1];
        let signal = ask_modulate(&bits, 100, 1.0);
        let energy: f64 = signal.iter().map(|x| x * x).sum();
        assert!(energy > 0.0, "Bit 1 should have energy");
    }

    #[test]
    fn test_ask_roundtrip() {
        let bits: [u8; 8] = [1, 0, 1, 1, 0, 0, 1, 0];
        let signal = ask_modulate(&bits, 100, 1.0);
        let demod = ask_demodulate(&signal, 100, 0.1);
        assert_eq!(demod, bits);
    }

    #[test]
    fn test_ask_roundtrip_noisy() {
        let bits: [u8; 8] = [1, 0, 1, 1, 0, 0, 1, 0];
        let signal = ask_modulate(&bits, 100, 1.0);
        // Add noise
        let noisy: Vec<f64> = signal.iter().enumerate().map(|(i, &s)| {
            s + 0.1 * ((i as f64 * 3.7).sin() * 2.3 + (i as f64 * 1.1).cos())
        }).collect();
        let demod = ask_demodulate(&noisy, 100, 0.1);
        // Should mostly survive with low noise
        let errors: usize = demod.iter().zip(bits.iter()).filter(|(&a, &b)| a != b).count();
        assert!(errors <= 2, "Too many errors: {}", errors);
    }
}
