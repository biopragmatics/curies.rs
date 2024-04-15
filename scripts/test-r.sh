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
    Rscript -e 'required_packages <- c("usethis", "devtools", "testthat", "rextendr"); install.packages(required_packages, repos="http://cran.r-project.org")'
    # Rscript --save -e 'required_packages <- c("usethis", "devtools", "testthat", "rextendr"); install.packages(required_packages, repos="http://cran.r-project.org", dependencies=TRUE)'
fi

Rscript -e 'rextendr::document("./r"); library(testthat); test_dir("r/tests");'
