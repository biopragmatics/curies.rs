#!/usr/bin/env bash
set -e

source .venv/bin/activate
cd python

maturin develop

python -m pip install pytest
python -m pytest
