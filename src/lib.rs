//! Human-readable byte size formatting and parsing.
//!
//! This crate provides a [`ByteSize`] type that wraps a `u64` byte count and supports
//! formatting to human-readable strings and parsing from them.
//!
//! # Examples
//!
//! ```
//! use philiprehberger_byte_fmt::ByteSize;
//!
//! // Format bytes using SI units (default)
//! let size = ByteSize::from_bytes(1_500_000);
//! assert_eq!(size.to_string(), "1.5 MB");
//!
//! // Format using binary units
//! let size = ByteSize::from_bytes(1_048_576);
//! assert_eq!(size.format_binary(), "1 MiB");
//!
//! // Parse from string
//! let size: ByteSize = "1.5 GB".parse().unwrap();
//! assert_eq!(size.as_bytes(), 1_500_000_000);
//!
//! // Custom precision
//! let size = ByteSize::from_bytes(1_536_000);
//! assert_eq!(size.with_precision(2).to_string(), "1.54 MB");
//! ```

use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

const KB: u64 = 1_000;
const MB: u64 = 1_000_000;
const GB: u64 = 1_000_000_000;
const TB: u64 = 1_000_000_000_000;

const KIB: u64 = 1_024;
const MIB: u64 = 1_048_576;
const GIB: u64 = 1_073_741_824;
const TIB: u64 = 1_099_511_627_776;

/// A byte size value that can be formatted as a human-readable string and parsed from one.
///
/// By default, [`Display`](fmt::Display) uses SI units (KB = 1000) and automatically
/// selects the most appropriate unit.
///
/// # Examples
///
/// ```
/// use philiprehberger_byte_fmt::ByteSize;
///
/// let size = ByteSize::from_bytes(2_500_000);
/// assert_eq!(size.to_string(), "2.5 MB");
/// assert_eq!(size.as_bytes(), 2_500_000);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ByteSize(u64);

impl Hash for ByteSize {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

/// A wrapper around [`ByteSize`] that formats with a specific decimal precision.
///
/// Created by [`ByteSize::with_precision`].
#[derive(Clone, Copy, Debug)]
pub struct FormattedByteSize {
    bytes: u64,
    precision: usize,
}

/// Errors that can occur when parsing a byte size string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The input string was empty.
    Empty,
    /// The numeric part of the input could not be parsed.
    InvalidNumber(String),
    /// The unit suffix was not recognized.
    InvalidUnit(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "empty input"),
            ParseError::InvalidNumber(s) => write!(f, "invalid number: {s}"),
            ParseError::InvalidUnit(s) => write!(f, "invalid unit: {s}"),
        }
    }
}

impl std::error::Error for ParseError {}

impl ByteSize {
    /// Creates a [`ByteSize`] from a raw byte count.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_byte_fmt::ByteSize;
    ///
    /// let size = ByteSize::from_bytes(1024);
    /// assert_eq!(size.as_bytes(), 1024);
    /// ```
    pub fn from_bytes(n: u64) -> Self {
        Self(n)
    }

    /// Creates a [`ByteSize`] from kilobytes (SI, 1 KB = 1000 bytes).
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_byte_fmt::ByteSize;
    ///
    /// let size = ByteSize::from_kb(1.5);
    /// assert_eq!(size.as_bytes(), 1500);
    /// ```
    pub fn from_kb(n: f64) -> Self {
        Self((n * KB as f64) as u64)
    }

    /// Creates a [`ByteSize`] from megabytes (SI, 1 MB = 1,000,000 bytes).
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_byte_fmt::ByteSize;
    ///
    /// let size = ByteSize::from_mb(2.5);
    /// assert_eq!(size.as_bytes(), 2_500_000);
    /// ```
    pub fn from_mb(n: f64) -> Self {
        Self((n * MB as f64) as u64)
    }

    /// Creates a [`ByteSize`] from gigabytes (SI, 1 GB = 1,000,000,000 bytes).
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_byte_fmt::ByteSize;
    ///
    /// let size = ByteSize::from_gb(1.5);
    /// assert_eq!(size.as_bytes(), 1_500_000_000);
    /// ```
    pub fn from_gb(n: f64) -> Self {
        Self((n * GB as f64) as u64)
    }

