# ℹ️ Introduction

[![crates.io](https://img.shields.io/crates/v/curies.svg)](https://crates.io/crates/curies)
[![PyPI](https://img.shields.io/pypi/v/curies-rs)](https://pypi.org/project/curies-rs/)
[![npm](https://img.shields.io/npm/v/@biopragmatics/curies)](https://www.npmjs.com/package/@biopragmatics/curies)
[![Tests](https://github.com/biopragmatics/curies.rs/actions/workflows/test.yml/badge.svg)](https://github.com/biopragmatics/curies.rs/actions/workflows/test.yml)
[![Build](https://github.com/biopragmatics/curies.rs/actions/workflows/build.yml/badge.svg)](https://github.com/biopragmatics/curies.rs/actions/workflows/build.yml)

A cross-platform library for idiomatic conversion between URIs and compact URIs (CURIEs).

Uniform resource identifiers (URIs) and compact URIs (CURIEs) have become the predominant syntaxes for identifying concepts in linked data applications. Therefore, efficient, faultless, and idiomatic conversion between them is a crucial low-level utility whose need is ubiquitous across many codebases.

[`curies`](https://curies.readthedocs.io/en/latest/api.html#module-curies) fills this need. This cross-platform package can be used by a variety of people:

1. **Data Scientist** - someone who consumes and modifies data to suit an analysis or application. For example, they might want to convert tabular data containing CURIEs into IRIs, translate into RDF, then query with SPARQL.
2. **Curator** - someone who creates data. For example, an ontologist may want to curate using CURIEs but have their toolchain 1) validate the syntax and semantics and 2) convert to IRIs for their data persistence
3. **Data Consumer** - someone who consumes data. This kind of user likely won’t interact with [`curies`](https://curies.readthedocs.io/en/latest/api.html#module-curies) directly, but will likely use tools that build on top of it. For example, someone using the Bioregistry resolution service uses this package’s expansion utilities indirectly.
4. **Software Developer** - someone who develops tools to support data creators, data consumers, and other software developers. For example, a software developer might want to make their toolchain more generic for loading, merging, and outputting prefix maps and extended prefix maps.

For many users, expansion (CURIE to URI) and contraction (URI to CURIE) are the two most important tools. Example:

| CURIE       | URI                                                          |
| ----------- | ------------------------------------------------------------ |
| `doid:1234` | [http://purl.obolibrary.org/obo/DOID_1234](http://purl.obolibrary.org/obo/DOID_1234) |


## ✨ Features

- 📥 **Import converters** from JSON prefix maps or JSON-LD context, with helper functions for popular converters, such as `get_obo_converter()`, or create a custom converter programmatically.
- 🔗 **Expand CURIEs** from their compressed form to URIs.
- 🗜️ **Compress URIs** to CURIEs.
- 🧩 **Standardize** prefixes, CURIEs, or URIs.

## 📦️ Packaged for multiple interfaces

This library is packaged for easy use across various interfaces and languages:

- 🦀 **Rust developers**: available as a Rust crate `curies`
- 🐍 **Python programmers**: available as a Python pip package `curies-rs`
- 🌐 **JavaScript web developers**: available as a NPM package `@biopragmatics/curies`, compiled to [WebAssembly](https://webassembly.org/), for browser integrations with JavaScript, or NodeJS.
- 📈 **R data scientists**: soon available as a R package `curies`

## ⚔️ Cross-platform support

It runs seamlessly on x86 and ARM architectures for many platforms:

- 🐧 Linux
- 🍎 MacOS
- 🪟 Windows
- 🦊 Web browsers

> 💡 **Need Help or Have Suggestions?** We welcome your input and feedback! If you encounter any issues or have ideas to enhance this tool, please [create an issue](https://github.com/biopragmatics/curies.rs/issues) on our GitHub repository.
