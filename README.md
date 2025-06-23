# rs-byte-fmt

[![CI](https://github.com/philiprehberger/rs-byte-fmt/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-byte-fmt/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-byte-fmt.svg)](https://crates.io/crates/philiprehberger-byte-fmt)
[![License](https://img.shields.io/github/license/philiprehberger/rs-byte-fmt)](LICENSE)

Human-readable byte size formatting and parsing

## Installation

```toml
[dependencies]
philiprehberger-byte-fmt = "0.1.8"
```

## Usage

```rust
use philiprehberger_byte_fmt::ByteSize;

// Format bytes
let size = ByteSize::from_bytes(1_500_000);
assert_eq!(size.to_string(), "1.5 MB");

// Binary units
assert_eq!(ByteSize::from_bytes(1_048_576).format_binary(), "1 MiB");

// Parse strings
let size: ByteSize = "1.5 GB".parse().unwrap();
assert_eq!(size.as_bytes(), 1_500_000_000);

// Custom precision
let size = ByteSize::from_bytes(1_536_000);
assert_eq!(size.with_precision(2).to_string(), "1.54 MB");
```

### Arithmetic and conversions

```rust
use philiprehberger_byte_fmt::ByteSize;

// Add and subtract
let total = ByteSize::from_mb(10.0) + ByteSize::from_mb(5.0);
assert_eq!(total.to_string(), "15 MB");

let diff = ByteSize::from_gb(2.0) - ByteSize::from_gb(0.5);
assert_eq!(diff.to_string(), "1.5 GB");

// Scalar multiply and divide
let doubled = ByteSize::from_kb(500.0) * 2;
assert_eq!(doubled.to_string(), "1 MB");

// Unit conversions
let size = ByteSize::from_bytes(2_500_000);
assert_eq!(size.to_mb(), 2.5);
assert_eq!(size.to_kb(), 2500.0);
```

## API

| Function / Type | Description |
|-----------------|-------------|
| `ByteSize::from_bytes(n)` | Create from raw byte count |
| `ByteSize::from_kb(n)` | Create from kilobytes |
| `ByteSize::from_mb(n)` | Create from megabytes |
| `ByteSize::from_gb(n)` | Create from gigabytes |
| `ByteSize::from_tb(n)` | Create from terabytes |
| `.as_bytes()` | Get raw byte count |
| `.to_kb()`, `.to_mb()`, `.to_gb()`, `.to_tb()` | Convert to SI unit as `f64` |
| `.format_binary()` | Format using binary units (KiB, MiB, etc.) |
| `.with_precision(n)` | Format with n decimal places |
| `.to_string()` | Format using SI units (KB, MB, etc.) |
| `"1.5 GB".parse::<ByteSize>()` | Parse from string |
| `ByteSize::from(n)` | Create from `u64` byte count |
| `+`, `-`, `+=`, `-=` | Arithmetic between two `ByteSize` values |
| `*`, `/`, `*=`, `/=` | Scalar arithmetic with `u64` |


## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## License

MIT
