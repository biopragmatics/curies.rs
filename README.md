<h1 align="center">
  🦀 curies.rs
</h1>

<p align="center">
    <a href="https://github.com/biopragmatics/curies.rs/actions/workflows/test.yml">
        <img alt="Test" src="https://github.com/biopragmatics/curies.rs/actions/workflows/test.yml/badge.svg" />
    </a>
    <a href="https://github.com/biopragmatics/curies.rs/actions/workflows/build.yml">
        <img alt="Build" src="https://github.com/biopragmatics/curies.rs/actions/workflows/build.yml/badge.svg" />
    </a>
    <a href="https://deps.rs/repo/github/biopragmatics/curies.rs">
        <img src="https://deps.rs/repo/github/biopragmatics/curies.rs/status.svg" alt="Dependency status" />
    </a>
    <a href="https://github.com/biopragmatics/curies.rs/blob/main/LICENSE">
        <img alt="MIT license" src="https://img.shields.io/badge/License-MIT-brightgreen.svg" />
    </a>
    <!-- a href="https://codecov.io/gh/biopragmatics/curies.rs/branch/main">
        <img src="https://codecov.io/gh/biopragmatics/curies.rs/branch/main/graph/badge.svg" alt="Codecov status" />
    </a -->
</p>

Idiomatic conversion between URIs and compact URIs (CURIEs) in Rust.

## 📥 Install dependencies

[Rust](https://www.rust-lang.org/tools/install) is required for development.

```bash
rustup update
rustup component add rustfmt clippy
cargo install wasm-pack cargo-tarpaulin mdbook mdbook-admonish
```

> If you are using VSCode we strongly recommend to install the `rust-lang.rust-analyzer` extension.


## Development

### 🧪 Run tests

Run tests and display prints:

```shell
cargo test -- --nocapture
```

### 🧹 Format

```shell
cargo fmt
```

### 📖 Documentation

```shell
cargo doc --open
```
