# PHP TOON Extension (Rust)

**Languages**: [English](#) | [繁體中文](README.zh_TW.md)

---

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-2021-orange?style=flat-square)](https://www.rust-lang.org/)
[![PHP](https://img.shields.io/badge/PHP-8.0%2B-777BB4?style=flat-square)](https://www.php.net/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)
[![Language: English | 繁體中文](https://img.shields.io/badge/Language-English%20%7C%20%E7%B9%81%E9%AB%94%E4%B8%AD%E6%96%87-blue?style=flat-square)](#languages)

A blazing-fast PHP extension for encoding and decoding [TOON (Token-Oriented Object Notation)](https://github.com/HelgeSverre/toon-php) format, built with Rust for maximum performance and safety.

[English](#english) • [繁體中文](#languages)

</div>

---

## <a id="english"></a>English

### ✨ Features

- **⚡ Lightning-fast Performance** – Crafted in Rust for unparalleled speed and safety
- **🔄 Full Bidirectional Support** – `toon_encode()` and `toon_decode()` for seamless conversion
- **🎯 Smart Type Detection** – Automatically distinguishes between sequential arrays and associative maps
- **📍 Order Preservation** – Maintains insertion order for associative arrays (PHP 7.1+ native array behavior)
- **🔐 Type-Safe** – Memory-safe with zero unsafe code in the critical path

### 📋 Requirements

- **Rust** – Latest stable version
- **PHP** – 8.0 or higher
- **php-config** – Included in `php-dev` or `php-devel` package
- **Clang** – Required for `bindgen`

### 📚 Documentation

**👉 New to php-rs-toon? Start here:** **[START_HERE.md](START_HERE.md)**

Complete documentation available:

- **[QUICKSTART.md](QUICKSTART.md)** – Get started in 5 minutes
- **[docs/INSTALLATION.md](docs/INSTALLATION.md)** – Complete installation guide
- **[docs/USAGE.md](docs/USAGE.md)** – Usage guide with examples
- **[docs/API_REFERENCE.md](docs/API_REFERENCE.md)** – Full API documentation
- **[docs/BENCHMARKS.md](docs/BENCHMARKS.md)** – Performance comparisons
- **[docs/INDEX.md](docs/INDEX.md)** – Complete documentation index

📂 **[Browse all documentation →](docs/)**

### 🚀 Quick Start

#### Building

```bash
# Clone and navigate
git clone <repository_url>
cd php-rs-toon

# Build optimized release
cargo build --release
```

Output: `target/release/libphp_rs_toon.so` (Linux) or `target/release/libphp_rs_toon.dylib` (macOS)

#### Installation

```bash
# Find PHP extension directory
php-config --extension-dir

# Copy the built extension (example for Linux)
cp target/release/libphp_rs_toon.so $(php-config --extension-dir)/

# Enable in php.ini
echo "extension=libphp_rs_toon.so" >> /etc/php/8.2/cli/php.ini
```

### 💡 Usage Examples

#### Basic Encoding

```php
<?php

$data = [
    "user" => [
        "id" => 123,
        "email" => "ada@example.com",
        "metadata" => [
            "active" => true,
            "score" => 9.5
        ]
    ]
];

$toonString = toon_encode($data);
echo $toonString;
```

**Output:**
```
user:
  id: 123
  email: ada@example.com
  metadata:
    active: true
    score: 9.5
```

#### Basic Decoding

```php
<?php

$toonString = <<<'TOON'
user:
  id: 123
  email: ada@example.com
TOON;

$array = toon_decode($toonString);
var_dump($array);
```

### 🐳 Docker Verification

Test the extension in an isolated environment:

```bash
# Build container
docker build -t php-rs-toon-test .

# Run tests
docker run --rm -v $(pwd):/app php-rs-toon-test \
  bash -c "cargo build --release && php -d extension=target/release/libphp_rs_toon.so test.php"
```

### 📚 Project Structure

```
php-rs-toon/
├── Cargo.toml              # Rust package manifest
├── Cargo.lock              # Reproducible builds
├── Dockerfile              # Clean build environment
├── README.md               # This file (English)
├── README.zh_TW.md         # 繁體中文文檔
├── test.php                # Integration test suite
├── expanded.rs             # Generated macro expansion
└── src/
    ├── lib.rs              # PHP FFI bindings
    └── toon.rs             # Parser & encoder
```

### 🤝 Contributing

Contributions are welcome! Please ensure:

- Code follows Rust conventions (`cargo fmt`, `cargo clippy`)
- Tests pass (`cargo test`)
- PHP integration tests work

### 📄 License

MIT – See [LICENSE](LICENSE) for details

---

## <a id="languages"></a>Languages

- **English** – This file
- **[繁體中文](README.zh_TW.md)** – Traditional Chinese documentation
