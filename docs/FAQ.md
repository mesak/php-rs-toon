# Frequently Asked Questions

Common questions about php-rs-toon.

> **[← Back to Docker Guide](DOCKER_GUIDE.md)** | **[Index](INDEX.md)**

## General Questions

### What is TOON?

TOON (Token-Oriented Object Notation) is a compact data format designed to reduce token consumption when sending structured data to Large Language Models (LLMs). It uses indentation-based syntax similar to YAML but more concise.

### Why use php-rs-toon instead of pure PHP?

The Rust implementation offers:
- **5-20x faster** encoding/decoding
- **75% less** memory usage
- **Native performance** through compiled code
- **Type safety** at the extension level

See [BENCHMARKS.md](BENCHMARKS.md) for detailed comparisons.

### Is it production-ready?

Yes. The extension is:
- ✅ Thoroughly tested
- ✅ Type-safe (Rust's memory safety)
- ✅ Battle-tested in production environments
- ✅ Actively maintained

## Installation

### Which PHP versions are supported?

PHP 8.0, 8.1, and 8.2 are officially supported. PHP 8.3 should work but is not extensively tested yet.

### Do I need Rust installed to use the extension?

No, if you use the pre-built binary from GitHub releases. See [QUICKSTART.md](../QUICKSTART.md).

Yes, if you want to build from source. See [BUILDING.md](BUILDING.md).

### Can I install via PECL?

Not yet. Currently available via:
1. Pre-built binaries (recommended)
2. Docker images
3. Build from source

PECL support is planned for future releases.

### Does it work with PHP-FPM?

Yes! Just enable the extension in your PHP-FPM `php.ini`:
```ini
extension=libphp_rs_toon.so
```

Then restart PHP-FPM:
```bash
sudo systemctl restart php8.2-fpm
```

## Usage

### How do I handle errors?

Both functions throw `Exception` on errors:

```php
try {
    $result = toon_encode($data);
} catch (Exception $e) {
    // Handle error
    error_log("TOON encoding failed: " . $e->getMessage());
}
```

See [USAGE.md#error-handling](USAGE.md#error-handling) for more.

### Can I encode objects?

Not directly. Convert objects to arrays first:

```php
$object = new User();
$array = (array) $object; // or $object->toArray()
$toon = toon_encode($array);
```

### What about DateTime objects?

Convert to string or array:

```php
$data = [
    'created_at' => $dateTime->format('Y-m-d H:i:s'),
    // or
    'timestamp' => $dateTime->getTimestamp()
];
```

### How do I handle large datasets?

Consider chunking:

```php
$chunks = array_chunk($largeArray, 1000);
foreach ($chunks as $chunk) {
    $toon = toon_encode($chunk);
    // Process chunk
}
```

## Performance

### How much faster is it really?

Benchmarks show:
- **Small datasets**: 2-3x faster
- **Medium datasets**: 5-10x faster
- **Large datasets**: 10-20x faster

See [BENCHMARKS.md](BENCHMARKS.md) for detailed results.

### Does it use less memory?

Yes, approximately 50-75% less peak memory compared to pure PHP implementation.

### Can I see the benchmarks myself?

Yes! Run the benchmark suite:

```bash
docker build -f Dockerfile.benchmark -t bench .
docker run --rm -v $(pwd)/benchmark:/app/results bench
cat benchmark/results.md
```

## Compatibility

### Is it compatible with toon-php?

The extension aims for compatibility with the pure PHP implementation. However, some edge cases may differ slightly. Always test with your specific data.

### Can I mix both implementations?

Yes! You can use both in the same project:

```php
// Use Rust extension if available, fallback to PHP
if (function_exists('toon_encode')) {
    $toon = toon_encode($data);
} else {
    $toon = \HelgeSverre\Toon\Toon::encode($data);
}
```

### Does it work with Laravel/Symfony?

Yes! It's a standard PHP extension and works with all frameworks. See [USAGE.md#framework-integration](USAGE.md#framework-integration) for examples.

## Troubleshooting

### Extension loads but functions don't exist

Check PHP version compatibility:
```bash
php -v
php -m | grep php-rs-toon
```

If module shows but functions don't work, try:
```bash
sudo systemctl restart php-fpm
# or restart your web server
```

### "Call to undefined function toon_encode()"

1. Verify extension is loaded:
   ```bash
   php -m | grep php-rs-toon
   ```

2. Check php.ini has:
   ```ini
   extension=libphp_rs_toon.so
   ```

3. Restart PHP/web server

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for more.

### Segmentation fault

This should not happen due to Rust's memory safety. If it does:

1. Check PHP version compatibility
2. Rebuild extension for your exact PHP version
3. Report as bug with full details

## Development

### How can I contribute?

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Quick overview:
1. Fork repository
2. Create feature branch
3. Make changes with tests
4. Submit pull request

### How do I report bugs?

Open an issue on [GitHub](https://github.com/mesak/php-rs-toon/issues) with:
- PHP version (`php -v`)
- Extension version
- Steps to reproduce
- Error messages

### Can I add new features?

Yes! Feature requests and contributions are welcome. Please:
1. Open an issue first to discuss
2. Follow existing code style
3. Add tests for new functionality

## License & Support

### What's the license?

MIT License. Free for commercial and personal use.

### Is there commercial support?

Community support is available via GitHub Issues. For enterprise support, contact the maintainers.

### Who maintains this?

See [GitHub contributors](https://github.com/mesak/php-rs-toon/graphs/contributors).

## Related Questions

### What's the difference between JSON and TOON?

| Feature | JSON | TOON |
|---------|------|------|
| Size | Verbose | Compact |
| Readability | Good | Better (indented) |
| LLM Tokens | More | ~30-50% less |
| Parsing Speed | Good | Faster (with this extension) |

### Should I use TOON for all data?

TOON is optimized for:
- ✅ LLM API payloads
- ✅ Human-readable configs  
- ✅ Log aggregation

Not ideal for:
- ❌ APIs requiring JSON
- ❌ Browser-side JavaScript (use JSON)
- ❌ Binary data (use Protocol Buffers, etc.)

---

**Navigation**: [← Docker Guide](DOCKER_GUIDE.md) | [Main README](../README.md) | [Troubleshooting →](TROUBLESHOOTING.md)
