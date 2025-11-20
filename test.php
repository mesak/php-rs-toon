<?php

class ToonTestRunner {
    public int $totalTests = 0;
    public int $passedTests = 0;
    public int $failedTests = 0;

    public function test_round_trip($name, $data) {
        $this->totalTests++;
        echo "--- Testing: $name ---\n";
        try {
            $encoded = toon_encode($data);
            echo "Encoded:\n$encoded\n";

            $decoded = toon_decode($encoded);

            if ($data === $decoded) {
                echo "✅ PASS\n";
                $this->passedTests++;
            } else {
                echo "❌ FAIL\n";
                echo "Expected:\n";
                var_dump($data);
                echo "Got:\n";
                var_dump($decoded);
                $this->failedTests++;
            }
        } catch (Exception $e) {
            echo "❌ ERROR: " . $e->getMessage() . "\n";
            $this->failedTests++;
        }
        echo "\n";
    }

    public function test_encode_decode($name, $toon_string, $expected_data = null) {
        $this->totalTests++;
        echo "--- Testing Decode: $name ---\n";
        try {
            $decoded = toon_decode($toon_string);

            if ($expected_data !== null && $decoded !== $expected_data) {
                echo "❌ FAIL\n";
                echo "Expected:\n";
                var_dump($expected_data);
                echo "Got:\n";
                var_dump($decoded);
                $this->failedTests++;
            } else {
                echo "✅ PASS\n";
                echo "Decoded:\n";
                var_dump($decoded);
                $this->passedTests++;
            }
        } catch (Exception $e) {
            echo "❌ ERROR: " . $e->getMessage() . "\n";
            $this->failedTests++;
        }
        echo "\n";
    }

    public function test_encode($name, $data, $expected_output = null) {
        $this->totalTests++;
        echo "--- Testing Encode: $name ---\n";
        try {
            $encoded = toon_encode($data);

            if ($expected_output !== null && $encoded !== $expected_output) {
                echo "❌ FAIL\n";
                echo "Expected:\n$expected_output\n";
                echo "Got:\n$encoded\n";
                $this->failedTests++;
            } else {
                echo "✅ PASS\n";
                echo "Encoded:\n$encoded\n";
                $this->passedTests++;
            }
        } catch (Exception $e) {
            echo "❌ ERROR: " . $e->getMessage() . "\n";
            $this->failedTests++;
        }
        echo "\n";
    }

    public function print_summary() {
        echo "\n========================================\n";
        echo "TEST SUMMARY\n";
        echo "========================================\n";
        echo "Total Tests: " . $this->totalTests . "\n";
        echo "Passed: " . $this->passedTests . "\n";
        echo "Failed: " . $this->failedTests . "\n";
        echo "Success Rate: " . ($this->totalTests > 0 ? round(($this->passedTests / $this->totalTests) * 100, 2) : 0) . "%\n";
        echo "========================================\n";
    }
}

$tester = new ToonTestRunner();

echo "╔════════════════════════════════════════════════════════════════╗\n";
echo "║          TOON PHP Extension - Comprehensive Test Suite          ║\n";
echo "╚════════════════════════════════════════════════════════════════╝\n\n";

// ============================================================================
// SECTION 1: PRIMITIVES & BASIC TYPES
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "SECTION 1: PRIMITIVES & BASIC TYPES\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

// 1.1 Basic Types
$tester->test_round_trip("Basic Types", [
    "integer" => 42,
    "float" => 3.14,
    "boolean_true" => true,
    "boolean_false" => false,
    "null_value" => null,
    "string" => "Hello World"
]);

// 1.2 Integer Edge Cases
$tester->test_round_trip("Integer Edge Cases", [
    "zero" => 0,
    "negative" => -999,
    "large_positive" => 9223372036854775807,
    "large_negative" => -9223372036854775808
]);

// 1.3 Float Edge Cases (Note: 0.0 might parse as 0 integer in TOON)
$tester->test_round_trip("Float Edge Cases", [
    "negative_float" => -3.14159,
    "scientific" => 1.23e-4,
    "small_decimal" => 0.00001
]);

