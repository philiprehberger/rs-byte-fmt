# Changelog

## 0.1.7 (2026-03-20)

- Add arithmetic operators (Add, Sub, Mul, Div and their Assign variants) for ByteSize
- Add unit conversion methods: to_kb(), to_mb(), to_gb(), to_tb()
- Add From<u64> implementation for ByteSize
- Add #[must_use] attributes on constructor and formatting methods

## 0.1.6 (2026-03-17)

- Add readme, rust-version, documentation to Cargo.toml
- Add Development section to README
## 0.1.5 (2026-03-16)

- Update install snippet to use full version

## 0.1.4 (2026-03-16)

- Add README badges
- Synchronize version across Cargo.toml, README, and CHANGELOG

## 0.1.0 (2026-03-15)

- Initial release
- `ByteSize` struct with formatting and parsing
- SI units (KB, MB, GB, TB) and binary units (KiB, MiB, GiB, TiB)
- Configurable decimal precision
- Convenience constructors (`from_kb`, `from_mb`, `from_gb`, `from_tb`)
