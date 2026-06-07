//! Phase Shift Keying (PSK) modulation and demodulation.
//! Supports BPSK, QPSK, and M-PSK.

use std::f64::consts::PI;

/// A constellation point with real and imaginary components.
#[derive(Debug, Clone, Copy)]
pub struct ConstellationPoint {
    pub symbol: u32,
    pub re: f64,
    pub im: f64,
}

/// Generate M-PSK constellation points.
pub fn psk_constellation(m: u32) -> Vec<ConstellationPoint> {
    (0..m).map(|k| {
        let angle = 2.0 * PI * k as f64 / m as f64;
        ConstellationPoint {
            symbol: k,
            re: angle.cos(),
            im: angle.sin(),
        }
    }).collect()
}

/// PSK modulate a sequence of symbols.
///
/// # Arguments
/// * `symbols` - Input symbols (0 to M-1)
/// * `m` - Modulation order (2=BPSK, 4=QPSK, etc.)
/// * `samples_per_symbol` - Samples per symbol period
pub fn psk_modulate(symbols: &[u32], m: u32, samples_per_symbol: usize) -> Vec<f64> {
    let constellation = psk_constellation(m);
    let mut signal = Vec::with_capacity(symbols.len() * samples_per_symbol);

    for &sym in symbols {
        let idx = (sym % m) as usize;
        let point = constellation[idx];
        for i in 0..samples_per_symbol {
            let t = i as f64 / samples_per_symbol as f64;
            let carrier = (2.0 * PI * 4.0 * t).cos();
            signal.push(point.re * carrier - point.im * (2.0 * PI * 4.0 * t).sin());
        }
    }

    signal
}

/// PSK demodulate using coherent detection.
pub fn psk_demodulate(signal: &[f64], m: u32, samples_per_symbol: usize) -> Vec<u32> {
    let constellation = psk_constellation(m);
    let n_symbols = signal.len() / samples_per_symbol;
    let mut symbols = Vec::with_capacity(n_symbols);

    for sym in 0..n_symbols {
        let start = sym * samples_per_symbol;
        let end = start + samples_per_symbol;

        // Correlate with I and Q carriers
        let mut re = 0.0;
        let mut im = 0.0;
        for (idx, &sig_val) in signal.iter().enumerate().take(end).skip(start) {
            let t = (idx - start) as f64 / samples_per_symbol as f64;
            re += sig_val * (2.0 * PI * 4.0 * t).cos();
            im -= sig_val * (2.0 * PI * 4.0 * t).sin();
        }

        // Find nearest constellation point
        let mut best_sym = 0;
        let mut best_dist = f64::MAX;
        for point in &constellation {
            let dist = crate::complex_distance(re, im, point.re, point.im);
            if dist < best_dist {
                best_dist = dist;
                best_sym = point.symbol;
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
    fn test_bpsk_constellation() {
        let c = psk_constellation(2);
        assert_eq!(c.len(), 2);
        assert!((c[0].re - 1.0).abs() < 1e-14);
        assert!((c[1].re + 1.0).abs() < 1e-14);
    }

    #[test]
    fn test_qpsk_constellation() {
        let c = psk_constellation(4);
        assert_eq!(c.len(), 4);
        // All points should be on unit circle
        for p in &c {
            let r = p.re.hypot(p.im);
            assert!((r - 1.0).abs() < 1e-14, "Point not on unit circle: r={}", r);
        }
    }

    #[test]
    fn test_bpsk_roundtrip() {
        let symbols = [0, 1, 0, 1, 1, 0, 0, 1];
        let signal = psk_modulate(&symbols, 2, 100);
        let demod = psk_demodulate(&signal, 2, 100);
        assert_eq!(demod, symbols);
    }

    #[test]
    fn test_qpsk_roundtrip() {
        let symbols = [0, 1, 2, 3, 0, 2, 1, 3];
        let signal = psk_modulate(&symbols, 4, 100);
        let demod = psk_demodulate(&signal, 4, 100);
        assert_eq!(demod, symbols);
    }

    #[test]
    fn test_psk_8psk_constellation() {
        let c = psk_constellation(8);
        assert_eq!(c.len(), 8);
        // Adjacent points should be 45° apart
        let angle0 = c[0].im.atan2(c[0].re);
        let angle1 = c[1].im.atan2(c[1].re);
        let diff = (angle1 - angle0).abs();
        assert!((diff - PI / 4.0).abs() < 1e-14, "8-PSK spacing should be 45°: {}", diff);
    }

    #[test]
    fn test_psk_signal_length() {
        let symbols = [0, 1, 2];
        let signal = psk_modulate(&symbols, 4, 50);
        assert_eq!(signal.len(), 150);
    }

    #[test]
    fn test_bpsk_antipodal() {
        let c = psk_constellation(2);
        let dist = crate::complex_distance(c[0].re, c[0].im, c[1].re, c[1].im);
        assert!((dist - 2.0).abs() < 1e-14, "BPSK points should be 2 apart");
    }
}
