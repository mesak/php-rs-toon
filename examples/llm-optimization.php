<?php
/**
 * Example: LLM Token Optimization
 * 
 * Demonstrates token savings when using TOON vs JSON for LLM payloads.
 */

// Sample dataset that might be sent to an LLM
$customers = array_map(function($i) {
    return [
        'id' => $i,
        'name' => "Customer $i",
        'email' => "customer$i@example.com",
        'status' => 'active',
        'balance' => round(rand(1000, 10000) / 100, 2),
        'last_order' => '2024-01-15'
    ];
}, range(1, 50));

$data = ['customers' => $customers];

echo "=== LLM Token Optimization ===" . PHP_EOL . PHP_EOL;

// JSON encoding
$json = json_encode($data, JSON_PRETTY_PRINT);
$jsonSize = strlen($json);
$jsonLines = substr_count($json, "\n") + 1;

echo "JSON Format:" . PHP_EOL;
echo "- Size: $jsonSize bytes" . PHP_EOL;
echo "- Lines: $jsonLines" . PHP_EOL;
echo "- Estimated tokens: ~" . ceil($jsonSize / 4) . PHP_EOL;

// TOON encoding
$toon = toon_encode($data);
$toonSize = strlen($toon);
$toonLines = substr_count($toon, "\n") + 1;

echo PHP_EOL . "TOON Format:" . PHP_EOL;
echo "- Size: $toonSize bytes" . PHP_EOL;
echo "- Lines: $toonLines" . PHP_EOL;
echo "- Estimated tokens: ~" . ceil($toonSize / 4) . PHP_EOL;

// Savings
$savings = round((1 - $toonSize / $jsonSize) * 100, 1);
$tokenSavings = ceil($jsonSize / 4) - ceil($toonSize / 4);

echo PHP_EOL . "Savings:" . PHP_EOL;
echo "- Size reduction: {$savings}%" . PHP_EOL;
echo "- Token savings: ~{$tokenSavings} tokens" . PHP_EOL;
echo "- Cost impact: Reduces API costs by {$savings}%" . PHP_EOL;

echo PHP_EOL . "Sample TOON output (first 20 lines):" . PHP_EOL;
echo "---" . PHP_EOL;
$lines = explode("\n", $toon);
echo implode("\n", array_slice($lines, 0, 20)) . PHP_EOL;
if (count($lines) > 20) {
    echo "... (" . (count($lines) - 20) . " more lines)" . PHP_EOL;
}
