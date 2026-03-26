#!/bin/bash
# Benchmark runner script for markov-rs vs markovify
#
# This script runs both Rust and Python benchmarks and generates a comparison report.
#
# Usage:
#   ./run_benchmarks.sh [--iterations N] [--output report.json]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Default values
ITERATIONS=10
OUTPUT_FILE=""
RUN_PYTHON=true
RUN_RUST=true
COMPARE=true

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --iterations|-i)
            ITERATIONS="$2"
            shift 2
            ;;
        --output|-o)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --python-only)
            RUN_RUST=false
            shift
            ;;
        --rust-only)
            RUN_PYTHON=false
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --iterations, -i N    Number of iterations (default: 10)"
            echo "  --output, -o FILE     Save results to JSON file"
            echo "  --python-only         Run only Python benchmarks"
            echo "  --rust-only           Run only Rust benchmarks"
            echo "  --help, -h            Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo "============================================================"
echo "  Markov Chain Performance Benchmarks"
echo "  Python (markovify) vs Rust (markov-rs)"
echo "============================================================"
echo ""
echo "Configuration:"
echo "  Iterations: $ITERATIONS"
echo "  Run Python: $RUN_PYTHON"
echo "  Run Rust:   $RUN_RUST"
echo ""

# Create temp directory for results
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

PYTHON_RESULTS="$TEMP_DIR/python_results.json"
RUST_RESULTS="$TEMP_DIR/rust_results.json"

# Run Python benchmarks
if [ "$RUN_PYTHON" = true ]; then
    echo "============================================================"
    echo "  Running Python Benchmarks"
    echo "============================================================"
    
    # Check if markovify is installed
    if ! python3 -c "import markovify" 2>/dev/null; then
        echo "Error: markovify not installed. Installing..."
        pip3 install markovify
    fi
    
    python3 benchmarks/python_benchmark.py \
        --iterations "$ITERATIONS" \
        --output "$PYTHON_RESULTS"
    
    echo ""
fi

# Run Rust benchmarks
if [ "$RUN_RUST" = true ]; then
    echo "============================================================"
    echo "  Running Rust Benchmarks"
    echo "============================================================"
    
    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        echo "Error: cargo not found. Please install Rust."
        exit 1
    fi
    
    echo "Running cargo bench..."
    cargo bench 2>&1 | tee "$TEMP_DIR/rust_output.txt"
    
    echo ""
fi

# Generate comparison report
echo "============================================================"
echo "  Generating Comparison Report"
echo "============================================================"
echo ""

# Create comparison script
cat > "$TEMP_DIR/compare.py" << 'PYTHON_SCRIPT'
#!/usr/bin/env python3
import json
import sys
from pathlib import Path

def format_time(ms):
    """Format time in human-readable format."""
    if ms < 0.001:
        return f"{ms * 1000000:.1f} ns"
    elif ms < 1:
        return f"{ms * 1000:.2f} μs"
    else:
        return f"{ms:.2f} ms"

def calculate_speedup(python_ms, rust_ms):
    """Calculate speedup factor."""
    if rust_ms == 0:
        return float('inf')
    return python_ms / rust_ms

def main():
    python_file = sys.argv[1] if len(sys.argv) > 1 else None
    rust_output = sys.argv[2] if len(sys.argv) > 2 else None
    
    # Load Python results
    python_results = {}
    if python_file and Path(python_file).exists():
        with open(python_file) as f:
            python_results = json.load(f)
    
    # Parse Rust output
    rust_results = {}
    if rust_output and Path(rust_output).exists():
        content = Path(rust_output).read_text()
        # Simple parsing for criterion output
        for line in content.split('\n'):
            if 'time' in line and ('ns' in line or 'ms' in line):
                parts = line.strip().split()
                if len(parts) >= 3:
                    try:
                        time_val = float(parts[-2].replace(',', ''))
                        unit = parts[-1]
                        if unit == 'ns':
                            time_val /= 1_000_000
                        elif unit == 'μs':
                            time_val /= 1_000
                        rust_results[parts[0]] = time_val
                    except (ValueError, IndexError):
                        pass
    
    # Print report
    print("=" * 70)
    print("  PERFORMANCE COMPARISON REPORT")
    print("=" * 70)
    print()
    
    if python_results:
        print("Python (markovify) Results:")
        print("-" * 70)
        
        if 'model_creation_medium' in python_results.get('python', python_results):
            mc = python_results.get('python', python_results)['model_creation_medium']
            print(f"  Model Creation (medium):      {format_time(mc['avg_ms'])}")
        
        if 'sentence_generation' in python_results.get('python', python_results):
            sg = python_results.get('python', python_results)['sentence_generation']
            print(f"  Sentence Generation:          {format_time(sg['avg_ms'])}")
            print(f"    Success Rate:               {sg['success_rate']:.1f}%")
        
        if 'compiled_generation' in python_results.get('python', python_results):
            cg = python_results.get('python', python_results)['compiled_generation']
            print(f"  Compiled Generation:          {format_time(cg['avg_ms'])}")
            print(f"    Success Rate:               {cg['success_rate']:.1f}%")
        
        if 'model_compilation' in python_results.get('python', python_results):
            mc = python_results.get('python', python_results)['model_compilation']
            print(f"  Model Compilation:            {format_time(mc['avg_ms'])}")
        
        if 'json_serialization' in python_results.get('python', python_results):
            js = python_results.get('python', python_results)['json_serialization']
            print(f"  JSON Serialization:           {format_time(js['to_json']['avg_ms'])}")
            print(f"  JSON Deserialization:         {format_time(js['from_json']['avg_ms'])}")
        
        print()
    
    if rust_results:
        print("Rust (markov-rs) Results:")
        print("-" * 70)
        for name, time_ms in rust_results.items():
            print(f"  {name:30s} {format_time(time_ms)}")
        print()
    
    print("=" * 70)
    print("  NOTES")
    print("=" * 70)
    print("""
  - Lower times are better
  - Rust benchmarks use criterion for statistical accuracy
  - Python benchmarks use timeit-style averaging
  - For detailed analysis, see:
    - Rust: target/criterion/report/index.html
    - Python: See --output JSON file
  """)

if __name__ == "__main__":
    main()
PYTHON_SCRIPT

python3 "$TEMP_DIR/compare.py" "$PYTHON_RESULTS" "$TEMP_DIR/rust_output.txt"

# Save combined results if output file specified
if [ -n "$OUTPUT_FILE" ]; then
    echo "Saving results to: $OUTPUT_FILE"
    
    cat > "$TEMP_DIR/combine.py" << COMBINE_SCRIPT
#!/usr/bin/env python3
import json
import sys
from pathlib import Path

python_file = sys.argv[1]
rust_output = sys.argv[2]
output_file = sys.argv[3]

# Load Python results
python_results = {}
if Path(python_file).exists():
    with open(python_file) as f:
        python_results = json.load(f)

# Create combined results
combined = {
    "python": python_results.get("python", python_results) if python_results else {},
    "rust": {},
    "metadata": {
        "iterations": $ITERATIONS,
    }
}

with open(output_file, 'w') as f:
    json.dump(combined, f, indent=2)

print(f"Results saved to {output_file}")
COMBINE_SCRIPT

    python3 "$TEMP_DIR/combine.py" "$PYTHON_RESULTS" "$TEMP_DIR/rust_output.txt" "$OUTPUT_FILE"
fi

echo ""
echo "============================================================"
echo "  Benchmarks Complete!"
echo "============================================================"
