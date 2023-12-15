#!/usr/bin/env bash
set -e
# Script to install dependencies for development and enable pre-commit hooks

rustup update

cargo install mdbook mdbook-admonish mdbook-pagetoc

mkdir -p theme
wget -O theme/mdbook-admonish.css https://raw.githubusercontent.com/leptos-rs/leptos/a8e25af5233bb014d3cee85e4e9be8b3e4586de9/docs/book/mdbook-admonish.css
