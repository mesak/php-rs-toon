# API Reference

Complete API documentation for php-rs-toon extension.

> **[← Back to Usage Guide](USAGE.md)** | **[Index](INDEX.md)**

## Table of Contents

1. [Functions](#functions)
2. [toon_encode()](#toon_encode)
3. [toon_decode()](#toon_decode)
4. [Type Mapping](#type-mapping)
5. [Error Codes](#error-codes)

## Functions

The extension provides two core functions:

| Function | Purpose | Since |
|----------|---------|-------|
| `toon_encode()` | Convert PHP array to TOON string | v1.0.0 |
| `toon_decode()` | Parse TOON string to PHP array | v1.0.0 |

---

## toon_encode()

Converts a PHP array or value into TOON format string.

### Signature

```php
function toon_encode(mixed $value): string
```

### Parameters

- **`$value`** (`mixed`, required)  
  The PHP value to encode. Can be:
  - `null`
  - `bool`
  - `int`
  - `float`
  - `string`
  - `array` (sequential or associative)

### Return Value

Returns a TOON-formatted string representation of the input value.

### Throws

- **`Exception`** - If encoding fails (e.g., unsupported type, circular reference)

### Examples

#### Basic Types

```php
<?php
echo toon_encode(null);           // Output: null
echo toon_encode(true);           // Output: true
echo toon_encode(42);             // Output: 42
echo toon_encode(3.14);           // Output: 3.14
echo toon_encode('Hello');        // Output: Hello
```

#### Arrays

```php
<?php
$simple = ['a', 'b', 'c'];
echo toon_encode($simple);
// Output: a, b, c

$assoc = ['name' => 'Alice', 'age' => 30];
echo toon_encode($assoc);
// Output:
// name: Alice
// age: 30
```

#### Nested Structures

```php
<?php
$nested = [
    'user' => [
        'name' => 'Bob',
        'meta' => ['role' => 'admin']
    ]
];

echo toon_encode($nested);
// Output:
// user:
//   name: Bob
//   meta:
//     role: admin
```

### Behavior Details

#### String Quoting

Strings are automatically quoted if they contain:
- Newlines (`\n`)
- Colons (`:`)
- Commas (`,`)
- Quotes (`"`)
- Leading/trailing whitespace

```php
echo toon_encode(['msg' => 'Hello, World!']);
// Output: msg: "Hello, World!"
```

#### Array Detection

Arrays are treated as:
- **Lists** (comma-separated) if keys are sequential integers starting from 0
- **Maps** (nested key-value) otherwise

```php
// List
echo toon_encode([1, 2, 3]);
// Output: 1, 2, 3

// Map
echo toon_encode(['a' => 1, 'b' => 2]);
// Output:
// a: 1
// b: 2
```

#### Complex Lists

Arrays containing complex values (arrays/objects) are encoded as maps with integer keys:

```php
$data = [
    ['id' => 1],
    ['id' => 2]
];

echo toon_encode($data);
// Output:
// 0:
//   id: 1
// 1:
//   id: 2
```

---

## toon_decode()

Parses a TOON-formatted string into a PHP array.

### Signature

```php
function toon_decode(string $toon): mixed
```

### Parameters

- **`$toon`** (`string`, required)  
  A valid TOON-formatted string to decode

### Return Value

Returns the decoded PHP value. Type depends on the TOON input:
- `null` → `null`
- `true/false` → `bool`  
- numbers → `int` or `float`
- text → `string`
- key-value pairs → `array` (associative)
- lists → `array` (sequential)

### Throws

- **`Exception`** - If parsing fails (e.g., invalid TOON syntax)

### Examples

#### Basic Decoding

```php
<?php
$toon = "name: Alice\nage: 30";
$data = toon_decode($toon);

print_r($data);
// Array
// (
//     [name] => Alice
//     [age] => 30
// )
```

#### Nested Structures

```php
<?php
$toon = <<<'TOON'
user:
  profile:
    name: Bob
    age: 25
  settings:
    theme: dark
TOON;

$data = toon_decode($toon);
// Returns deeply nested associative array
```

#### Lists

```php
<?php
$toon = "tags: php, rust, toon";
$data = toon_decode($toon);

print_r($data);
// Array
// (
//     [tags] => Array
//         (
//             [0] => php
//             [1] => rust
//             [2] => toon
//         )
// )
```

### Behavior Details

#### Type Inference

Values are automatically converted to appropriate PHP types:

```php
$toon = <<<'TOON'
null_val: null
bool_val: true
int_val: 42
float_val: 3.14
string_val: Hello
TOON;

$data = toon_decode($toon);
// Returns:
// [
//     'null_val' => null,
//     'bool_val' => true,
//     'int_val' => 42,
//     'float_val' => 3.14,
//     'string_val' => 'Hello'
// ]
```

#### Quoted String Handling

Escape sequences in quoted strings are processed:

```php
$toon = 'message: "Line 1\nLine 2"';
$data = toon_decode($toon);

echo $data['message'];
// Output:
// Line 1
// Line 2
```

---

## Type Mapping

### PHP → TOON

| PHP Type | TOON Format | Example |
|----------|-------------|---------|
| `null` | `null` | `value: null` |
| `true` | `true` | `active: true` |
| `false` | `false` | `active: false` |
| `int` | number | `count: 42` |
| `float` | decimal | `price: 19.99` |
| `string` | text/quoted | `name: Alice` or `msg: "Hello\nWorld"` |
| `array` (list) | comma-separated | `tags: a, b, c` |
| `array` (map) | indented key-value | `user:\n  name: Alice` |

### TOON → PHP

| TOON Value | PHP Type | Example |
|------------|----------|---------|
| `null` | `null` | `null` |
| `true` | `bool` | `true` |
| `false` | `bool` | `false` |
| `123` | `int` | `123` |
| `3.14` | `float` | `3.14` |
| `text` | `string` | `"text"` |
| `"quoted"` | `string` | `"quoted"` (unescaped) |
| `a, b, c` | `array` | `['a', 'b', 'c']` |
| `key: value` | `array` | `['key' => 'value']` |

---

## Error Codes

### Encoding Errors

| Error | Cause | Solution |
|-------|-------|----------|
| Generic Exception | Unsupported type or internal error | Check input type, report bug if PHP type is supported |

### Decoding Errors

| Error | Cause | Solution |
|-------|-------|----------|
| Parse Error | Invalid TOON syntax | Validate TOON string format |
| Generic Exception | Malformed input | Check for proper indentation and colons |

### Example Error Handling

```php
<?php
try {
    $result = toon_encode($data);
} catch (Exception $e) {
    error_log("TOON encoding failed: " . $e->getMessage());
    // Fallback to JSON or handle error
}

try {
    $data = toon_decode($toonString);
} catch (Exception $e) {
    error_log("TOON decoding failed: " . $e->getMessage());
    // Return default value or re-throw
}
```

---

## Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| `toon_encode()` | O(n) | n = total elements in structure |
| `toon_decode()` | O(n) | n = number of lines in TOON string |

### Space Complexity

Both operations use O(n) additional memory for the output.

### Comparison to JSON

Based on benchmarks (see [BENCHMARKS.md](BENCHMARKS.md)):
- **Encoding**: 5-20x faster than pure PHP implementation
- **Decoding**: 5-20x faster than pure PHP implementation
- **Memory**: ~50-70% less peak memory usage

---

## See Also

- **[USAGE.md](USAGE.md)** - Usage guide with examples
- **[BENCHMARKS.md](BENCHMARKS.md)** - Performance comparisons
- **[examples/](../examples/)** - Working code samples

---

**Navigation**: [← Usage](USAGE.md) | [Main README](../README.md) | [Benchmarks →](BENCHMARKS.md)
