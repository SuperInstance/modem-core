//! Quadrature Amplitude Modulation (QAM) modulation and demodulation.


/// Generate a square QAM constellation (16-QAM, 64-QAM, etc.).
///
/// # Arguments
/// * `m` - Modulation order (must be a perfect square: 4, 16, 64, 256)
pub fn qam_constellation(m: u32) -> Vec<(u32, f64, f64)> {
    let sq = (m as f64).sqrt() as u32;
    assert_eq!(sq * sq, m, "QAM order must be a perfect square");

    let mut points = Vec::with_capacity(m as usize);
    let mut sym = 0;

    // Normalize so average power = 1
    let scale = (3.0 / (m - 1) as f64).sqrt();
    let half = (sq - 1) as f64 / 2.0;

    for i in 0..sq {
        for j in 0..sq {
            let re = (j as f64 - half) * scale;
            let im = (i as f64 - half) * scale;
            points.push((sym, re, im));
            sym += 1;
        }
    }

    points
}

/// QAM modulate a sequence of symbols.
///
/// Returns the complex baseband signal as interleaved (I, Q) pairs.
pub fn qam_modulate(symbols: &[u32], m: u32) -> Vec<f64> {
    let constellation = qam_constellation(m);
    let mut signal = Vec::with_capacity(symbols.len() * 2);

    for &sym in symbols {
        let idx = (sym % m) as usize;
        let (_, re, im) = constellation[idx];
        signal.push(re);
        signal.push(im);
    }

    signal
}

/// QAM demodulate using minimum distance detection.
///
/// Input is interleaved (I, Q) pairs.
pub fn qam_demodulate(signal: &[f64], m: u32) -> Vec<u32> {
    let constellation = qam_constellation(m);
    let n_symbols = signal.len() / 2;
    let mut symbols = Vec::with_capacity(n_symbols);

    for i in 0..n_symbols {
        let re = signal[2 * i];
        let im = signal[2 * i + 1];

        let mut best_sym = 0;
        let mut best_dist = f64::MAX;
        for &(sym, cre, cim) in &constellation {
            let dist = crate::complex_distance(re, im, cre, cim);
            if dist < best_dist {
                best_dist = dist;
                best_sym = sym;
            }
        }

        symbols.push(best_sym);
    }

    symbols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qam16_constellation_size() {
        let c = qam_constellation(16);
        assert_eq!(c.len(), 16);
    }

    #[test]
    fn test_qam4_constellation() {
        let c = qam_constellation(4);
        assert_eq!(c.len(), 4);
        // 4-QAM is same as QPSK essentially
    }

    #[test]
    fn test_qam16_roundtrip() {
        let symbols = [0, 5, 10, 15, 3, 7, 11, 12];
        let signal = qam_modulate(&symbols, 16);
        let demod = qam_demodulate(&signal, 16);
        assert_eq!(demod, symbols);
    }

    #[test]
    fn test_qam4_roundtrip() {
        let symbols = [0, 1, 2, 3, 0, 2, 1, 3];
        let signal = qam_modulate(&symbols, 4);
        let demod = qam_demodulate(&signal, 4);
        assert_eq!(demod, symbols);
    }

    #[test]
    fn test_qam64_roundtrip() {
        let symbols = [0, 15, 30, 45, 63, 32, 10, 50];
        let signal = qam_modulate(&symbols, 64);
        let demod = qam_demodulate(&signal, 64);
        assert_eq!(demod, symbols);
    }

    #[test]
    fn test_qam_signal_length() {
        let symbols = [0, 1, 2];
        let signal = qam_modulate(&symbols, 16);
        assert_eq!(signal.len(), 6); // 3 symbols * 2 (I,Q)
    }

    #[test]
    fn test_qam_symmetry() {
        let c = qam_constellation(16);
        // Check that constellation has symmetric points
        let mut sum_re = 0.0;
        let mut sum_im = 0.0;
        for (_, re, im) in &c {
            sum_re += re;
            sum_im += im;
        }
        assert!(sum_re.abs() < 1e-10, "Constellation should be centered: sum_re={}", sum_re);
        assert!(sum_im.abs() < 1e-10, "Constellation should be centered: sum_im={}", sum_im);
    }

    #[test]
    #[should_panic(expected = "perfect square")]
    fn test_qam_invalid_order() {
        qam_constellation(8);
    }
}
