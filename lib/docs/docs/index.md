# Introduction

[![crates.io](https://img.shields.io/crates/v/curies.svg)](https://crates.io/crates/curies)
[![PyPI](https://img.shields.io/pypi/v/curies-rs)](https://pypi.org/project/curies-rs/)
[![npm](https://img.shields.io/npm/v/@biopragmatics/curies)](https://www.npmjs.com/package/@biopragmatics/curies)

A cross-platform Rust library for idiomatic conversion between URIs and compact URIs (CURIEs).

Whether you're a developer looking to work with CURIEs (e.g. expand or compress) in your application, or a researcher seeking an efficient way to handle CURIEs, `curies` offers a suite of tools tailored to meet your needs.

## ✨ CURIEs management

- 📥 **Import converters** from JSON prefix maps or JSON-LD context, with helper functions for popular converters, such as `get_obo_converter()`, or create a custom converter programmatically.
- 🔗 **Expand CURIEs** from their compressed form to URIs.
- 🗜️ **Compress URIs** to CURIEs.

Example:

| CURIE       | URI                                                          |
| ----------- | ------------------------------------------------------------ |
| `doid:1234` | [http://purl.obolibrary.org/obo/DOID_1234](http://purl.obolibrary.org/obo/DOID_1234) |


## 📦️ Packaged for multiple interfaces

This library is packaged for easy use across various interfaces and languages:

- 🦀 **Rust developers**: available as a Rust crate `curies`
- 🐍 **Python programmers**: available as a Python pip package `curies-rs`
- 🌐 **Web developers**: available as a NPM package `@biopragmatics/curies`, compiled to [WebAssembly](https://webassembly.org/), for browser integrations with JavaScript, or NodeJS.
- 📈 **R data scientists**: soon available as a R package `curies`

## ⚔️ Cross-platform support

It runs seamlessly on x86 and ARM architectures for many platforms:

- 🐧 Linux
- 🍎 MacOS
- 🪟 Windows
- 🦊 Web browsers

> 💡 **Need Help or Have Suggestions?** We welcome your input and feedback! If you encounter any issues or have ideas to enhance this tool, please [create an issue](https://github.com/biopragmatics/curies.rs/issues) on our GitHub repository.
