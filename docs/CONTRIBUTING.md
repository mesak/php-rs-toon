# Contributing to php-rs-toon

Thank you for your interest in contributing! We welcome contributions of all kinds.

## Quick Start

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/php-rs-toon.git
cd php-rs-toon

# 2. Build and test
cargo build --release
cargo test
php -d extension=target/release/libphp_rs_toon.so test.php
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Keep changes focused and atomic
- Write clear commit messages
- Add tests for new features

### 3. Code Quality

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy --release

# Run tests
cargo test
```

### 4. Test with PHP

```bash
# Build extension
cargo build --release

# Run PHP tests
php -d extension=target/release/libphp_rs_toon.so test.php
```

### 5. Submit Pull Request

- Push to your fork
- Create PR with clear description
- Link related issues

## Code Guidelines

### Rust Code

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Address `cargo clippy` warnings
- Add unit tests in `src/toon.rs`

### PHP Tests

- Add test cases to `test.php`
- Test both encode and decode paths
- Include edge cases

## Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
type(scope): description

[optional body]
[optional footer]
```

**Types**: `feat`, `fix`, `docs`, `test`, `refactor`, `perf`, `chore`

**Examples**:
```
feat(parser): add support for multiline strings
fix(encoder): handle empty arrays correctly
docs: update installation guide
test: add edge cases for nested structures
```

## Project Structure

```
src/
├── lib.rs    # PHP FFI bridge (toon_encode/toon_decode)
└── toon.rs   # TOON parser and encoder
```

### Key Functions

- `lib.rs`:
  - `toon_value_to_zval()` - Convert ToonValue → PHP Zval
  - `zval_to_toon_value()` - Convert PHP Zval → ToonValue

- `toon.rs`:
  - `parse()` - Parse TOON string → ToonValue
  - `encode()` - Encode ToonValue → TOON string

## Testing

### Rust Unit Tests

Located in `src/toon.rs` under `#[cfg(test)]`:

```rust
#[test]
fn test_my_feature() {
    // Your test here
}
```

### PHP Integration Tests

Add to `test.php`:

```php
function test_my_feature() {
    $data = ['key' => 'value'];
    $encoded = toon_encode($data);
    $decoded = toon_decode($encoded);
    assert($decoded == $data);
    echo "✓ test_my_feature passed\n";
}
```

## Performance Considerations

- Use `Vec::with_capacity()` when size is known
- Avoid unnecessary allocations
- Prefer `&str` over `String` where possible
- Check recursion depth limits

## Documentation

- Update README.md for user-facing changes
- Add inline comments for complex logic
- Update API_REFERENCE.md for API changes

## Getting Help

- Check [FAQ](FAQ.md) and [TROUBLESHOOTING](TROUBLESHOOTING.md)
- Open an issue for questions
- Join discussions in pull requests

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