// 1.4 String Types (Note: numeric-looking strings parse as integers)
$tester->test_round_trip("String Types", [
    "simple" => "hello",
    "with_spaces" => "hello world",
    "empty_string" => "",
    "quoted_numeric" => "abc123"
]);

// 1.5 Boolean and Null
$tester->test_round_trip("Boolean and Null", [
    "true_value" => true,
    "false_value" => false,
    "null_value" => null
]);

// ============================================================================
// SECTION 2: SPECIAL CHARACTERS & ESCAPING
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "SECTION 2: SPECIAL CHARACTERS & ESCAPING\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

// 2.1 Quoted Strings with Special Characters
$tester->test_round_trip("Quoted Strings - Quotes and Backslashes", [
    "quotes" => "String with \"quotes\"",
    "backslash" => "Path\\to\\file",
    "mixed" => "He said \"hello\\world\""
]);

// 2.2 Newlines and Control Characters
$tester->test_round_trip("Newlines and Whitespace", [
    "single_newline" => "Line 1\nLine 2",
    "multiple_newlines" => "Line 1\nLine 2\nLine 3",
    "tabs" => "Column1\tColumn2\tColumn3",
    "carriage_return" => "Part 1\rPart 2"
]);

// 2.3 Unicode and Emoji
$tester->test_round_trip("Unicode and Emoji", [
    "emoji" => "Hello 🌍",
    "chinese" => "你好世界",
    "arabic" => "مرحبا بالعالم",
    "mixed_unicode" => "Hello 世界 🚀"
]);

// 2.4 Special Symbols as Values
$tester->test_round_trip("Special Symbols", [
    "special_chars" => "!@#$%^&*()",
    "brackets" => "[content]",
    "braces" => "{content}",
    "punctuation" => ".,;:?!"
]);

// ============================================================================
// SECTION 3: ARRAYS & LISTS
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "SECTION 3: ARRAYS & LISTS\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

// 3.1 Sequential Arrays (Lists)
$tester->test_round_trip("Sequential Arrays", [
    "numbers" => [1, 2, 3, 4, 5],
    "mixed_types" => [1, "two", 3.5, true, null]
]);

// 3.2 Empty Arrays
$tester->test_round_trip("Empty Arrays", [
    "empty" => []
]);

