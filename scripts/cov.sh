#!/usr/bin/env bash
set -e

cargo tarpaulin -p curies --doc --tests --out html --out xml --timeout 120

# --exclude-files lib/src/error.rs
# python -m http.server 3000 --directory .
