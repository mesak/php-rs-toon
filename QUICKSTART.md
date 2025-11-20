# Quick Start Guide

## Using Pre-built Extension from GitHub Release

The fastest way to get started is using the pre-built extension from the GitHub release.

### 1. Production Docker Setup (Recommended)

```bash
# Build the production image
docker build -f Dockerfile.prod -t php-rs-toon:8.2 .

# Test the extension
docker run --rm php-rs-toon:8.2 php -r "echo toon_encode(['test' => 'works']);"
```

### 2. Run Examples

```bash
# Basic encoding example
docker run --rm -v $(pwd)/examples:/examples php-rs-toon:8.2 \
  php /examples/basic-encode.php

# Nested structures
docker run --rm -v $(pwd)/examples:/examples php-rs-toon:8.2 \
  php /examples/nested-structures.php

# LLM optimization demo
docker run --rm -v $(pwd)/examples:/examples php-rs-toon:8.2 \
  php /examples/llm-optimization.php
```

### 3. Run Benchmarks

```bash
# Build benchmark image
docker build -f Dockerfile.benchmark -t php-rs-toon-bench .

# Run benchmarks
docker run --rm -v $(pwd)/benchmark:/app/results php-rs-toon-bench

# View results
cat benchmark/results.md
```

## Manual Installation

### Download Extension

```bash
# Download from GitHub release
curl -L https://github.com/mesak/php-rs-toon/releases/download/v1.0.0/libphp_rs_toon.so \
  -o libphp_rs_toon.so

# Copy to PHP extension directory
sudo cp libphp_rs_toon.so $(php-config --extension-dir)/

# Enable in php.ini
echo "extension=libphp_rs_toon.so" | sudo tee -a $(php -i | grep "php.ini" | head -1 | awk '{print $5}')

# Verify
php -m | grep php-rs-toon
```

## Quick Test

```php
<?php
// test.php
$data = ['name' => 'Test', 'value' => 123];
echo toon_encode($data);
```

```bash
php test.php
# Output:
# name: Test
# value: 123
```

## Next Steps

- **Complete Guide**: See [USAGE.md](USAGE.md) for detailed examples
- **Performance**: Check [BENCHMARKS.md](BENCHMARKS.md) for benchmark results
- **Examples**: Explore [examples/](examples/) directory
- **Building**: See [README.md](README.md) for building from source
