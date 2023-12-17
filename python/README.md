## Curies bindings to python

https://docs.rs/pyo3/latest/pyo3

Install maturin:

```bash
python -m venv .venv
source .venv/bin/activate
pip install "maturin[patchelf]" pytest
```

## Develop

Start in dev:

```bash
maturin develop
```

Run the tests with:

```bash
python -m pytest
```

## Build

Build the wheel:

```bash
maturin build
```
