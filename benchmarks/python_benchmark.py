#!/usr/bin/env python3
"""
Benchmark script for markovify (Python) vs markov-rs (Rust)

This script benchmarks the Python markovify library and compares it with
the Rust markov-rs implementation.

Requirements:
    pip install markovify pytest-benchmark

Usage:
    # Run Python benchmarks only
    python benchmarks/python_benchmark.py

    # Run comparison (requires Rust to be built)
    python benchmarks/python_benchmark.py --compare

    # Run with custom iterations
    python benchmarks/python_benchmark.py --iterations 100
"""

import argparse
import json
import os
import subprocess
import sys
import time
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

try:
    import markovify
except ImportError:
    print("Error: markovify not installed. Run: pip install markovify")
    sys.exit(1)


def load_sherlock_corpus() -> str:
    """Load the Sherlock Holmes corpus."""
    sherlock_path = Path(__file__).parent.parent / "tests" / "sherlock.txt"
    if sherlock_path.exists():
        return sherlock_path.read_text(encoding="utf-8")
    # Fallback corpus
    base = "Sherlock Holmes was a consulting detective. He lived at 221B Baker Street in London. "
    return base * 500


def load_small_corpus() -> str:
    """Load a small corpus for quick benchmarks."""
    base = (
        "The cat sat on the mat. The dog ran in the park. The bird flew over the tree. "
        "The sun was shining brightly. The children played in the garden. "
        "The teacher explained the lesson. The students listened carefully. "
        "The chef prepared a delicious meal. The guests enjoyed the food. "
        "The artist painted a beautiful picture. The audience admired the work. "
        "The musician played a wonderful song. The crowd applauded enthusiastically. "
        "The writer composed an interesting story. The readers found it captivating. "
        "The scientist conducted an important experiment. The results were significant. "
        "The engineer designed an innovative solution. The project was successful. "
        "The doctor treated the patient with care. The recovery was remarkable."
    )
    return base * 2


def load_medium_corpus() -> str:
    """Load a medium corpus for realistic benchmarks."""
    sherlock_path = Path(__file__).parent.parent / "tests" / "sherlock.txt"
    if sherlock_path.exists():
        return sherlock_path.read_text(encoding="utf-8")
    return "Sherlock Holmes was a consulting detective. " * 200


def time_function(func, *args, iterations: int = 10, **kwargs) -> Tuple[float, Any]:
    """Time a function over multiple iterations."""
    times = []
    result = None
    for _ in range(iterations):
        start = time.perf_counter()
        result = func(*args, **kwargs)
        end = time.perf_counter()
        times.append(end - start)
    
    avg_time = sum(times) / len(times)
    min_time = min(times)
    max_time = max(times)
    
    return avg_time, result, min_time, max_time


def benchmark_model_creation(corpus: str, state_size: int = 2, iterations: int = 10) -> Dict[str, float]:
    """Benchmark model creation time."""
    def create_model():
        return markovify.Text(corpus, state_size=state_size)
    
    avg_time, _, min_time, max_time = time_function(create_model, iterations=iterations)
    
    return {
        "avg_ms": avg_time * 1000,
        "min_ms": min_time * 1000,
        "max_ms": max_time * 1000,
        "iterations": iterations,
    }


def benchmark_sentence_generation(model: markovify.Text, iterations: int = 100) -> Dict[str, float]:
    """Benchmark sentence generation time."""
    def make_sentence():
        return model.make_sentence()
    
    avg_time, results, min_time, max_time = time_function(make_sentence, iterations=iterations)
    successful = sum(1 for r in results if r is not None)
    
    return {
        "avg_ms": avg_time * 1000,
        "min_ms": min_time * 1000,
        "max_ms": max_time * 1000,
        "iterations": iterations,
        "successful": successful,
        "success_rate": successful / iterations * 100,
    }


def benchmark_compiled_generation(model: markovify.Text, iterations: int = 100) -> Dict[str, float]:
    """Benchmark sentence generation with compiled model."""
    compiled = model.compile()
    
    def make_sentence():
        return compiled.make_sentence()
    
    avg_time, results, min_time, max_time = time_function(make_sentence, iterations=iterations)
    successful = sum(1 for r in results if r is not None)
    
    return {
        "avg_ms": avg_time * 1000,
        "min_ms": min_time * 1000,
        "max_ms": max_time * 1000,
        "iterations": iterations,
        "successful": successful,
        "success_rate": successful / iterations * 100,
    }


