#!/usr/bin/env bash
set -e

cd js

npm run test

python3 -m http.server
# Or npm run start
