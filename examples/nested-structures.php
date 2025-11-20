<?php
/**
 * Example: Nested Structures
 * 
 * Shows how TOON handles deeply nested data structures efficiently.
 */

$data = [
    'organization' => [
        'name' => 'Tech Corp',
        'founded' => 2020,
        'departments' => [
            'engineering' => [
                'head' => 'Bob Smith',
                'team_size' => 50,
                'projects' => ['Project A', 'Project B', 'Project C']
            ],
            'marketing' => [
                'head' => 'Carol White',
                'team_size' => 20,
                'campaigns' => ['Campaign 1', 'Campaign 2']
            ]
        ],
        'locations' => [
            'headquarters' => 'San Francisco',
            'branches' => ['New York', 'London', 'Tokyo']
        ]
    ]
];

echo "=== Nested Structures ===" . PHP_EOL;
$toon = toon_encode($data);
echo $toon . PHP_EOL;

echo PHP_EOL . "=== Decoding back ===" . PHP_EOL;
$decoded = toon_decode($toon);
print_r($decoded);

// Verify round-trip
echo PHP_EOL . "Round-trip verification: ";
echo ($data === $decoded) ? "✅ PASS" : "❌ FAIL";
echo PHP_EOL;