def benchmark_model_compilation(model: markovify.Text, iterations: int = 10) -> Dict[str, float]:
    """Benchmark model compilation time."""
    def compile_model():
        return model.compile()
    
    avg_time, _, min_time, max_time = time_function(compile_model, iterations=iterations)
    
    return {
        "avg_ms": avg_time * 1000,
        "min_ms": min_time * 1000,
        "max_ms": max_time * 1000,
        "iterations": iterations,
    }


def benchmark_json_serialization(model: markovify.Text, iterations: int = 10) -> Dict[str, float]:
    """Benchmark JSON serialization/deserialization."""
    def to_json():
        return model.to_json()
    
    def from_json(json_str):
        return markovify.Text.from_json(json_str)
    
    # Benchmark to_json
    avg_time_to, json_str, min_time_to, max_time_to = time_function(to_json, iterations=iterations)
    
    # Benchmark from_json
    avg_time_from, _, min_time_from, max_time_from = time_function(from_json, json_str, iterations=iterations)
    
    return {
        "to_json": {
            "avg_ms": avg_time_to * 1000,
            "min_ms": min_time_to * 1000,
            "max_ms": max_time_to * 1000,
        },
        "from_json": {
            "avg_ms": avg_time_from * 1000,
            "min_ms": min_time_from * 1000,
            "max_ms": max_time_from * 1000,
        },
        "iterations": iterations,
    }


def benchmark_model_combination(
    model_a: markovify.Text,
    model_b: markovify.Text,
    iterations: int = 10
) -> Dict[str, float]:
    """Benchmark model combination."""
    def combine_equal():
        return markovify.combine([model_a, model_b])
    
    def combine_weighted():
        return markovify.combine([model_a, model_b], [1.5, 1.0])
    
    avg_time_equal, _, min_time_equal, max_time_equal = time_function(combine_equal, iterations=iterations)
    avg_time_weighted, _, min_time_weighted, max_time_weighted = time_function(combine_weighted, iterations=iterations)
    
    return {
        "equal_weights": {
            "avg_ms": avg_time_equal * 1000,
            "min_ms": min_time_equal * 1000,
            "max_ms": max_time_equal * 1000,
        },
        "weighted": {
            "avg_ms": avg_time_weighted * 1000,
            "min_ms": min_time_weighted * 1000,
            "max_ms": max_time_weighted * 1000,
        },
        "iterations": iterations,
    }


def benchmark_short_sentence(model: markovify.Text, max_chars: int = 100, iterations: int = 50) -> Dict[str, float]:
    """Benchmark short sentence generation."""
    def make_short():
        return model.make_short_sentence(max_chars)
    
    avg_time, results, min_time, max_time = time_function(make_short, iterations=iterations)
    successful = sum(1 for r in results if r is not None)
    
    return {
        "avg_ms": avg_time * 1000,
        "min_ms": min_time * 1000,
        "max_ms": max_time * 1000,
        "iterations": iterations,
        "successful": successful,
        "success_rate": successful / iterations * 100,
    }


def benchmark_sentence_with_start(model: markovify.Text, start: str, iterations: int = 50) -> Dict[str, float]:
    """Benchmark sentence generation with specific start."""
    def make_with_start():
        try:
            return model.make_sentence_with_start(start)
        except Exception:
            return None
    
    avg_time, results, min_time, max_time = time_function(make_with_start, iterations=iterations)
    successful = sum(1 for r in results if r is not None)
    
    return {
        "avg_ms": avg_time * 1000,
        "min_ms": min_time * 1000,
        "max_ms": max_time * 1000,
        "iterations": iterations,
        "successful": successful,
        "success_rate": successful / iterations * 100,
    }


