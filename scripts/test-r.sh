#!/usr/bin/env bash
set -e

# Build and run tests for R bindings

# Check for --install flag
INSTALL_DEPS=false
for arg in "$@"; do
    if [[ $arg == "--install" ]]; then
        INSTALL_DEPS=true
        break
    fi
done

if [ "$INSTALL_DEPS" = true ]; then
    Rscript -e 'install.packages("usethis")'
    Rscript -e 'install.packages("devtools")'
    Rscript -e 'install.packages("testthat")'

    # Rscript -e 'install.packages("rextendr")'
    Rscript -e 'remotes::install_github("extendr/rextendr")'
fi

Rscript -e 'rextendr::document("./r"); library(testthat); test_dir("r/tests");'
