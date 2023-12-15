#!/usr/bin/env bash
set -e

cd js

npm run build

python3 -m http.server
# Or npm run start
