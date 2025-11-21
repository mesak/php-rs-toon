<?php

/**
 * Performance benchmark with comparison to baseline
 */

echo "╔════════════════════════════════════════════════════════════════╗\n";
echo "║    PHP TOON Extension - Performance Benchmark & Comparison      ║\n";
echo "╚════════════════════════════════════════════════════════════════╝\n\n";

// Verify extension is loaded
if (!extension_loaded('php-rs-toon')) {
    echo "❌ Error: php-rs-toon extension is not loaded!\n";
    exit(1);
}

echo "✅ php-rs-toon extension is loaded\n\n";

// Baseline performance from docs (estimated from BENCHMARKS.md)
$baselines = [
    'small_encode' => 0.005,  // ms
    'small_decode' => 0.005,  // ms
    'medium_encode' => 0.5,   // ms
    'medium_decode' => 0.6,   // ms
    'large_encode' => 5.0,    // ms
    'large_decode' => 6.0,    // ms
];

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

// Performance testing function with comparison
function benchmark_with_comparison($name, $baseline_key, $callback, $iterations = 1000) {
    global $baselines;
    
    echo "--- $name ($iterations iterations) ---\n";
    
    $start = microtime(true);
    for ($i = 0; $i < $iterations; $i++) {
        $callback();
    }
    $end = microtime(true);
    
    $totalTime = ($end - $start) * 1000; // Convert to milliseconds
    $avgTime = $totalTime / $iterations;
    $opsPerSec = round($iterations / ($totalTime / 1000));
    
    printf("  Current: %.4f ms/op | %d ops/sec\n", $avgTime, $opsPerSec);
    
    if (isset($baselines[$baseline_key])) {
        $baseline = $baselines[$baseline_key];
        $diff = $avgTime - $baseline;
        $percent = ($diff / $baseline) * 100;
        
        if ($diff < 0) {
            printf("  Baseline: %.4f ms/op | ✅ FASTER by %.2f%% (%.4f ms)\n", $baseline, abs($percent), abs($diff));
        } else {
            printf("  Baseline: %.4f ms/op | ⚠️  SLOWER by %.2f%% (+%.4f ms)\n", $baseline, $percent, $diff);
        }
    }
    echo "\n";
}

// ============================================================================
// ENCODE BENCHMARKS
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "ENCODING BENCHMARKS WITH BASELINE COMPARISON\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

benchmark_with_comparison("Small Encode (3 items)", "small_encode", function() use ($smallData) {
    toon_encode($smallData);
}, 5000);

benchmark_with_comparison("Medium Encode (100 items)", "medium_encode", function() use ($mediumData) {
    toon_encode($mediumData);
}, 1000);

benchmark_with_comparison("Large Encode (1000 items)", "large_encode", function() use ($largeData) {
    toon_encode($largeData);
}, 100);

// ============================================================================
// DECODE BENCHMARKS
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "DECODING BENCHMARKS WITH BASELINE COMPARISON\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";

$smallToon = toon_encode($smallData);
$mediumToon = toon_encode($mediumData);
$largeToon = toon_encode($largeData);

benchmark_with_comparison("Small Decode (3 items)", "small_decode", function() use ($smallToon) {
    toon_decode($smallToon);
}, 5000);

benchmark_with_comparison("Medium Decode (100 items)", "medium_decode", function() use ($mediumToon) {
    toon_decode($mediumToon);
}, 1000);

benchmark_with_comparison("Large Decode (1000 items)", "large_decode", function() use ($largeToon) {
    toon_decode($largeToon);
}, 100);

// ============================================================================
// SUMMARY
// ============================================================================
echo "═══════════════════════════════════════════════════════════════════\n";
echo "BASELINE REFERENCE\n";
echo "═══════════════════════════════════════════════════════════════════\n\n";
echo "The baseline figures from docs/BENCHMARKS.md are estimated averages\n";
echo "for the previous version. Current results show actual performance.\n\n";
echo "Speedup Expectations (vs pure PHP toon-php implementation):\n";
echo "  - Small operations: 3x faster\n";
echo "  - Medium operations: 10x faster\n";
echo "  - Large operations: 20x faster\n\n";

echo "═══════════════════════════════════════════════════════════════════\n";
echo "✅ Benchmark complete!\n";
echo "═══════════════════════════════════════════════════════════════════\n";

?>
