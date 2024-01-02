# 🛠️ Contributing

The usual process to make a contribution is to:

1. Check for existing related [issues on GitHub](https://github.com/biopragmatics/curies.rs/issues)
2. [Fork](https://github.com/biopragmatics/curies.rs/fork) the repository and create a new branch
3. Make your changes
4. Make sure formatting, linting and tests passes.
5. Add tests if possible to cover the lines you added.
6. Commit, and send a Pull Request.

## ️🗺️ Architecture details

### 🗃️ Folder structure

```
curies.rs/
├── lib/
│   ├── src/
│   │   └── 🦀 Source code for the core Rust crate.
│   ├── tests/
│   │   └── 🧪 Tests for the core Rust crate.
│   └── docs/
│       └── 📖 Markdown and HTML files for the documentation website.
├── python/
│   └── 🐍 Python bindings for interacting with the Rust crate.
├── js/
│   └── 🌐 JavaScript bindings for integrating into JS environments.
├── scripts/
│   └── 🛠️ Development scripts (build docs, testing).
└── .github/
    └── workflows/
        └── ⚙️ Automated CI/CD workflows.
```

## 🧑‍💻 Development workflow

[![Build](https://github.com/biopragmatics/curies.rs/actions/workflows/build.yml/badge.svg)](https://github.com/biopragmatics/curies.rs/actions/workflows/build.yml) [![Lint and Test](https://github.com/biopragmatics/curies.rs/actions/workflows/test.yml/badge.svg)](https://github.com/biopragmatics/curies.rs/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/biopragmatics/curies.rs/graph/badge.svg?token=BF15PSO6GN)](https://codecov.io/gh/biopragmatics/curies.rs) [![dependency status](https://deps.rs/repo/github/biopragmatics/curies.rs/status.svg)](https://deps.rs/repo/github/biopragmatics/curies.rs)

[Rust](https://www.rust-lang.org/tools/install), python, and NodeJS are required for development.

> If you are using VSCode we strongly recommend to install the `rust-lang.rust-analyzer` extension.

Install development dependencies:

```bash
# Activate python virtual env
python3 -m venv .venv
source .venv/bin/activate
# Install python dependencies
pip install maturin
# Install rust dev tools
rustup update
rustup component add rustfmt clippy
cargo install wasm-pack cargo-tarpaulin mdbook mdbook-admonish cargo-deny
```

### 📥️ Clone the repository

Clone the `curies.rs` repository, `cd` into it, and create a new branch for your contribution:

```bash
cd curies.rs
git checkout -b add-my-contribution
```

### 🧪 Run tests

Run tests for all packages:

```bash
cargo test
```

Display prints:

```bash
cargo test -- --nocapture
```

Run a specific test:

```bash
cargo test new_empty_converter -- --nocapture
```

If tests panic without telling on which test it failed, use:

```bash
cargo test -- --test-threads=1
```

Test the `curies` crate with code coverage:

```bash
./scripts/cov.sh
```

### 🐍 Run python

Build the pip package, and run the `python/try.py` script:

```bash
./scripts/build-python.sh
```

Or just run the tests:

```bash
source .venv/bin/activate
python -m pytest python/tests/
```

### 🟨 Run JavaScript

Build the npm package, and run the TypeScript tests in a NodeJS environment:

```bash
./scripts/build-js.py
```

Start a web server:

```bash
python -m http.server 3000 --directory ./js
```

Open [localhost:3000](http://localhost:3000) in your web browser to check the browser version.

### ✨ Format

```bash
cargo fmt
```

### 🧹 Lint

```bash
cargo clippy --all --all-targets --all-features
```

### 📖 Generate docs locally

Install dependencies:

```bash
./scripts/docs-install.sh
```

Build and serve:

```bash
./scripts/docs-serve.sh
```

### ️⛓️ Check supply chain

Check the dependency supply chain, only accept dependencies with OSI or FSF approved licenses.

```bash
cargo deny check
```

### 🏷️ New release

Publishing artifacts will be done by the `build.yml` workflow, make sure you have set the following tokens as secrets
for this repository: `PYPI_TOKEN`, `NPM_TOKEN`, `CRATES_IO_TOKEN`, `CODECOV_TOKEN`

1. Install dependency:

   ```bash
   cargo install cargo-outdated
   ```

2. Make sure dependencies have been updated:

   ```bash
   cargo update
   cargo outdated
   ```

3. Bump the version in the `Cargo.toml` file in folders `lib`, `python`, and `js`:

   ```bash
   ./scripts/bump.sh 0.1.2
   ```

4. Commit, push, and **create a new release on GitHub**.

5. The `build.yml` workflow will automatically build artifacts (pip wheel, npm package), add them to the new release,
   and publish to public registries (crates.io, PyPI, NPM).
