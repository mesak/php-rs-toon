# Troubleshooting Guide

Solutions to common problems with php-rs-toon.

> **[← Back to FAQ](FAQ.md)** | **[Index](INDEX.md)**

## Installation Issues

### Extension File Not Found

**Symptom**:
```
PHP Warning: PHP Startup: Unable to load dynamic library 'libphp_rs_toon.so'
```

**Solutions**:

1. Verify file exists:
   ```bash
   ls -la $(php-config --extension-dir)/libphp_rs_toon.so
   ```

2. Check file permissions:
   ```bash
   sudo chmod 644 $(php-config --extension-dir)/libphp_rs_toon.so
   ```

3. Verify correct extension directory:
   ```bash
   php-config --extension-dir
   php --ini | grep extension_dir
   ```

### Wrong PHP Version

**Symptom**:
```
PHP Warning: PHP Startup: libphp_rs_toon.so: cannot open shared object file
```

**Solution**: Rebuild extension for your PHP version:
```bash
cargo clean
cargo build --release
sudo cp target/release/libphp_rs_toon.so $(php-config --extension-dir)/
```

### Multiple PHP Installations

**Symptom**: Extension works in CLI but not in web server

**Solution**: Check both php.ini files:
```bash
# CLI
php --ini

# FPM (if using)
/etc/php/8.2/fpm/php.ini
```

Ensure both have:
```ini
extension=libphp_rs_toon.so
```

Restart services:
```bash
sudo systemctl restart php8.2-fpm
sudo systemctl restart nginx  # or apache2
```

## Runtime Errors

### Function Not Found

**Symptom**:
```php
Fatal error: Call to undefined function toon_encode()
```

**Solutions**:

1. Verify extension is loaded:
   ```bash
   php -m | grep php-rs-toon
   ```

2. Check functions exist:
   ```bash
   php -r "var_dump(function_exists('toon_encode'), function_exists('toon_decode'));"
   ```

3. Restart PHP:
   ```bash
   sudo systemctl restart php-fpm
   # or
   sudo systemctl restart apache2
   ```

### Encoding Errors

**Symptom**:
```php
Exception: Encoding failed
```

**Common Causes**:

1. **Circular References**:
   ```php
   $a = ['ref' => &$a]; // Don't do this
   toon_encode($a); // Will fail
   ```

2. **Resource Types**:
   ```php
   $file = fopen('file.txt', 'r');
   toon_encode(['file' => $file]); // Will fail
   ```

**Solution**: Convert to serializable data first

### Decoding Errors

**Symptom**:
```php
Exception: Parse error at line X
```

**Solutions**:

1. Validate TOON syntax:
   ```php
   $toon = trim($toonString);
   if (empty($toon)) {
       throw new Exception("Empty TOON string");
   }
   ```

2. Check indentation (must use spaces, not tabs):
   ```
   # ❌ Wrong (tabs)
   user:
   	name: Alice
   
   # ✅ Correct (2 spaces)
   user:
     name: Alice
   ```

3. Verify colon spacing:
   ```
   # ❌ Wrong
   user :name
   
   # ✅ Correct
   user: name
   ```

## Performance Issues

### Slow Encoding

**Symptom**: Encoding takes longer than expected

**Diagnosis**:
```php
$start = microtime(true);
$toon = toon_encode($data);
$time = microtime(true) - $start;
echo "Took: " . ($time * 1000) . "ms\n";
```

**Solutions**:

1. **Profile data size**:
   ```php
   echo "Elements: " . count($data, COUNT_RECURSIVE) . "\n";
   ```

2. **Chunk large datasets**:
   ```php
   foreach (array_chunk($data, 1000) as $chunk) {
       $toon = toon_encode($chunk);
   }
   ```

3. **Check for deep nesting**:
   ```php
   function getDepth($array) {
       $max = 1;
       foreach ($array as $value) {
           if (is_array($value)) {
               $depth = getDepth($value) + 1;
               if ($depth > $max) $max = $depth;
           }
       }
       return $max;
   }
   
   echo "Depth: " . getDepth($data) . "\n";
   ```

