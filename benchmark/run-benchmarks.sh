#!/bin/bash
set -e

echo "🚀 PHP TOON Extension Benchmark Suite"
echo "======================================"
echo ""

# Check if php-rs-toon extension is loaded
if ! php -m | grep -q "php-rs-toon"; then
    echo "❌ Error: php-rs-toon extension is not loaded!"
    echo "Please ensure the extension is installed and enabled in php.ini"
    exit 1
fi

echo "✅ php-rs-toon extension is loaded"
echo ""

# Install dependencies
if [ ! -d "vendor" ]; then
    echo "📦 Installing dependencies..."
    composer install --no-dev --optimize-autoloader
    echo ""
fi

# Run benchmarks
echo "🏃 Running benchmarks..."
echo ""

vendor/bin/phpbench run benchmarks/ \
    --report=default \
    --output=console \
    --output=markdown \
    --output-file=results.md

echo ""
echo "✅ Benchmarks complete!"
echo "📊 Results saved to results.md"
echo ""

# Display summary
if [ -f "results.md" ]; then
    echo "📈 Benchmark Summary:"
    echo "==================="
    tail -n 20 results.md
fi
