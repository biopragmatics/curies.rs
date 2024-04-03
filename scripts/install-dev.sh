#!/usr/bin/env bash
set -e
# Script to install dependencies for development and enable pre-commit hooks

python3 -m venv .venv
source .venv/bin/activate

pip install -r python/requirements.txt
pip install -r lib/docs/requirements.txt

rustup update
rustup toolchain install nightly # For tarpaulin

cargo install wasm-pack cargo-tarpaulin cargo-make

pre-commit install
