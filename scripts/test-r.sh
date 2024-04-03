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
    Rscript -e 'install.packages("usethis"); install.packages("devtools"); install.packages("testthat"); install.packages("rextendr"); rextendr::document("./r"); library(testthat); test_dir("r/tests");'
    # NOTE: the packages installed in separate Rscript commands are not available in the next command, so we need to install them all in one command
    # Rscript -e 'install.packages("usethis")'
    # Rscript -e 'install.packages("devtools")'
    # Rscript -e 'install.packages("testthat")'
    # Rscript -e 'install.packages("rextendr")'
    # Rscript -e 'remotes::install_github("extendr/rextendr")'
else
    Rscript -e 'rextendr::document("./r"); library(testthat); test_dir("r/tests");'
fi
