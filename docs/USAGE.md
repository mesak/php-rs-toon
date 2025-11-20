# Usage Guide

Complete guide to using the php-rs-toon extension.

> **[← Back to Installation](INSTALLATION.md)** | **[API Reference →](API_REFERENCE.md)**

## Table of Contents

1. [Basic Usage](#basic-usage)
2. [Encoding](#encoding)
3. [Decoding](#decoding)
4. [Data Types](#data-types)
5. [Special Cases](#special-cases)
6. [Error Handling](#error-handling)
7. [Framework Integration](#framework-integration)

## Basic Usage

### Encoding

Convert PHP arrays to TOON format:

```php
<?php
$data = [
    'name' => 'Alice',
    'age' => 30,
    'email' => 'alice@example.com'
];

$toon = toon_encode($data);
echo $toon;
```

**Output**:
```
name: Alice
age: 30
email: alice@example.com
```

### Decoding

Parse TOON strings back to PHP arrays:

```php
<?php
$toonString = <<<'TOON'
name: Alice
age: 30
email: alice@example.com
TOON;

$data = toon_decode($toonString);
print_r($data);
```

**Output**:
```
Array
(
    [name] => Alice
    [age] => 30
    [email] => alice@example.com
)
```

## Data Types

### Supported Types

| PHP Type | TOON Representation | Example |
|----------|---------------------|---------|
| `null` | `null` | `value: null` |
| `bool` | `true/false` | `active: true` |
| `int` | number | `count: 42` |
| `float` | decimal | `price: 19.99` |
| `string` | text | `name: Alice` |
| `array` (sequential) | comma-separated | `tags: a, b, c` |
| `array` (associative) | nested structure | See below |

### Nested Structures

```php
<?php
$data = [
    'user' => [
        'profile' => [
            'name' => 'Bob',
            'age' => 25
        ],
        'settings' => [
            'theme' => 'dark',
            'notifications' => true
        ]
    ]
];

echo toon_encode($data);
```

**Output**:
```
user:
  profile:
    name: Bob
    age: 25
  settings:
    theme: dark
    notifications: true
```

### Arrays

#### Simple Lists
```php
$data = ['tags' => ['php', 'rust', 'toon']];
echo toon_encode($data);
// Output: tags: php, rust, toon
```

#### Complex Lists (Array of Objects)
```php
$data = [
    'users' => [
        ['id' => 1, 'name' => 'Alice'],
        ['id' => 2, 'name' => 'Bob']
    ]
];

echo toon_encode($data);
```

**Output**:
```
users:
  0:
    id: 1
    name: Alice
  1:
    id: 2
    name: Bob
```

## Special Cases

### Quoted Strings

Strings with special characters are automatically quoted:

```php
$data = [
    'message' => 'Hello, "World"!',
    'multiline' => "Line 1\nLine 2",
    'emoji' => '🚀 Rocket'
];

echo toon_encode($data);
```

**Output**:
```
message: "Hello, \"World\"!"
multiline: "Line 1\nLine 2"
emoji: 🚀 Rocket
```

### Empty Values

```php
$data = [
    'empty_string' => '',
    'empty_array' => [],
    'null_value' => null
];

echo toon_encode($data);
```

**Output**:
```
empty_string: 
empty_array: 
null_value: null
```

### Unicode Support

```php
$data = [
    'chinese' => '你好世界',
    'emoji' => '😀🎉🚀',
    'arabic' => 'مرحبا'
];

echo toon_encode($data);
// Handles all Unicode correctly
```

## Error Handling

### Encoding Errors

```php
<?php
try {
    $result = toon_encode($data);
} catch (Exception $e) {
    echo "Encoding error: " . $e->getMessage();
}
```

### Decoding Errors

```php
<?php
try {
    $invalidToon = "user:\n  invalid: [unclosed";
    $data = toon_decode($invalidToon);
} catch (Exception $e) {
    echo "Decoding error: " . $e->getMessage();
}
```

## Framework Integration

### Laravel

```php
<?php

namespace App\Services;

class TOONService
{
    public function encodeForLLM(array $data): string
    {
        return toon_encode($data);
    }

    public function decodeFromLLM(string $toon): array
    {
        return toon_decode($toon);
    }
}
```

**Usage in Controller**:
```php
use App\Services\TOONService;

class APIController extends Controller
{
    public function sendToLLM(TOONService $toon)
    {
        $data = User::all()->toArray();
        $encoded = $toon->encodeForLLM($data);
        
        // Send to LLM API...
    }
}
```

### Symfony

```php
<?php

namespace App\Service;

class ToonFormatter
{
    public function format(array $data): string
    {
        return toon_encode($data);
    }

    public function parse(string $toon): array
    {
        return toon_decode($toon);
    }
}
```

## Best Practices

### 1. Validate Input

```php
if (empty($data)) {
    throw new InvalidArgumentException('Data cannot be empty');
}
$toon = toon_encode($data);
```

### 2. Handle Large Data

For large datasets, consider chunking:
```php
$chunks = array_chunk($largeArray, 1000);
foreach ($chunks as $chunk) {
    $toon = toon_encode($chunk);
    // Process chunk...
}
```

### 3. Cache Encoded Results

```php
$cacheKey = md5(json_encode($data));
$toon = $cache->remember($cacheKey, 3600, function() use ($data) {
    return toon_encode($data);
});
```

## Performance Tips

1. **Batch Operations**: Encode/decode multiple items in one call
2. **Reuse Connections**: Keep encoded strings in memory when possible
3. **Profile**: Use `microtime()` to measure encoding/decoding time

```php
$start = microtime(true);
$toon = toon_encode($largeData);
$time = microtime(true) - $start;
echo "Encoded in " . ($time * 1000) . "ms\n";
```

## See Also

- **[API_REFERENCE.md](API_REFERENCE.md)** - Complete API documentation
- **[examples/](../examples/)** - Working code examples
- **[BENCHMARKS.md](BENCHMARKS.md)** - Performance data

---

**Navigation**: [← Installation](INSTALLATION.md) | [Index](INDEX.md) | [API Reference →](API_REFERENCE.md)
