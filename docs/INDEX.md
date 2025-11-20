# Documentation Index

Complete guide to all php-rs-toon documentation.

## 📁 Documentation Structure

```
php-rs-toon/
├── START_HERE.md          # 👈 Navigation guide (start here!)
├── README.md              # Project overview
├── QUICKSTART.md          # Quick installation guide
├── docs/                  # Detailed documentation
│   ├── INDEX.md          # This file
│   ├── INSTALLATION.md   # Installation guide
│   ├── USAGE.md          # Usage guide with examples
│   ├── API_REFERENCE.md  # Complete API documentation
│   ├── BENCHMARKS.md     # Performance benchmarks
│   ├── BUILDING.md       # Building from source
│   ├── DOCKER_GUIDE.md   # Docker deployment
│   ├── FAQ.md            # Frequently asked questions
│   ├── TROUBLESHOOTING.md # Common issues and solutions
│   └── CONTRIBUTING.md   # Contribution guidelines
├── examples/             # Code examples
│   ├── README.md
│   ├── basic-encode.php
│   ├── nested-structures.php
│   └── llm-optimization.php
└── benchmark/            # Performance testing
    ├── benchmarks/
    └── run-benchmarks.sh
```

## 📖 Documentation By Category

### Getting Started
- **[START_HERE.md](../START_HERE.md)** - Navigation and quick links
- **[README.md](../README.md)** - Project overview and features
- **[QUICKSTART.md](../QUICKSTART.md)** - Get started in 5 minutes
- **[INSTALLATION.md](INSTALLATION.md)** - Detailed installation instructions

### Usage & Reference
- **[USAGE.md](USAGE.md)** - Comprehensive usage guide
- **[API_REFERENCE.md](API_REFERENCE.md)** - Complete API documentation
- **[examples/](../examples/)** - Working code examples

### Performance & Testing
- **[BENCHMARKS.md](BENCHMARKS.md)** - Performance comparisons
- **[benchmark/](../benchmark/)** - Benchmark suite

### Development
- **[BUILDING.md](BUILDING.md)** - Build from source
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute
- **[DOCKER_GUIDE.md](DOCKER_GUIDE.md)** - Docker deployment

### Help & Support
- **[FAQ.md](FAQ.md)** - Frequently asked questions
- **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)** - Common issues

## 🔍 Quick Reference

### Installation Methods
| Method | Document | Best For |
|--------|----------|----------|
| Docker (Pre-built) | [QUICKSTART.md](../QUICKSTART.md) | Quick testing |
| Manual Install | [INSTALLATION.md](INSTALLATION.md) | Production use |
| Build from Source | [BUILDING.md](BUILDING.md) | Development |

### Common Use Cases
| Task | Reference |
|------|-----------|
| Encode PHP array to TOON | [USAGE.md#encoding](USAGE.md#encoding) |
| Decode TOON string | [USAGE.md#decoding](USAGE.md#decoding) |
| Handle nested structures | [examples/nested-structures.php](../examples/nested-structures.php) |
| Optimize for LLMs | [examples/llm-optimization.php](../examples/llm-optimization.php) |
| Compare performance | [BENCHMARKS.md](BENCHMARKS.md) |

## 📝 Document Summaries

### START_HERE.md
Navigation hub with links to all documentation. Best starting point for new users.

### README.md
Project overview, key features, and quick examples. Read this first to understand what php-rs-toon does.

### QUICKSTART.md
Fast-track guide to get the extension running using Docker. Includes verification steps.

### INSTALLATION.md
Complete installation guide covering all methods: Docker, manual, and building from source.

### USAGE.md
Comprehensive guide with 7+ examples covering basic types, nested structures, arrays, special characters, error handling, and framework integration.

### API_REFERENCE.md
Complete technical reference for `toon_encode()` and `toon_decode()` functions, including parameters, return values, and error handling.

### BENCHMARKS.md
Performance comparison between php-rs-toon (Rust) and toon-php (pure PHP). Includes methodology, results, and real-world impact analysis.

### BUILDING.md
Instructions for building the extension from source, including dependencies, compilation, and troubleshooting.

### DOCKER_GUIDE.md
Docker deployment strategies, production setup, and orchestration examples.

### FAQ.md
Answers to common questions about installation, usage, performance, and troubleshooting.

### TROUBLESHOOTING.md
Solutions to common problems with installation, extension loading, and runtime errors.

### CONTRIBUTING.md
Guidelines for contributing code, reporting bugs, and submitting pull requests.

## 🔗 External Resources

- [Rust Documentation](https://www.rust-lang.org/learn)
- [ext-php-rs Documentation](https://docs.rs/ext-php-rs/latest/ext_php_rs/)
- [TOON Format Specification](https://github.com/HelgeSverre/toon-php)
- [PHP Extension Development](https://www.php.net/manual/en/internals2.php)

---

**Navigation**: [← Back to START_HERE](../START_HERE.md) | [Quick Start →](../QUICKSTART.md)