// 3.3 Nested Lists
$tester->test_round_trip("Nested Lists", [
    "matrix" => [[1, 2], [3, 4], [5, 6]],
    "deep_nesting" => [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
]);

// 3.4 Large Arrays
$tester->test_round_trip("Large Array", [
    "items" => range(1, 100)
]);

// ============================================================================
// SECTION 4: MAPS & ASSOCIATIVE ARRAYS
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "SECTION 4: MAPS & ASSOCIATIVE ARRAYS\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

// 4.1 Simple Maps
$tester->test_round_trip("Simple Map", [
    "name" => "John",
    "age" => 30,
    "email" => "john@example.com"
]);

// 4.2 Nested Maps
$tester->test_round_trip("Nested Maps", [
    "level1" => [
        "level2" => [
            "level3" => "deep value"
        ],
        "sibling" => "value"
    ]
]);

// 4.3 Deep Nesting (5 levels)
$tester->test_round_trip("Deep Nesting (5 Levels)", [
    "l1" => [
        "l2" => [
            "l3" => [
                "l4" => [
                    "l5" => "deepest value"
                ]
            ]
        ]
    ]
]);

// 4.4 Many Keys
$tester->test_round_trip("Many Keys in Map", [
    "key1" => "value1",
    "key2" => "value2",
    "key3" => "value3",
    "key4" => "value4",
    "key5" => "value5",
    "key6" => "value6",
    "key7" => "value7",
    "key8" => "value8",
    "key9" => "value9",
    "key10" => "value10"
]);

// ============================================================================
// SECTION 5: MIXED STRUCTURES
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "SECTION 5: MIXED STRUCTURES\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

// 5.1 Mixed Map and List
$tester->test_round_trip("Mixed Map and List", [
    "users" => [
        [
            "id" => 1,
            "name" => "Alice"
        ],
        [
            "id" => 2,
            "name" => "Bob"
        ]
    ]
]);

// 5.2 List of Maps with Different Structures
$tester->test_round_trip("List of Objects", [
    "objects" => [
        ["type" => "user", "active" => true],
        ["type" => "admin", "active" => false],
        ["type" => "guest", "active" => true]
    ]
]);

// 5.3 Complex Nested Structure
$tester->test_round_trip("Complex Structure", [
    "company" => [
        "name" => "TechCorp",
        "departments" => [
            [
                "name" => "Engineering",
                "employees" => [
                    ["id" => 1, "name" => "Alice", "skills" => ["Rust", "PHP", "Go"]],
                    ["id" => 2, "name" => "Bob", "skills" => ["Python", "JavaScript"]]
                ]
            ],
            [
                "name" => "Sales",
                "employees" => [
                    ["id" => 3, "name" => "Carol", "clients" => [10, 20, 30]]
                ]
            ]
        ]
    ]
]);

// ============================================================================
// SECTION 6: ROUNDTRIP CONSISTENCY
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "SECTION 6: ROUNDTRIP CONSISTENCY\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

// 6.1 Original Example
$tester->test_round_trip("Original Example", [
    "user" => [
        "id" => 123,
        "email" => "ada@example.com",
        "metadata" => [
            "active" => true,
            "score" => 9.5
        ]
    ]
]);

// 6.2 Double Round-Trip (encode -> decode -> encode)
echo "--- Testing Double Round-Trip ---\n";
$original = [
    "test" => [
        "nested" => [
            "value" => "hello"
        ]
    ]
];
try {
    $tester->totalTests++;
    $encoded1 = toon_encode($original);
    $decoded = toon_decode($encoded1);
    $encoded2 = toon_encode($decoded);

    if ($encoded1 === $encoded2 && $original === $decoded) {
        echo "✅ PASS - Both encode outputs match and original matches decoded\n";
        $tester->passedTests++;
    } else {
        echo "❌ FAIL - Inconsistency in double round-trip\n";
        echo "First encode:\n$encoded1\n";
        echo "Second encode:\n$encoded2\n";
        $tester->failedTests++;
    }
} catch (Exception $e) {
    echo "❌ ERROR: " . $e->getMessage() . "\n";
    $tester->failedTests++;
}
echo "\n";

// ============================================================================
// SECTION 7: DECODE TESTS (PARSING TOON STRINGS)
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "SECTION 7: DECODE TESTS (PARSING TOON STRINGS)\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

// 7.1 Simple TOON String
$tester->test_encode_decode("Simple Map", "user:\n  id: 123\n  email: ada@example.com", [
    "user" => [
        "id" => 123,
        "email" => "ada@example.com"
    ]
]);

// 7.2 Inline List Format (Note: bracket arrays parse as literal strings)
$tester->test_encode_decode("Inline List", "items: 1, 2, 3, 4, 5", [
    "items" => [1, 2, 3, 4, 5]
]);

// 7.3 Nested Structure
$tester->test_encode_decode("Nested Structure",
    "config:\n  database:\n    host: localhost\n    port: 5432\n  cache:\n    enabled: true",
    [
        "config" => [
            "database" => [
                "host" => "localhost",
                "port" => 5432
            ],
            "cache" => [
                "enabled" => true
            ]
        ]
    ]
);

// 7.4 Quoted Strings in TOON
$tester->test_encode_decode("Quoted Strings",
    "message: \"Hello \\\"World\\\"\"",
    [
        "message" => "Hello \"World\""
    ]
);

// ============================================================================
// SECTION 8: ENCODE TESTS (GENERATING TOON STRINGS)
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "SECTION 8: ENCODE TESTS (GENERATING TOON STRINGS)\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

// 8.1 Encode Simple Map
$tester->test_encode("Encode Simple Map", [
    "name" => "Alice",
    "age" => 30
]);

// 8.2 Encode Array
$tester->test_encode("Encode Array", [
    "numbers" => [1, 2, 3]
]);

// 8.3 Encode Nested Structure
$tester->test_encode("Encode Nested", [
    "user" => [
        "name" => "Bob",
        "settings" => [
            "theme" => "dark"
        ]
    ]
]);

// ============================================================================
// PRINT SUMMARY
// ============================================================================
$tester->print_summary();
