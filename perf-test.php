<?php

/**
 * Simple performance test for php-rs-toon extension
 */

echo "╔════════════════════════════════════════════════════════════════╗\n";
echo "║         PHP TOON Extension - Performance Benchmark              ║\n";
echo "╚════════════════════════════════════════════════════════════════╝\n\n";

// Verify extension is loaded
if (!extension_loaded('php-rs-toon')) {
    echo "❌ Error: php-rs-toon extension is not loaded!\n";
    exit(1);
}

echo "✅ php-rs-toon extension is loaded\n\n";

// Test datasets
$smallData = [
    'user' => [
        'id' => 123,
        'name' => 'John Doe',
        'email' => 'john@example.com'
    ]
];

$mediumData = [
    'users' => array_map(function ($i) {
        return [
            'id' => $i,
            'name' => "User $i",
            'email' => "user$i@example.com",
            'metadata' => [
                'active' => true,
                'score' => rand(1, 100),
                'tags' => ['tag1', 'tag2', 'tag3']
            ]
        ];
    }, range(1, 100))
];

$largeData = [
    'records' => array_map(function ($i) {
        return [
            'id' => $i,
            'type' => 'record',
            'attributes' => [
                'field1' => "Value $i",
                'field2' => rand(1, 1000),
                'field3' => true,
                'nested' => [
                    'deep1' => "Deep value $i",
                    'deep2' => rand(1, 100),
                    'deep3' => ['a', 'b', 'c']
                ]
            ]
        ];
    }, range(1, 1000))
];

// Performance testing function
function benchmark($name, $callback, $iterations = 1000) {
    echo "--- $name ($iterations iterations) ---\n";
    
    $start = microtime(true);
    for ($i = 0; $i < $iterations; $i++) {
        $callback();
    }
    $end = microtime(true);
    
    $totalTime = ($end - $start) * 1000; // Convert to milliseconds
    $avgTime = $totalTime / $iterations;
    $opsPerSec = round($iterations / ($totalTime / 1000));
    
    printf("  Total: %.2f ms\n", $totalTime);
    printf("  Average: %.4f ms per op\n", $avgTime);
    printf("  Ops/sec: %d\n", $opsPerSec);
    echo "\n";
}

// ============================================================================
// ENCODE BENCHMARKS
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "ENCODING BENCHMARKS\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

benchmark("Small Encode (3 items)", function() use ($smallData) {
    toon_encode($smallData);
}, 5000);

benchmark("Medium Encode (100 items)", function() use ($mediumData) {
    toon_encode($mediumData);
}, 1000);

benchmark("Large Encode (1000 items)", function() use ($largeData) {
    toon_encode($largeData);
}, 100);

// ============================================================================
// DECODE BENCHMARKS
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "DECODING BENCHMARKS\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

$smallToon = toon_encode($smallData);
$mediumToon = toon_encode($mediumData);
$largeToon = toon_encode($largeData);

benchmark("Small Decode (3 items)", function() use ($smallToon) {
    toon_decode($smallToon);
}, 5000);

benchmark("Medium Decode (100 items)", function() use ($mediumToon) {
    toon_decode($mediumToon);
}, 1000);

benchmark("Large Decode (1000 items)", function() use ($largeToon) {
    toon_decode($largeToon);
}, 100);

// ============================================================================
// ROUNDTRIP BENCHMARKS
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "ROUNDTRIP BENCHMARKS (Encode + Decode)\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

benchmark("Small Roundtrip", function() use ($smallData) {
    toon_decode(toon_encode($smallData));
}, 3000);

benchmark("Medium Roundtrip", function() use ($mediumData) {
    toon_decode(toon_encode($mediumData));
}, 500);

benchmark("Large Roundtrip", function() use ($largeData) {
    toon_decode(toon_encode($largeData));
}, 50);

echo "═══════════════════════════════════════════════════════════════════\n";
echo "✅ Benchmark complete!\n";
echo "═══════════════════════════════════════════════════════════════════\n";

?>
