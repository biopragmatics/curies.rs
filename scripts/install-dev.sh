#!/usr/bin/env bash
set -e
# Script to install dependencies for development and enable pre-commit hooks

python3 -m venv .venv
source .venv/bin/activate

pip install "maturin[patchelf]" pre-commit

rustup update
rustup toolchain install nightly # For tarpaulin
rustup component add rustfmt clippy

cargo install wasm-pack cargo-tarpaulin cargo-make

source scripts/docs-install.sh

pre-commit install
