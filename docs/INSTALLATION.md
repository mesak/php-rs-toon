# Installation Guide

Complete guide to installing the php-rs-toon extension.

> **[← Back to Documentation Index](INDEX.md)** | **[Quick Start →](../QUICKSTART.md)**

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Docker Installation (Recommended)](#docker-installation-recommended)
3. [Manual Installation](#manual-installation)
4. [Building from Source](#building-from-source)
5. [Verification](#verification)
6. [Troubleshooting](#troubleshooting)

## Prerequisites

- **PHP 8.0+** (PHP 8.2 recommended)
- **Operating System**: Linux, macOS, or Windows (WSL2)

## Docker Installation (Recommended)

The easiest way to get started. See **[QUICKSTART.md](../QUICKSTART.md)** for details.

### Quick Setup

```bash
# Build production image
docker build -f Dockerfile.prod -t php-rs-toon:8.2 .

# Verify installation
docker run --rm php-rs-toon:8.2 php -r "var_dump(function_exists('toon_encode'));"
```

## Manual Installation

### Step 1: Download Extension

Download the pre-built `.so` file from the [latest release](https://github.com/mesak/php-rs-toon/releases):

```bash
curl -L https://github.com/mesak/php-rs-toon/releases/download/v1.0.0/php-rs-toon-linux-x86_64.zip \
  -o extension.zip
unzip extension.zip
```

### Step 2: Find Extension Directory

```bash
php-config --extension-dir
# Output: /usr/lib/php/20220829 (example)
```

### Step 3: Install Extension

```bash
sudo cp libphp_rs_toon.so $(php-config --extension-dir)/
```

### Step 4: Enable Extension

Add to your `php.ini`:

```bash
echo "extension=libphp_rs_toon.so" | sudo tee -a /etc/php/8.2/cli/php.ini
```

Or for PHP-FPM:
```bash
echo "extension=libphp_rs_toon.so" | sudo tee -a /etc/php/8.2/fpm/php.ini
sudo systemctl restart php8.2-fpm
```

## Building from Source

See **[BUILDING.md](BUILDING.md)** for complete build instructions.

### Quick Build

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Install
sudo cp target/release/libphp_rs_toon.so $(php-config --extension-dir)/
```

## Verification

### Check Extension Loaded

```bash
php -m | grep php-rs-toon
# Should output: php-rs-toon
```

### Check Functions Available

```bash
php -r "var_dump(function_exists('toon_encode'), function_exists('toon_decode'));"
# Should output: bool(true) bool(true)
```

### Run Test Script

```php
<?php
// test.php
$data = ['test' => 'success', 'value' => 123];
echo toon_encode($data);
```

```bash
php test.php
# Output:
# test: success
# value: 123
```

## Troubleshooting

### Extension Not Loading

**Problem**: `PHP Warning: PHP Startup: Unable to load dynamic library 'libphp_rs_toon.so'`

**Solution**: Check file permissions and location
```bash
ls -l $(php-config --extension-dir)/libphp_rs_toon.so
# Should be readable: -rw-r--r--

# If not found, verify you copied to correct directory
```

### Function Not Found

**Problem**: `Call to undefined function toon_encode()`

**Solution**: Ensure extension is enabled in php.ini
```bash
php --ini | grep "php.ini"
# Check all listed INI files for: extension=libphp_rs_toon.so
```

### Wrong PHP Version

**Problem**: Extension compiled for different PHP version

**Solution**: Build from source for your specific PHP version (see [BUILDING.md](BUILDING.md))

## Platform-Specific Notes

### Ubuntu/Debian

```bash
# Install PHP development files
sudo apt-get install php8.2-dev

# PHP-FPM config location
/etc/php/8.2/fpm/php.ini
```

### CentOS/RHEL

```bash
# Install PHP development files
sudo yum install php-devel

# PHP-FPM config location
/etc/php-fpm.d/www.conf
```

### macOS

```bash
# Install PHP via Homebrew
brew install php@8.2

# Extension directory
$(brew --prefix)/lib/php/pecl/20220829/
```

## Next Steps

- **[USAGE.md](USAGE.md)** - Learn how to use the extension
- **[examples/](../examples/)** - See working examples
- **[FAQ.md](FAQ.md)** - Common questions

---

**Navigation**: [← Index](INDEX.md) | [USAGE.md →](USAGE.md)