    /// Creates a [`ByteSize`] from terabytes (SI, 1 TB = 1,000,000,000,000 bytes).
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_byte_fmt::ByteSize;
    ///
    /// let size = ByteSize::from_tb(1.0);
    /// assert_eq!(size.as_bytes(), 1_000_000_000_000);
    /// ```
    pub fn from_tb(n: f64) -> Self {
        Self((n * TB as f64) as u64)
    }

    /// Returns the raw byte count.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_byte_fmt::ByteSize;
    ///
    /// let size = ByteSize::from_bytes(42);
    /// assert_eq!(size.as_bytes(), 42);
    /// ```
    pub fn as_bytes(&self) -> u64 {
        self.0
    }

    /// Formats the byte size using binary units (KiB = 1024, MiB, GiB, TiB).
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_byte_fmt::ByteSize;
    ///
    /// assert_eq!(ByteSize::from_bytes(1_048_576).format_binary(), "1 MiB");
    /// assert_eq!(ByteSize::from_bytes(1_536).format_binary(), "1.5 KiB");
    /// ```
    pub fn format_binary(&self) -> String {
        format_value(self.0, true, None)
    }

    /// Returns a wrapper that formats with the given decimal precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_byte_fmt::ByteSize;
    ///
    /// let size = ByteSize::from_bytes(1_536_000);
    /// assert_eq!(size.with_precision(2).to_string(), "1.54 MB");
    /// assert_eq!(size.with_precision(0).to_string(), "2 MB");
    /// ```
    pub fn with_precision(&self, precision: usize) -> FormattedByteSize {
        FormattedByteSize {
            bytes: self.0,
            precision,
        }
    }
}

fn format_value(bytes: u64, binary: bool, precision: Option<usize>) -> String {
    let (kb, mb, gb, tb, units) = if binary {
        (
            KIB as f64,
            MIB as f64,
            GIB as f64,
            TIB as f64,
            ["B", "KiB", "MiB", "GiB", "TiB"],
        )
    } else {
        (
            KB as f64,
            MB as f64,
            GB as f64,
            TB as f64,
            ["B", "KB", "MB", "GB", "TB"],
        )
    };

    let b = bytes as f64;
    let (value, unit) = if b >= tb {
        (b / tb, units[4])
    } else if b >= gb {
        (b / gb, units[3])
    } else if b >= mb {
        (b / mb, units[2])
    } else if b >= kb {
        (b / kb, units[1])
    } else {
        (b, units[0])
    };

    match precision {
        Some(p) => {
            let formatted = format!("{value:.prec$}", prec = p);
            format!("{formatted} {unit}")
        }
        None => {
            // Auto precision: remove unnecessary trailing zeros
            if value.fract() == 0.0 {
                format!("{} {}", value as u64, unit)
            } else {
                // Format with enough decimals, then strip trailing zeros
                let s = format!("{value:.6}");
                let s = s.trim_end_matches('0');
                let s = s.trim_end_matches('.');
                format!("{s} {unit}")
            }
        }
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_value(self.0, false, None))
    }
}

impl fmt::Display for FormattedByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_value(self.bytes, false, Some(self.precision)))
    }
}

impl FromStr for ByteSize {
    type Err = ParseError;

