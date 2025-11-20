# Contributing Guide

Thank you for considering contributing to php-rs-toon!

> **[← Back to Troubleshooting](TROUBLESHOOTING.md)** | **[Index](INDEX.md)**

## Getting Started

### 1. Fork and Clone

```bash
# Fork on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/php-rs-toon.git
cd php-rs-toon
```

### 2. Set Up Development Environment

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PHP dev headers
sudo apt install php8.2-dev  # Ubuntu/Debian

# Build extension
cargo build

# Run tests
cargo test
php test.php
```

## Making Changes

###  Development Workflow

1. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make changes** in small, logical commits

3. **Test your changes**:
   ```bash
   cargo test
   cargo fmt
   cargo clippy
   php test.php
   ```

4. **Commit**:
   ```bash
   git add .
   git commit -m "feat: add new feature"
   ```

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): subject

body (optional)

footer (optional)
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(encoder): add support for custom indentation

fix(parser): handle quoted strings with newlines

docs(readme): update installation instructions

test(decode): add edge case for empty arrays
```

## Code Style

### Rust Code

Follow standard Rust conventions:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features

# Check for common issues
cargo clippy -- -D warnings
```

### PHP Code

Follow PSR-12:

```php
<?php

// ✅ Good
function processData(array $data): string
{
    return toon_encode($data);
}

// ❌ Bad
function processData($data) {
  return toon_encode($data);
}
```

## Testing

### Rust Unit Tests

Add tests in `src/toon.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        let input = "test: value";
        let result = parse(input).unwrap();
        assert_eq!(/* expected */, result);
    }
}
```

Run tests:
```bash
cargo test
```

### PHP Integration Tests

Add tests to `test.php`:

```php
<?php
test_round_trip("Your Test", [
    'test' => 'data'
]);
```

Run tests:
```bash
php test.php
```

### Benchmark Tests

Add benchmarks to `benchmark/benchmarks/ToonBench.php`:

```php
#[Bench\Revs(1000)]
#[Bench\Iterations(5)]
public function benchYourFeature(): void
{
    toon_encode($this->testData);
}
```

Run benchmarks:
```bash
cd benchmark
./run-benchmarks.sh
```

## Documentation

### Code Comments

```rust
/// Parses a TOON string into ToonValue
///
/// # Arguments
/// * `input` - TOON formatted string
///
/// # Returns
/// * `Ok(ToonValue)` on success
/// * `Err(anyhow::Error)` on parse error
///
/// # Example
/// ```
/// let result = parse("key: value")?;
/// ```
pub fn parse(input: &str) -> anyhow::Result<ToonValue> {
    // implementation
}
```

### Update Documentation

If you change functionality, update:
- `docs/USAGE.md` - Usage examples
- `docs/API_REFERENCE.md` - API documentation
- `README.md` - If adding major features
- `examples/` - Add example if appropriate

## Pull Request Process

### 1. Prepare PR

```bash
# Ensure tests pass
cargo test
cargo clippy
php test.php

# Update documentation if needed

# Push to your fork
git push origin feature/your-feature-name
```

### 2. Create PR

On GitHub:
1. Go to original repository
2. Click "New Pull Request"
3. Select your branch
4. Fill in template (see below)

### 3. PR Template

```markdown
## Description
Brief description of changes

## Motivation
Why is this change needed?

## Changes
- List specific changes
- One per line

## Testing
How did you test this?

## Checklist
- [ ] Tests pass (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Linter passes (`cargo clippy`)
- [ ] PHP tests pass
- [ ] Documentation updated
- [ ] Commit messages follow convention
```

### 4. PR Review

- Respond to feedback promptly
- Make requested changes in new commits
- Mark conversations as resolved after addressing

## Reporting Bugs

### Before Reporting

1. **Search existing issues**: [GitHub Issues](https://github.com/mesak/php-rs-toon/issues)
2. **Verify it's a bug**: Not a usage question
3. **Check latest version**: Bug may be fixed

### Bug Report Template

```markdown
**Environment**
- PHP version: `php -v`
- Extension version: [e.g., v0.1.0]
- OS: [e.g., Ubuntu 22.04]

**Steps to Reproduce**
1. Create file with...
2. Run command...
3. See error...

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Minimal Example**
```php
$data = ['test' => 'value'];
$result = toon_encode($data);
```

**Error Message**
```
Full error message here
```

**Additional Context**
Any other relevant information
```

## Feature Requests

### Before Requesting

1. **Check existing requests**: [GitHub Issues](https://github.com/mesak/php-rs-toon/issues)
2. **Consider scope**: Does it fit the project goals?
3. **Provide use case**: Why is this needed?

### Feature Request Template

```markdown
**Problem**
What problem does this solve?

**Proposed Solution**
How should it work?

**Example**
```php
// Show how it would be used
```

**Alternatives Considered**
What other approaches did you think about?

**Additional Context**
Any other relevant information
```

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all.

### Expected Behavior

- Be respectful and inclusive
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards others

### Unacceptable Behavior

- Harassment or discrimination
- Trolling or insulting comments
- Public or private harassment
- Publishing others' private information

### Enforcement

Violations may result in:
1. Warning
2. Temporary ban
3. Permanent ban

Report issues to project maintainers.

## Questions?

- **Usage questions**: [GitHub Discussions](https://github.com/mesak/php-rs-toon/discussions)
- **Bug reports**: [GitHub Issues](https://github.com/mesak/php-rs-toon/issues)
- **Feature requests**: [GitHub Issues](https://github.com/mesak/php-rs-toon/issues)

## Recognition

Contributors will be:
- Listed in README.md
- Credited in release notes  
- Added to CONTRIBUTORS file

Thank you for contributing! 🎉

---

**Navigation**: [← Troubleshooting](TROUBLESHOOTING.md) | [Index](INDEX.md)
