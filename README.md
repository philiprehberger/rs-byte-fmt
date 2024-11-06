# rs-byte-fmt

[![CI](https://github.com/philiprehberger/rs-byte-fmt/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-byte-fmt/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-byte-fmt.svg)](https://crates.io/crates/philiprehberger-byte-fmt)
[![License](https://img.shields.io/github/license/philiprehberger/rs-byte-fmt)](LICENSE)

Human-readable byte size formatting and parsing.

## Installation

```toml
[dependencies]
philiprehberger-byte-fmt = "0.1.5"
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

## API

| Function / Type | Description |
|-----------------|-------------|
| `ByteSize::from_bytes(n)` | Create from raw byte count |
| `ByteSize::from_kb(n)` | Create from kilobytes |
| `ByteSize::from_mb(n)` | Create from megabytes |
| `ByteSize::from_gb(n)` | Create from gigabytes |
| `ByteSize::from_tb(n)` | Create from terabytes |
| `.as_bytes()` | Get raw byte count |
| `.format_binary()` | Format using binary units (KiB, MiB, etc.) |
| `.with_precision(n)` | Format with n decimal places |
| `.to_string()` | Format using SI units (KB, MB, etc.) |
| `"1.5 GB".parse::<ByteSize>()` | Parse from string |

## License

MIT
