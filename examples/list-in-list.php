<?php
/**
 * Example: List containing List
 * 
 * Demonstrates that a List containing another List is correctly encoded as a List of Lists,
 * rather than degrading into a Map.
 */

$data = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];

echo "=== List containing List ===" . PHP_EOL;
$toon = toon_encode($data);
echo $toon . PHP_EOL;

// Expected output structure should look like:
// [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
// NOT:
// {0: [1, 2, 3], 1: [4, 5, 6], 2: [7, 8, 9]}

echo PHP_EOL . "=== Decoding back ===" . PHP_EOL;
$decoded = toon_decode($toon);
print_r($decoded);

// Verify round-trip
echo PHP_EOL . "Round-trip verification: ";
echo ($data === $decoded) ? "✅ PASS" : "❌ FAIL";
echo PHP_EOL;

// More complex nested list
$complex_data = [
    ["a", "b"],
    [["c", "d"], "e"], // List inside list inside list
    "f"
];

echo PHP_EOL . "=== Complex Nested List ===" . PHP_EOL;
$toon_complex = toon_encode($complex_data);
echo $toon_complex . PHP_EOL;

$decoded_complex = toon_decode($toon_complex);
echo PHP_EOL . "Round-trip verification (complex): ";
echo ($complex_data === $decoded_complex) ? "✅ PASS" : "❌ FAIL";
echo PHP_EOL;
