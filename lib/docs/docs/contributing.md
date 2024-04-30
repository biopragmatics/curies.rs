# 🛠️ Contributing

[![Build](https://github.com/biopragmatics/curies.rs/actions/workflows/build.yml/badge.svg)](https://github.com/biopragmatics/curies.rs/actions/workflows/build.yml) [![Lint and Test](https://github.com/biopragmatics/curies.rs/actions/workflows/test.yml/badge.svg)](https://github.com/biopragmatics/curies.rs/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/biopragmatics/curies.rs/graph/badge.svg?token=BF15PSO6GN)](https://codecov.io/gh/biopragmatics/curies.rs) [![dependency status](https://deps.rs/repo/github/biopragmatics/curies.rs/status.svg)](https://deps.rs/repo/github/biopragmatics/curies.rs)

The usual process to make a contribution is to:

1. Check for existing related [issues on GitHub](https://github.com/biopragmatics/curies.rs/issues)
2. [Fork](https://github.com/biopragmatics/curies.rs/fork) the repository and create a new branch
3. Make your changes
4. Make sure formatting, linting and tests passes.
5. Add tests if possible to cover the lines you added.
6. Commit, and send a Pull Request.


## 📥️ Clone the repository

Clone the `curies.rs` repository, `cd` into it, and create a new branch for your contribution:

```bash
git clone https://github.com/biopragmatics/curies.rs.git
cd curies.rs
```

## ⚙️ Install dependencies

[Rust](https://www.rust-lang.org/tools/install), [Python](https://www.python.org/downloads/), [NodeJS](https://nodejs.org/en/download), and [R](https://www.r-project.org/) are required for development.

Install development dependencies:

```bash
./scripts/install-dev.sh
```

!!! tip "VSCode extension"

    If you are using VSCode we strongly recommend to install the [`rust-lang.rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension.

## 🧪 Run tests

### 🦀 Test Rust crate

Run tests for all packages:

```bash
cargo test
```

!!! example "More options"

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

### 🐍 Test Python

Build the pip package, and run pytest:

```bash
./scripts/test-python.sh
```

Or just run the tests:

```bash
source .venv/bin/activate
python -m pytest python/tests/
```

### 🟨 Test JavaScript

Build the npm package, and run the jest tests in a NodeJS environment:

```bash
./scripts/test-js.sh
```

Start a web server to use the dev page:

```bash
python -m http.server 3000 --directory ./js
```

Open [localhost:3000](http://localhost:3000) in your web browser to check the browser dev page.

### 📈 Test R

Build and test R bindings:

```bash
./scripts/test-r.sh
```

The first time you will need to add the `--install` flag to install dependencies:

```bash
./scripts/test-r.sh --install
```

!!! info "Force build"

    You can force `rextendr` to re-build the bindings by making a change to one of the docstring `///` in the `/r/rust/src` code


## 🧹 Format and lint

Format code with `rustfmt`:

```bash
cargo fmt
```

Lint check with clippy:

```bash
cargo clippy --all --all-targets --all-features
```

## 📖 Generate docs locally

Build and serve:

```bash
./scripts/docs.sh
```

## ️⛓️ Check supply chain

Check the dependency supply chain, only accept dependencies with OSI or FSF approved licenses.

```bash
cargo deny check
```

Make sure dependencies are up-to-date:

```bash
cargo update
cargo outdated
```

## 🏷️ Publish a new release

!!! success "Automated release"

    Building and publishing artifacts (binaries, pip wheels, npm package) will be done automatically by the [`.github/workflows/build.yml`](https://github.com/biopragmatics/curies.rs/actions/workflows/build.yml) GitHub action when you push a new tag.

!!! warning "Set secrets for the GitHub repository"

    Make sure you have set the following tokens as secrets on GitHub for this repository: `PYPI_TOKEN`, `NPM_TOKEN`, `CRATES_IO_TOKEN`, `CODECOV_TOKEN`

To release a new version, run the release script providing the new version following [semantic versioning](https://semver.org), it will bump the version in the `Cargo.toml` files, generate the changelog from commit messages, create a new tag, and push to GitHub; the workflow will do the rest:

```bash
./scripts/release.sh 0.1.2
```