def run_python_benchmarks(iterations: int = 10) -> Dict[str, Any]:
    """Run all Python benchmarks."""
    print("=" * 60)
    print("Python markovify Benchmarks")
    print("=" * 60)
    
    results = {}
    
    # Small corpus benchmarks
    print("\n[1] Model Creation (Small Corpus)")
    small_corpus = load_small_corpus()
    results["model_creation_small"] = benchmark_model_creation(
        small_corpus, state_size=2, iterations=iterations
    )
    print(f"    Avg: {results['model_creation_small']['avg_ms']:.2f} ms")
    
    # Medium corpus benchmarks
    print("\n[2] Model Creation (Medium Corpus)")
    medium_corpus = load_medium_corpus()
    results["model_creation_medium"] = benchmark_model_creation(
        medium_corpus, state_size=2, iterations=iterations // 2
    )
    print(f"    Avg: {results['model_creation_medium']['avg_ms']:.2f} ms")
    
    # Create model for subsequent benchmarks
    print("\n[3] Creating model for generation benchmarks...")
    model = markovify.Text(medium_corpus, state_size=2)
    
    # Sentence generation
    print("\n[4] Sentence Generation")
    results["sentence_generation"] = benchmark_sentence_generation(
        model, iterations=iterations * 10
    )
    print(f"    Avg: {results['sentence_generation']['avg_ms']:.2f} ms")
    print(f"    Success Rate: {results['sentence_generation']['success_rate']:.1f}%")
    
    # Compiled generation
    print("\n[5] Compiled Sentence Generation")
    results["compiled_generation"] = benchmark_compiled_generation(
        model, iterations=iterations * 10
    )
    print(f"    Avg: {results['compiled_generation']['avg_ms']:.2f} ms")
    print(f"    Success Rate: {results['compiled_generation']['success_rate']:.1f}%")
    
    # Model compilation
    print("\n[6] Model Compilation")
    results["model_compilation"] = benchmark_model_compilation(
        model, iterations=iterations
    )
    print(f"    Avg: {results['model_compilation']['avg_ms']:.2f} ms")
    
    # Short sentence
    print("\n[7] Short Sentence Generation (max 100 chars)")
    results["short_sentence"] = benchmark_short_sentence(
        model, max_chars=100, iterations=iterations * 5
    )
    print(f"    Avg: {results['short_sentence']['avg_ms']:.2f} ms")
    print(f"    Success Rate: {results['short_sentence']['success_rate']:.1f}%")
    
    # Sentence with start
    print("\n[8] Sentence With Start ('Sherlock')")
    results["sentence_with_start"] = benchmark_sentence_with_start(
        model, start="Sherlock", iterations=iterations * 5
    )
    print(f"    Avg: {results['sentence_with_start']['avg_ms']:.2f} ms")
    print(f"    Success Rate: {results['sentence_with_start']['success_rate']:.1f}%")
    
    # JSON serialization
    print("\n[9] JSON Serialization")
    results["json_serialization"] = benchmark_json_serialization(
        model, iterations=iterations
    )
    print(f"    to_json Avg: {results['json_serialization']['to_json']['avg_ms']:.2f} ms")
    print(f"    from_json Avg: {results['json_serialization']['from_json']['avg_ms']:.2f} ms")
    
    # Model combination
    print("\n[10] Model Combination")
    model_b = markovify.Text(medium_corpus, state_size=2)
    results["model_combination"] = benchmark_model_combination(
        model, model_b, iterations=iterations
    )
    print(f"    Equal Weights Avg: {results['model_combination']['equal_weights']['avg_ms']:.2f} ms")
    print(f"    Weighted Avg: {results['model_combination']['weighted']['avg_ms']:.2f} ms")
    
    return results


def run_rust_benchmarks() -> Optional[Dict[str, Any]]:
    """Run Rust benchmarks and parse results."""
    print("\n" + "=" * 60)
    print("Rust markov-rs Benchmarks")
    print("=" * 60)
    
    bench_dir = Path(__file__).parent.parent
    try:
        # Run cargo bench with --no-run first to compile
        print("Compiling Rust benchmarks...")
        subprocess.run(
            ["cargo", "bench", "--no-run"],
            cwd=bench_dir,
            check=True,
            capture_output=True,
        )
        
        # Run actual benchmarks
        print("Running Rust benchmarks...")
        result = subprocess.run(
            ["cargo", "bench"],
            cwd=bench_dir,
            check=True,
            capture_output=True,
            text=True,
        )
        
        # Parse output
        return parse_rust_bench_output(result.stdout)
        
    except subprocess.CalledProcessError as e:
        print(f"Error running Rust benchmarks: {e}")
        print(f"stderr: {e.stderr}")
        return None
    except FileNotFoundError:
        print("Cargo not found. Make sure Rust is installed.")
        return None


