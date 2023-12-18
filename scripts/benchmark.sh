#!/usr/bin/env bash
set -e

# Install:
# HYPERFINE_VERSION="1.12.0"
# wget https://github.com/sharkdp/hyperfine/releases/download/v${HYPERFINE_VERSION}/hyperfine_${HYPERFINE_VERSION}_amd64.deb
# sudo dpkg -i hyperfine_${HYPERFINE_VERSION}_amd64.deb

source .venv/bin/activate

pip install --upgrade curies

maturin develop --release -m python/Cargo.toml
maturin build --release -m python/Cargo.toml
pip install --no-index --find-links=target/wheels/ curies-rs

# m = number of run
hyperfine -m 6 --warmup 3 --export-markdown benchmark.md \
    'python scripts/benchmark_rust.py' \
    'python scripts/benchmark_python.py'


# Benchmark 1: python scripts/benchmark_rust.py
#   Time (mean ± σ):     463.6 ms ±  24.1 ms    [User: 217.4 ms, System: 38.3 ms]
#   Range (min … max):   438.4 ms … 499.7 ms    6 runs

# Benchmark 2: python scripts/benchmark_python.py
#   Time (mean ± σ):     11.232 s ±  1.557 s    [User: 10.847 s, System: 0.060 s]
#   Range (min … max):   10.094 s … 14.161 s    6 runs

# Summary
#   'python scripts/benchmark_rust.py' ran
#    24.23 ± 3.59 times faster than 'python scripts/benchmark_python.py'
