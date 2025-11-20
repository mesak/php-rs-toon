<?php

namespace Benchmark;

use HelgeSverre\Toon\Toon;
use PhpBench\Attributes as Bench;

/**
 * Benchmark comparison between php-rs-toon (Rust) extension and toon-php (pure PHP)
 */
class ToonBench
{
    private array $smallData;
    private array $mediumData;
    private array $largeData;
    private string $smallToon;
    private string $mediumToon;
    private string $largeToon;

    public function __construct()
    {
        // Small dataset
        $this->smallData = [
            'user' => [
                'id' => 123,
                'name' => 'John Doe',
                'email' => 'john@example.com'
            ]
        ];

        // Medium dataset
        $this->mediumData = [
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

        // Large dataset
        $this->largeData = [
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

        // Pre-encode for decode benchmarks
        $this->smallToon = Toon::encode($this->smallData);
        $this->mediumToon = Toon::encode($this->mediumData);
        $this->largeToon = Toon::encode($this->largeData);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(1000)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchSmallEncodeRust(): void
    {
        toon_encode($this->smallData);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(1000)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchSmallEncodePHP(): void
    {
        Toon::encode($this->smallData);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(1000)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchSmallDecodeRust(): void
    {
        toon_decode($this->smallToon);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(1000)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchSmallDecodePHP(): void
    {
        Toon::decode($this->smallToon);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(100)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchMediumEncodeRust(): void
    {
        toon_encode($this->mediumData);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(100)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchMediumEncodePHP(): void
    {
        Toon::encode($this->mediumData);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(100)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchMediumDecodeRust(): void
    {
        toon_decode($this->mediumToon);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(100)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchMediumDecodePHP(): void
    {
        Toon::decode($this->mediumToon);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(10)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchLargeEncodeRust(): void
    {
        toon_encode($this->largeData);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(10)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchLargeEncodePHP(): void
    {
        Toon::encode($this->largeData);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(10)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchLargeDecodeRust(): void
    {
        toon_decode($this->largeToon);
    }

    #[Bench\BeforeMethods('__construct')]
    #[Bench\Revs(10)]
    #[Bench\Iterations(5)]
    #[Bench\OutputTimeUnit('milliseconds', precision: 3)]
    public function benchLargeDecodePHP(): void
    {
        Toon::decode($this->largeToon);
    }
}
