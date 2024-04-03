#!/usr/bin/env bash
set -e

# Check for --no-build flag
SKIP_BUILD=false
for arg in "$@"; do
    if [[ $arg == "--no-build" ]]; then
        SKIP_BUILD=true
        break
    fi
done

cd js
npm install

if [ "$SKIP_BUILD" = false ]; then
    npm run test -- --silent=false
else
    npm run jest -- --silent=false
fi

# python -m http.server
# Or npm run start
