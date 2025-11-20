# Examples

This directory contains practical examples demonstrating various use cases of the php-rs-toon extension.

## Running Examples

### Using Docker

```bash
# Build the production image
docker build -f Dockerfile.prod -t php-rs-toon:8.2 .

# Run an example
docker run --rm -v $(pwd)/examples:/examples php-rs-toon:8.2 php /examples/basic-encode.php
```

### Local Installation

```bash
# Ensure extension is loaded
php -m | grep php-rs-toon

# Run examples
php examples/basic-encode.php
php examples/nested-structures.php
php examples/llm-optimization.php
```

## Available Examples

### 1. basic-encode.php
Simple demonstration of encoding a PHP array to TOON format.

**Concepts**: Basic encoding, output comparison

### 2. nested-structures.php
Shows how to work with deeply nested data structures.

**Concepts**: Nested encoding/decoding, round-trip verification

### 3. llm-optimization.php
Demonstrates token savings when using TOON vs JSON for LLM APIs.

**Concepts**: Token optimization, cost reduction, size comparison

## Quick Reference

```php
// Encoding
$toon = toon_encode($phpArray);

// Decoding
$array = toon_decode($toonString);

// Round-trip
$original = ['key' => 'value'];
$roundTrip = toon_decode(toon_encode($original));
assert($original === $roundTrip); // true
```

## Next Steps

- See [USAGE.md](../USAGE.md) for detailed usage guide
- Check [BENCHMARKS.md](../BENCHMARKS.md) for performance data
- Read [README.md](../README.md) for installation instructions
