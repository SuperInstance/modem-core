# modem-core

Digital modulation and demodulation in pure Rust.

## Features

- **ASK** — Amplitude Shift Keying with envelope detection
- **FSK** — Frequency Shift Keying with coherent detection
- **PSK** — Phase Shift Keying (BPSK, QPSK, M-PSK) with constellation analysis
- **QAM** — Quadrature Amplitude Modulation (4/16/64/256-QAM)
- **BER** — Bit Error Rate computation and theoretical curves

## Modules

| Module | Description |
|--------|-------------|
| `ask` | ASK modulation and demodulation |
| `fsk` | FSK modulation and demodulation |
| `psk` | PSK modulation, demodulation, constellation |
| `qam` | QAM modulation and demodulation |
| `ber` | BER computation and theoretical curves |

## Quick Start

```rust
use modem_core::{psk_modulate, psk_demodulate, ber_compute};

let symbols = [0, 1, 0, 1, 1, 0];
let signal = psk_modulate(&symbols, 2, 100); // BPSK
let demod = psk_demodulate(&signal, 2, 100);

let ber = ber_theoretical_bpsk(10.0); // BER at 10 dB SNR
```

## License

MIT OR Apache-2.0
