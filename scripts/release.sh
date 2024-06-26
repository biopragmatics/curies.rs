#!/usr/bin/env bash
set -e

# Script to bump version in Cargo.toml, update CHANGELOG.md and create a new tag

# Check if version argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <new_version>"
    exit 1
fi
new_version=$1

echo ""
echo "  🏔️ Update version in Cargo.toml"
echo ""


sed -i "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"\$/version = \"$new_version\"/" "Cargo.toml"
sed -i "s/curies = { version = \"[0-9]*\.[0-9]*\.[0-9]*\"/curies = { version = \"$new_version\"/" "Cargo.toml"
echo "🔼  Updated version in Cargo.toml"

git cliff -o CHANGELOG.md --tag $new_version
git add Cargo.toml */Cargo.toml CHANGELOG.md
git commit -m "chore: Bump version to $new_version"
git push

echo ""
echo "  🏷️  Create and push tag"
echo ""
git tag -a v$new_version -m "v$new_version"
git push origin v$new_version

echo ""
echo "  🎉 Version $new_version released"
