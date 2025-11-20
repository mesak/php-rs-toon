<?php
/**
 * Example: Basic TOON Encoding
 * 
 * This example demonstrates how to encode a simple PHP array into TOON format.
 */

// Simple user data
$user = [
    'id' => 123,
    'name' => 'Alice Johnson',
    'email' => 'alice@example.com',
    'active' => true
];

echo "=== Basic Encoding ===" . PHP_EOL;
echo "Original PHP Array:" . PHP_EOL;
print_r($user);

echo PHP_EOL . "TOON Encoded:" . PHP_EOL;
$toon = toon_encode($user);
echo $toon . PHP_EOL;

echo PHP_EOL . "Character count: " . strlen($toon) . PHP_EOL;
echo "Lines: " . substr_count($toon, "\n") + 1 . PHP_EOL;
