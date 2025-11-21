# PHP TOON Extension (Rust)

**Languages**: [English](#) | [繁體中文](README.zh_TW.md)

---

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-2021-orange?style=flat-square)](https://www.rust-lang.org/)
[![PHP](https://img.shields.io/badge/PHP-8.0%2B-777BB4?style=flat-square)](https://www.php.net/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)

A blazing-fast PHP extension for encoding and decoding [TOON (Token-Oriented Object Notation)](https://github.com/HelgeSverre/toon-php) format, built with Rust for maximum performance and safety.

</div>

---

## ✨ Features

- **⚡ Lightning-fast Performance** – Rust-powered for unparalleled speed
- **🔄 Bidirectional Support** – `toon_encode()` and `toon_decode()`
- **🎯 Smart Type Detection** – Auto-detects arrays vs. associative maps
- **📍 Order Preservation** – Maintains insertion order
- **🔐 Type-Safe** – Memory-safe with zero unsafe code

---

## 📦 Installation

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PHP development headers
sudo apt install php8.2-dev clang  # Ubuntu/Debian
# or
brew install php clang              # macOS
```

### Build & Install

```bash
# Clone repository
git clone https://github.com/mesak/php-rs-toon.git
cd php-rs-toon

# Build release version
cargo build --release

# Install extension
sudo cp target/release/libphp_rs_toon.so $(php-config --extension-dir)/

# Enable extension
echo "extension=libphp_rs_toon.so" | sudo tee -a $(php-config --ini-path)/20-toon.ini

# Verify installation
php -m | grep php_rs_toon
```

---

## 🔧 Development

### Build for Development

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# Format code
cargo fmt

# Check code quality
cargo clippy --release
```

### Build with Docker

```bash
# Build test environment
docker build -f Dockerfile.test -t php-rs-toon:test .

# Build production
docker build -f Dockerfile.prod -t php-rs-toon:prod .
```

---

## 🧪 Testing

### Run Rust Unit Tests

```bash
cargo test
```

### Run PHP Integration Tests

```bash
# Using installed extension
php test.php

# Using built extension (without installation)
php -d extension=target/release/libphp_rs_toon.so test.php

# Docker testing
docker build -f Dockerfile.test -t php-rs-toon:test .
docker run --rm php-rs-toon:test
```

---

## ⚡ Performance Testing

### Quick Benchmark

```bash
# Single performance test
php -d extension=target/release/libphp_rs_toon.so perf-test.php

# Comparison with pure PHP implementation
php -d extension=target/release/libphp_rs_toon.so perf-compare.php
```

### Full Benchmark Suite

```bash
cd benchmark
composer install
./run-benchmarks.sh

# Docker benchmarking
docker build -f Dockerfile.benchmark -t php-rs-toon:bench .
docker run --rm php-rs-toon:bench
```

**Performance Results**:
- **10-30x faster** than pure PHP implementation
- **Optimized memory usage** with pre-allocation
- **Recursion depth protection** (max depth: 100)

---

## 💡 Usage Examples

### Basic Encoding

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

$toon = toon_encode($data);
echo $toon;
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

### Basic Decoding

```php
<?php

$toon = <<<'TOON'
user:
  id: 123
  name: Alice
  tags: 1, 2, 3
TOON;

$data = toon_decode($toon);
print_r($data);
```

**Output:**
```
Array
(
    [user] => Array
        (
            [id] => 123
            [name] => Alice
            [tags] => Array
                (
                    [0] => 1
                    [1] => 2
                    [2] => 3
                )
        )
)
```

### Nested Structures

```php
<?php

$data = [
    "company" => [
        "name" => "TechCorp",
        "departments" => [
            ["name" => "Engineering", "employees" => 50],
            ["name" => "Sales", "employees" => 30],
        ],
        "metadata" => [
            "founded" => 2020,
            "public" => false
        ]
    ]
];

$toon = toon_encode($data);
$decoded = toon_decode($toon);

assert($data === $decoded); // Round-trip consistency
```

### Error Handling

```php
<?php

try {
    $result = toon_decode("invalid: : syntax");
} catch (Exception $e) {
    echo "Parse error: " . $e->getMessage();
}
```

### More Examples

See [`examples/`](examples/) directory:
- [`basic-encode.php`](examples/basic-encode.php) - Simple encoding
- [`nested-structures.php`](examples/nested-structures.php) - Complex nested data
- [`llm-optimization.php`](examples/llm-optimization.php) - LLM-friendly formatting

---

## 📚 API Reference

### `toon_encode(mixed $data): string`

Encodes PHP data into TOON format string.

**Parameters:**
- `$data` - PHP value (array, string, int, float, bool, null)

**Returns:** TOON formatted string

**Throws:** Exception on recursion depth limit (>100)

---

### `toon_decode(string $toon): mixed`

Decodes TOON string into PHP data.

**Parameters:**
- `$toon` - TOON formatted string

**Returns:** PHP value (array, string, int, float, bool, null)

**Throws:** Exception on parse error

---

## 🏗️ Project Structure

```
php-rs-toon/
├── src/
│   ├── lib.rs              # PHP FFI bridge
│   └── toon.rs             # TOON parser & encoder
├── examples/               # Usage examples
├── benchmark/              # Performance benchmarks
├── test.php                # Integration tests
├── perf-test.php           # Quick performance test
├── perf-compare.php        # Rust vs PHP comparison
├── Cargo.toml              # Rust dependencies
└── Dockerfile.*            # Docker configurations
```

---

## 🤝 Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `cargo fmt && cargo clippy && cargo test`
5. Submit pull request

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details

---

## 🔗 Resources

- [TOON Format Specification](https://github.com/HelgeSverre/toon-php)
- [Rust ext-php-rs Documentation](https://docs.rs/ext-php-rs/)
- [PHP Extension Development](https://www.php.net/manual/en/internals2.php)

---

**Languages**: [English](#) | [繁體中文](README.zh_TW.md)
