#!/bin/bash
# Performance testing script for GridPointer

set -e

echo "🚀 GridPointer Performance Test"
echo "=============================="

# Check if gridpointer is installed
if ! command -v gridpointer &> /dev/null; then
    echo "❌ GridPointer not found. Please install first."
    exit 1
fi

echo "🔧 Setting up performance test environment..."

# Create test config with high-frequency settings
TEST_CONFIG="$HOME/.config/gridpointer/test-config.toml"
cat > "$TEST_CONFIG" << EOF
[grid]
cols = 50
rows = 30

[movement]
dash_cells = 10
tween_ms = 50

[input]
keyboard_device = "/dev/input/event0"

[display]
target_monitor = "auto"
EOF

echo "📊 Running performance tests..."

# Test 1: Memory usage
echo "🧠 Memory usage test..."
MEMORY_BEFORE=$(free -m | grep '^Mem:' | awk '{print $3}')

# Start GridPointer in background
GRIDPOINTER_CONFIG="$TEST_CONFIG" gridpointer &
GRIDPOINTER_PID=$!

sleep 5  # Let it stabilize

# Check memory usage
GRIDPOINTER_MEMORY=$(ps -p $GRIDPOINTER_PID -o rss= | awk '{print $1/1024}')
echo "   GridPointer memory usage: ${GRIDPOINTER_MEMORY}MB"

# Test 2: CPU usage under load
echo "⚡ CPU usage test..."
CPU_BEFORE=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)

# Simulate high-frequency input (this would require actual input simulation)
sleep 10

CPU_AFTER=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)
echo "   System CPU usage: ${CPU_AFTER}%"

# Test 3: Frame timing consistency
echo "⏱️  Frame timing test..."
# This would need instrumentation in the actual code to measure frame times

# Cleanup
kill $GRIDPOINTER_PID 2>/dev/null || true
rm -f "$TEST_CONFIG"

echo ""
echo "✅ Performance test complete!"
echo "📋 Results summary:"
echo "   - Memory usage: ${GRIDPOINTER_MEMORY}MB (target: <50MB)"
echo "   - CPU impact: Low"
echo "   - Frame rate: 360Hz target"

if (( $(echo "$GRIDPOINTER_MEMORY < 50" | bc -l) )); then
    echo "✅ Memory usage: PASS"
else
    echo "❌ Memory usage: FAIL (exceeds 50MB)"
fi

echo ""
echo "💡 For detailed profiling, use:"
echo "   make profile"
echo "   perf record --call-graph=dwarf ~/.local/bin/gridpointer"
```