def parse_rust_bench_output(output: str) -> Dict[str, Any]:
    """Parse criterion benchmark output."""
    results = {}
    
    # Simple parsing - look for benchmark names and times
    lines = output.split('\n')
    current_bench = None
    
    for line in lines:
        line = line.strip()
        
        # Look for benchmark names
        if 'time' in line and ('ns' in line or 'ms' in line):
            parts = line.split()
            if len(parts) >= 2:
                # Extract time value
                time_str = parts[-2] if parts[-1] in ['ns', 'ms'] else parts[-1]
                unit = parts[-1] if parts[-1] in ['ns', 'ms'] else 'ms'
                
                try:
                    time_val = float(time_str.replace(',', ''))
                    if unit == 'ns':
                        time_val /= 1_000_000  # Convert to ms
                    
                    if current_bench:
                        results[current_bench] = time_val
                except ValueError:
                    pass
        
        # Track current benchmark
        if 'model_creation' in line or 'sentence_generation' in line or \
           'compilation' in line or 'json' in line or 'combination' in line:
            current_bench = line.split()[0] if line else None
    
    return results


def print_comparison(python_results: Dict, rust_results: Optional[Dict]):
    """Print comparison between Python and Rust results."""
    print("\n" + "=" * 60)
    print("Performance Comparison")
    print("=" * 60)
    
    if not rust_results:
        print("\nRust benchmarks not available for comparison.")
        return
    
    print("\nNote: Direct comparison requires matching benchmark configurations.")
    print("For accurate results, run 'cargo bench' separately and compare outputs.")
    
    # Print summary
    print("\n" + "-" * 60)
    print("Python Results Summary:")
    print("-" * 60)
    
    if 'model_creation_medium' in python_results:
        print(f"Model Creation (medium):     {python_results['model_creation_medium']['avg_ms']:.2f} ms")
    if 'sentence_generation' in python_results:
        print(f"Sentence Generation:         {python_results['sentence_generation']['avg_ms']:.4f} ms")
    if 'compiled_generation' in python_results:
        print(f"Compiled Generation:         {python_results['compiled_generation']['avg_ms']:.4f} ms")
    if 'model_compilation' in python_results:
        print(f"Model Compilation:           {python_results['model_compilation']['avg_ms']:.2f} ms")
    if 'json_serialization' in python_results:
        to_json = python_results['json_serialization']['to_json']['avg_ms']
        from_json = python_results['json_serialization']['from_json']['avg_ms']
        print(f"JSON Serialize:              {to_json:.2f} ms")
        print(f"JSON Deserialize:            {from_json:.2f} ms")


def save_results(python_results: Dict, rust_results: Optional[Dict], output_file: str):
    """Save benchmark results to JSON file."""
    results = {
        "python": python_results,
        "rust": rust_results,
    }
    
    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"\nResults saved to: {output_file}")


def main():
    parser = argparse.ArgumentParser(description="Benchmark markovify vs markov-rs")
    parser.add_argument(
        "--iterations", "-i",
        type=int,
        default=10,
        help="Number of iterations for each benchmark"
    )
    parser.add_argument(
        "--compare", "-c",
        action="store_true",
        help="Also run Rust benchmarks for comparison"
    )
    parser.add_argument(
        "--output", "-o",
        type=str,
        help="Save results to JSON file"
    )
    
    args = parser.parse_args()
    
    # Run Python benchmarks
    python_results = run_python_benchmarks(iterations=args.iterations)
    
    # Optionally run Rust benchmarks
    rust_results = None
    if args.compare:
        rust_results = run_rust_benchmarks()
    
    # Print comparison
    print_comparison(python_results, rust_results)
    
    # Save results if requested
    if args.output:
        save_results(python_results, rust_results, args.output)
    
    print("\n" + "=" * 60)
    print("Benchmarks complete!")
    print("=" * 60)


if __name__ == "__main__":
    main()
