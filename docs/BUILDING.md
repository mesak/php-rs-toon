# Building from Source

Guide to building the php-rs-toon extension from source.

> **[← Back to Benchmarks](BENCHMARKS.md)** | **[Index](INDEX.md)**

## Prerequisites

### Required
- **Rust** - Latest stable (1.70+)
- **PHP 8.0+** with development headers
- **Clang** - For bindgen
- **Git** - To clone repository

### Platform-Specific

**Ubuntu/Debian**:
```bash
sudo apt install php8.2-dev clang libclang-dev
```

**macOS**:
```bash
brew install php@8.2
xcode-select --install
```

**CentOS/RHEL**:
```bash
sudo yum install php-devel clang
```

## Quick Build

```bash
# Clone repository
git clone https://github.com/mesak/php-rs-toon.git
cd php-rs-toon

# Build release version
cargo build --release

# Install
sudo cp target/release/libphp_rs_toon.so $(php-config --extension-dir)/
echo "extension=libphp_rs_toon.so" | sudo tee -a $(php --ini | grep "Loaded Configuration" | awk '{print $4}')

# Verify
php -m | grep php-rs-toon
```

## Detailed Steps

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

### 2. Install PHP Development Files

Find your PHP version:
```bash
php -v
```

Install corresponding dev package:
```bash
# Ubuntu/Debian
sudo apt install php8.2-dev

# Verify
php-config --version
```

### 3. Clone Repository

```bash
git clone https://github.com/mesak/php-rs-toon.git
cd php-rs-toon
```

### 4. Build Extension

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized)
cargo build --release
```

Output location:
- Debug: `target/debug/libphp_rs_toon.so`
- Release: `target/release/libphp_rs_toon.so`

### 5. Install Extension

```bash
# Find extension directory
EXT_DIR=$(php-config --extension-dir)
echo "Extension directory: $EXT_DIR"

# Copy extension
sudo cp target/release/libphp_rs_toon.so $EXT_DIR/

# Verify file
ls -l $EXT_DIR/libphp_rs_toon.so
```

### 6. Enable Extension

Find php.ini location:
```bash
php --ini | grep "Loaded Configuration"
```

Add extension:
```bash
INI_FILE=$(php --ini | grep "Loaded Configuration" | awk '{print $4}')
echo "extension=libphp_rs_toon.so" | sudo tee -a $INI_FILE
```

For PHP-FPM, also update FPM configuration:
```bash
echo "extension=libphp_rs_toon.so" | sudo tee -a /etc/php/8.2/fpm/php.ini
sudo systemctl restart php8.2-fpm
```

### 7. Verify Installation

```bash
# Check module loaded
php -m | grep php-rs-toon

# Check functions available
php -r "var_dump(function_exists('toon_encode'));"

# Run test
php test.php
```

## Development Build

### Debug Mode

```bash
cargo build
```

Benefits:
- Faster compilation
- Debug symbols included
- Better error messages

### Running Tests

```bash
# Rust unit tests
cargo test

# PHP integration tests
php test.php
```

### Code Formatting

```bash
cargo fmt
cargo clippy
```

## Cross-Compilation

### Building for Different PHP Versions

```bash
# Set PHP version
export PHP_VERSION=8.2

# Build
cargo clean
cargo build --release
```

### Creating Release Artifacts

```bash
# Build optimized release
cargo build --release --target x86_64-unknown-linux-gnu

# Strip symbols (reduce size)
strip target/x86_64-unknown-linux-gnu/release/libphp_rs_toon.so

# Create tarball
tar -czf libphp_rs_toon-linux-x86_64.tar.gz \
  -C target/x86_64-unknown-linux-gnu/release \
  libphp_rs_toon.so
```

## Troubleshooting

### Common Build Errors

#### "php-config not found"

**Problem**: PHP development files not installed

**Solution**:
```bash
# Ubuntu/Debian
sudo apt install php8.2-dev

# Verify
which php-config
```

#### "clang: command not found"

**Problem**: Clang not installed (required for bindgen)

**Solution**:
```bash
# Ubuntu/Debian
sudo apt install clang libclang-dev

# macOS
xcode-select --install
```

#### "error: linking with `cc` failed"

**Problem**: Missing C compiler or linker

**Solution**:
```bash
# Ubuntu/Debian
sudo apt install build-essential

# macOS
xcode-select --install
```

#### PHP version mismatch

**Problem**: Extension compiled for different PHP version

**Solution**: Rebuild targeting your PHP version:
```bash
cargo clean
cargo build --release
```

### Build Optimization

#### Release Profile

Edit `Cargo.toml`:
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

#### Link-Time Optimization

```bash
cargo build --release
# or
RUSTFLAGS="-C lto=fat" cargo build --release
```

## Testing Your Build

### Unit Tests

```bash
cargo test --lib
```

### Integration Tests

```bash
php test.php
```

### Benchmark

```bash
cd benchmark
composer install
./run-benchmarks.sh
```

## See Also

- **[INSTALLATION.md](INSTALLATION.md)** - Installation guide
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guidelines
- **[Cargo Documentation](https://doc.rust-lang.org/cargo/)** - Cargo reference

---

**Navigation**: [← Benchmarks](BENCHMARKS.md) | [Main README](../README.md) | [Docker Guide →](DOCKER_GUIDE.md)
