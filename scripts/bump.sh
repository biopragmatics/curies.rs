#!/usr/bin/env bash
set -e

# Check if version argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <new_version>"
    exit 1
fi

new_version=$1

sed -i "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"\$/version = \"$new_version\"/" "Cargo.toml"
sed -i "s/curies = { version = \"[0-9]*\.[0-9]*\.[0-9]*\"/curies = { version = \"$new_version\"/" "Cargo.toml"
echo "🏷️  Updated version in Cargo.toml"

gmsg "🏷️ Bump to $new_version" || true