### Memory Issues

**Symptom**:
```
PHP Fatal error: Allowed memory size exhausted
```

**Solutions**:

1. **Increase PHP memory limit**:
   ```ini
   ; php.ini
   memory_limit = 512M
   ```

2. **Process in chunks**:
   ```php
   $chunks = array_chunk($largeData, 100);
   foreach ($chunks as $chunk) {
       $toon = toon_encode($chunk);
       // Process/save chunk
       unset($toon); // Free memory
   }
   ```

3. **Use generators** (for sequential data):
   ```php
   function generateData() {
       foreach ($items as $item) {
           yield $item;
       }
   }
   ```

## Docker Issues

### Extension Not Loading in Container

**Symptom**: Works locally but not in Docker

**Solutions**:

1. **Check Dockerfile**:
   ```dockerfile
   # Ensure extension is installed AND enabled
   RUN echo "extension=libphp_rs_toon.so" > /usr/local/etc/php/conf.d/php-rs-toon.ini
   ```

2. **Verify in running container**:
   ```bash
   docker exec <container> php -m | grep php-rs-toon
   ```

3. **Check logs**:
   ```bash
   docker logs <container>
   ```

### Permission Denied in Docker

**Symptom**:
```
Permission denied: /usr/local/lib/php/extensions/
```

**Solution**: Use correct Dockerfile ordering:
```dockerfile
# ✅ Correct: RUN as root before USER command
RUN cp extension.so /usr/local/lib/php/extensions/
RUN echo "extension=..." > /usr/local/etc/php/conf.d/ext.ini
USER www-data
```

## Build Issues

### Rust Not Found

**Symptom**:
```
cargo: command not found
```

**Solution**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### PHP Dev Headers Missing

**Symptom**:
```
php-config: not found
```

**Solution**:
```bash
# Ubuntu/Debian
sudo apt install php8.2-dev

# CentOS/RHEL
sudo yum install php-devel

# macOS
brew install php@8.2
```

### Clang Missing

**Symptom**:
```
error: failed to run custom build command for `ext-php-rs`
clang: not found
```

**Solution**:
```bash
# Ubuntu/Debian
sudo apt install clang libclang-dev

# macOS
xcode-select --install
```

### Compilation Fails

**Symptom**:
```
error: could not compile `php-rs-toon`
```

**Solutions**:

1. **Clean and rebuild**:
   ```bash
   cargo clean
   cargo build --release
   ```

2. **Update Rust**:
   ```bash
   rustup update stable
   ```

3. **Check Rust version**:
   ```bash
   rustc --version  # Should be 1.70+
   ```

## Debugging

### Enable Debug Mode

```bash
# Build debug version
cargo build

# Run with backtrace
RUST_BACKTRACE=1 php test.php
```

### Verbose Logging

```php
<?php
error_reporting(E_ALL);
ini_set('display_errors', 1);

try {
    $result = toon_encode($data);
} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
    echo "Trace: " . $e->getTraceAsString() . "\n";
}
```

### Check Extension Info

```bash
php -r "phpinfo();" | grep -A 10 "php-rs-toon"
```

## Getting Help

If you're still stuck:

1. **Check existing issues**: [GitHub Issues](https://github.com/mesak/php-rs-toon/issues)
2. **Gather information**:
   ```bash
   php -v
   php -m | grep php-rs-toon
   php -r "var_dump(function_exists('toon_encode'));"
   uname -a
   ```
3. **Create minimal reproduction**
4. **Open new issue** with details

## See Also

- **[FAQ.md](FAQ.md)** - Frequently asked questions
- **[INSTALLATION.md](INSTALLATION.md)** - Installation guide
- **[BUILDING.md](BUILDING.md)** - Building from source

---

**Navigation**: [← FAQ](FAQ.md) | [Main README](../README.md) | [Contributing →](CONTRIBUTING.md)