    /// Parses a human-readable byte size string.
    ///
    /// Accepts formats like `"1.5 GB"`, `"1.5GB"`, `"500 KB"`, `"1024 bytes"`,
    /// `"1.5 GiB"` (binary). Parsing is case-insensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_byte_fmt::ByteSize;
    ///
    /// let size: ByteSize = "1.5 GB".parse().unwrap();
    /// assert_eq!(size.as_bytes(), 1_500_000_000);
    ///
    /// let size: ByteSize = "1.5GiB".parse().unwrap();
    /// assert_eq!(size.as_bytes(), 1_610_612_736);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ParseError::Empty);
        }

        // Find the boundary between the number and the unit
        let s_bytes = s.as_bytes();
        let mut split_pos = 0;
        for (i, &ch) in s_bytes.iter().enumerate() {
            if ch.is_ascii_digit() || ch == b'.' || ch == b'-' || ch == b'+' {
                split_pos = i + 1;
            } else {
                break;
            }
        }

        if split_pos == 0 {
            return Err(ParseError::InvalidNumber(s.to_string()));
        }

        let num_str = s[..split_pos].trim();
        let unit_str = s[split_pos..].trim().to_lowercase();

        let value: f64 = num_str
            .parse()
            .map_err(|_| ParseError::InvalidNumber(num_str.to_string()))?;

        let multiplier: f64 = match unit_str.as_str() {
            "" | "b" | "byte" | "bytes" => 1.0,
            "kb" | "kilobyte" | "kilobytes" => KB as f64,
            "mb" | "megabyte" | "megabytes" => MB as f64,
            "gb" | "gigabyte" | "gigabytes" => GB as f64,
            "tb" | "terabyte" | "terabytes" => TB as f64,
            "kib" | "kibibyte" | "kibibytes" => KIB as f64,
            "mib" | "mebibyte" | "mebibytes" => MIB as f64,
            "gib" | "gibibyte" | "gibibytes" => GIB as f64,
            "tib" | "tebibyte" | "tebibytes" => TIB as f64,
            _ => return Err(ParseError::InvalidUnit(unit_str)),
        };

        Ok(ByteSize((value * multiplier) as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_bytes() {
        assert_eq!(ByteSize::from_bytes(0).to_string(), "0 B");
        assert_eq!(ByteSize::from_bytes(1).to_string(), "1 B");
        assert_eq!(ByteSize::from_bytes(999).to_string(), "999 B");
    }

    #[test]
    fn format_kilobytes() {
        assert_eq!(ByteSize::from_bytes(1_000).to_string(), "1 KB");
        assert_eq!(ByteSize::from_bytes(1_500).to_string(), "1.5 KB");
        assert_eq!(ByteSize::from_bytes(999_999).to_string(), "999.999 KB");
    }

    #[test]
    fn format_megabytes() {
        assert_eq!(ByteSize::from_bytes(1_000_000).to_string(), "1 MB");
        assert_eq!(ByteSize::from_bytes(1_500_000).to_string(), "1.5 MB");
        assert_eq!(ByteSize::from_bytes(999_999_999).to_string(), "999.999999 MB");
    }

    #[test]
    fn format_gigabytes() {
        assert_eq!(ByteSize::from_bytes(1_000_000_000).to_string(), "1 GB");
        assert_eq!(ByteSize::from_bytes(1_500_000_000).to_string(), "1.5 GB");
    }

    #[test]
    fn format_terabytes() {
        assert_eq!(ByteSize::from_bytes(1_000_000_000_000).to_string(), "1 TB");
        assert_eq!(ByteSize::from_bytes(2_500_000_000_000).to_string(), "2.5 TB");
    }

    #[test]
    fn format_binary_units() {
        assert_eq!(ByteSize::from_bytes(0).format_binary(), "0 B");
        assert_eq!(ByteSize::from_bytes(1_024).format_binary(), "1 KiB");
        assert_eq!(ByteSize::from_bytes(1_536).format_binary(), "1.5 KiB");
        assert_eq!(ByteSize::from_bytes(1_048_576).format_binary(), "1 MiB");
        assert_eq!(ByteSize::from_bytes(1_073_741_824).format_binary(), "1 GiB");
        assert_eq!(ByteSize::from_bytes(1_099_511_627_776).format_binary(), "1 TiB");
    }

    #[test]
    fn precision_control() {
        let size = ByteSize::from_bytes(1_536_000);
        assert_eq!(size.with_precision(0).to_string(), "2 MB");
        assert_eq!(size.with_precision(1).to_string(), "1.5 MB");
        assert_eq!(size.with_precision(2).to_string(), "1.54 MB");
        assert_eq!(size.with_precision(3).to_string(), "1.536 MB");
    }

    #[test]
    fn parse_bytes() {
        assert_eq!("0 B".parse::<ByteSize>().unwrap().as_bytes(), 0);
        assert_eq!("100 B".parse::<ByteSize>().unwrap().as_bytes(), 100);
        assert_eq!("1024 bytes".parse::<ByteSize>().unwrap().as_bytes(), 1024);
        assert_eq!("512".parse::<ByteSize>().unwrap().as_bytes(), 512);
    }

    #[test]
    fn parse_si_units() {
        assert_eq!("1 KB".parse::<ByteSize>().unwrap().as_bytes(), 1_000);
        assert_eq!("1.5 KB".parse::<ByteSize>().unwrap().as_bytes(), 1_500);
        assert_eq!("500 KB".parse::<ByteSize>().unwrap().as_bytes(), 500_000);
        assert_eq!("1 MB".parse::<ByteSize>().unwrap().as_bytes(), 1_000_000);
        assert_eq!("1.5 GB".parse::<ByteSize>().unwrap().as_bytes(), 1_500_000_000);
        assert_eq!("1 TB".parse::<ByteSize>().unwrap().as_bytes(), 1_000_000_000_000);
    }

    #[test]
    fn parse_binary_units() {
        assert_eq!("1 KiB".parse::<ByteSize>().unwrap().as_bytes(), 1_024);
        assert_eq!("1 MiB".parse::<ByteSize>().unwrap().as_bytes(), 1_048_576);
        assert_eq!("1.5 GiB".parse::<ByteSize>().unwrap().as_bytes(), 1_610_612_736);
        assert_eq!("1 TiB".parse::<ByteSize>().unwrap().as_bytes(), 1_099_511_627_776);
    }

    #[test]
    fn parse_no_space() {
        assert_eq!("1.5GB".parse::<ByteSize>().unwrap().as_bytes(), 1_500_000_000);
        assert_eq!("500KB".parse::<ByteSize>().unwrap().as_bytes(), 500_000);
        assert_eq!("1.5GiB".parse::<ByteSize>().unwrap().as_bytes(), 1_610_612_736);
    }

    #[test]
    fn parse_case_insensitive() {
        assert_eq!("1.5 gb".parse::<ByteSize>().unwrap().as_bytes(), 1_500_000_000);
        assert_eq!("1.5 Gb".parse::<ByteSize>().unwrap().as_bytes(), 1_500_000_000);
        assert_eq!("1 kb".parse::<ByteSize>().unwrap().as_bytes(), 1_000);
        assert_eq!("1 mib".parse::<ByteSize>().unwrap().as_bytes(), 1_048_576);
    }

    #[test]
    fn parse_errors() {
        assert_eq!("".parse::<ByteSize>(), Err(ParseError::Empty));
        assert!(matches!(
            "abc".parse::<ByteSize>(),
            Err(ParseError::InvalidNumber(_))
        ));
        assert!(matches!(
            "1.5 XB".parse::<ByteSize>(),
            Err(ParseError::InvalidUnit(_))
        ));
    }

    #[test]
    fn round_trip_si() {
        let sizes = [0u64, 1, 999, 1_000, 1_500, 1_000_000, 1_500_000_000, 1_000_000_000_000];
        for &bytes in &sizes {
            let original = ByteSize::from_bytes(bytes);
            let formatted = original.to_string();
            let parsed: ByteSize = formatted.parse().unwrap();
            assert_eq!(original, parsed, "round-trip failed for {bytes} bytes: '{formatted}'");
        }
    }

    #[test]
    fn ordering() {
        let a = ByteSize::from_bytes(100);
        let b = ByteSize::from_bytes(200);
        let c = ByteSize::from_bytes(200);
        assert!(a < b);
        assert!(b > a);
        assert_eq!(b, c);

        let mut sizes = vec![
            ByteSize::from_bytes(1_000_000),
            ByteSize::from_bytes(100),
            ByteSize::from_bytes(1_000),
        ];
        sizes.sort();
        assert_eq!(sizes[0].as_bytes(), 100);
        assert_eq!(sizes[1].as_bytes(), 1_000);
        assert_eq!(sizes[2].as_bytes(), 1_000_000);
    }

    #[test]
    fn convenience_constructors() {
        assert_eq!(ByteSize::from_kb(1.5).as_bytes(), 1_500);
        assert_eq!(ByteSize::from_mb(2.5).as_bytes(), 2_500_000);
        assert_eq!(ByteSize::from_gb(1.0).as_bytes(), 1_000_000_000);
        assert_eq!(ByteSize::from_tb(1.0).as_bytes(), 1_000_000_000_000);
    }

    #[test]
    fn edge_case_zero() {
        let zero = ByteSize::from_bytes(0);
        assert_eq!(zero.to_string(), "0 B");
        assert_eq!(zero.format_binary(), "0 B");
        assert_eq!(zero.as_bytes(), 0);
    }

    #[test]
    fn edge_case_u64_max() {
        let max = ByteSize::from_bytes(u64::MAX);
        // Should format as TB without panicking
        let formatted = max.to_string();
        assert!(formatted.contains("TB"), "u64::MAX should format as TB, got: {formatted}");
        assert_eq!(max.as_bytes(), u64::MAX);
    }

    #[test]
    fn hash_works() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ByteSize::from_bytes(100));
        set.insert(ByteSize::from_bytes(100));
        set.insert(ByteSize::from_bytes(200));
        assert_eq!(set.len(), 2);
    }
}
