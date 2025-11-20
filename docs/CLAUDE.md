# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**php-rs-toon** is a Rust-based PHP extension providing high-performance encoding/decoding for the TOON (Token-Oriented Object Notation) data format. It exposes two PHP functions (`toon_encode()` and `toon_decode()`) that interact with a Rust backend for fast serialization and parsing.

## Build & Test Commands

### Build
```bash
cargo build --release          # Release build (optimized)
cargo build                    # Debug build
```

Output: `target/release/libphp_rs_toon.so` (Linux) or `target/release/libphp_rs_toon.dylib` (macOS)

### Testing
```bash
cargo test                     # Run Rust unit tests
php -d extension=target/release/libphp_rs_toon.so test.php  # Run PHP integration tests
```

### Docker Verification (Clean Environment)
```bash
docker build -t php-rs-toon-debug .
docker run --rm -v $(pwd):/app php-rs-toon-debug bash -c "cargo build --release && php -d extension=target/release/libphp_rs_toon.so test.php"
```

### Lint
The project follows Rust standard conventions. Use:
```bash
cargo clippy --release        # Linting
cargo fmt                     # Code formatting
```

## Architecture Overview

### Core Components

**src/lib.rs** (139 lines) - PHP FFI Bridge
- Exports two functions: `toon_encode(Zval)` and `toon_decode(String)`
- Handles type conversion between PHP arrays (Zval) and internal ToonValue representation
- Detection logic: sequential integer keys → TOON Arrays; otherwise → TOON Maps
- Uses `#[php_function]` macros for FFI bindings

**src/toon.rs** (243 lines) - TOON Parser & Encoder
- Core `ToonValue` enum: Null, Bool, Int, Float, String, Array, Map
- `parse()` function: YAML-like indentation-based syntax, handles quoted strings with escaping
- `encode()` function: Converts ToonValue to TOON format with proper indentation
- Supports inline lists, special character escaping, nested structures

### Data Format

TOON is an indentation-based notation similar to YAML:
- Scalars: `key: value` or inline (for primitives)
- Maps (associative arrays): Key-value pairs with indentation
- Arrays (sequential): Comma-separated in inline format or as list items
- Supports: null, booleans, integers, floats, quoted strings with escapes

### Key Design Decisions

1. **Order Preservation**: Uses `Vec<(String, ToonValue)>` instead of HashMap to preserve insertion order (consistent with PHP 7.1+)
2. **Type Conversion**: PHP detects array structure (sequential vs associative) to map to appropriate TOON construct
3. **Error Handling**: Uses `anyhow::Result<T>` in Rust; errors converted to PHP exceptions automatically
4. **Module System**: Uses `ext-php-rs` crate for Rust-PHP FFI boilerplate

## Dependencies

- `ext-php-rs = "0.15.1"` - PHP extension framework for Rust
- `anyhow = "1.0"` - Error handling

## Project Structure

```
.
├── Cargo.toml              # Rust manifest (dependencies, package config)
├── Cargo.lock              # Lockfile for reproducible builds
├── Dockerfile              # Clean build environment (PHP 8.2 + Rust)
├── README.md               # User-facing documentation
├── test.php                # PHP integration test suite (7 test cases)
├── expanded.rs             # Generated macro expansion (don't edit)
└── src/
    ├── lib.rs              # PHP FFI bindings & core logic
    └── toon.rs             # TOON parser & encoder implementation
```

## Development Notes

### Adding New Features
- Parser logic modifications → src/toon.rs (parse function)
- Encoder/formatting changes → src/toon.rs (encode function)
- PHP API additions → src/lib.rs (new #[php_function] exports)
- Type conversions → src/lib.rs (php_array_to_toon_value, toon_value_to_php_value)

### Testing Pattern
- **Rust unit tests**: Use `#[test]` in toon.rs
- **Integration tests**: Add PHP test cases to test.php
- Always test round-trip: encode → decode (or vice versa) to ensure consistency

### Building for Production
1. Use `cargo build --release` for optimized builds
2. Copy `.so`/`.dylib` to PHP extension directory
3. Add `extension=libphp_rs_toon.so` to php.ini
4. Verify with: `php -m | grep php_rs_toon`

### Common Development Tasks

**Modify TOON parsing logic**:
- Edit `parse()` function in src/toon.rs
- Add corresponding tests in toon.rs `#[cfg(test)]` module
- Run: `cargo test` to validate

**Add PHP test case**:
- Edit test.php and add new test case
- Run: `php -d extension=target/release/libphp_rs_toon.so test.php`

**Debug parsing issues**:
- Add println! macros in src/toon.rs
- Rebuild: `cargo build`
- Run tests with output

## Cargo.toml Configuration

Key settings:
- `edition = "2021"`
- `crate-type = ["cdylib"]` - Builds shared library for PHP
- PHP extension metadata (name, version) in `[package]`

## Version & Status

- Current Version: 0.1.0
- Status: Active development
- Latest features: Enhanced string quoting/escaping, inline list parsing, expanded test suite
