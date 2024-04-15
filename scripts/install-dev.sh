#!/usr/bin/env bash
set -e
# Script to install dependencies for development and enable pre-commit hooks

python3 -m venv .venv
source .venv/bin/activate

pip install -r python/requirements.txt
pip install -r lib/docs/requirements.txt

if [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    echo "Installing Linux specific dependency"
    maturin[patchelf]
fi

# On MacOS you might need to setup the default CRAN mirror
# echo 'options(repos=c(CRAN="https://cran.r-project.org"))' >> ~/.Rprofile

rustup update
rustup toolchain install nightly # For tarpaulin

cargo install wasm-pack cargo-tarpaulin cargo-deny cargo-outdated

pre-commit install
