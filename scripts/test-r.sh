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

# TODO: we still need to figure out how to properly install R dependencies in a way that is stable and does not depend on user config
# By default R installs packages in /usr/local/lib/R/site-library, which requires sudo...

# A proposed solution is to override an env variable like R_LIBS_USER=~/R_libs, but that does not look stable
# And it is not serious to ask all potential users to set this variable in their environment
# Crazy idea: R should automatically install the packages in a local hidden folder for this project
# Otherwise we can also just chown or chmod the whole R/site-library directory, but that's not safe

# The commands below have been working on Ubuntu and MacOS the past months, but on new Ubuntu setup it fails even when permissions are fixed (nice reproducibility)
# > ERROR: dependencies ‘usethis’, ‘pkgdown’, ‘rcmdcheck’, ‘rversions’, ‘urlchecker’ are not available for package ‘devtools’

# R installed with: sudo apt install r-base r-base-dev


if [ "$INSTALL_DEPS" = true ]; then
    echo "☕️ Get yourself a coffee, installing R packages is the slowest process I have ever seen."
    echo "Apparently R does not install packages, it rebuilds all of them from scratch calling CMake and other stuff."
    echo "If facing issues related to the installation directory permissions use:"
    echo 'sudo chown -R $(id -u):$(id -g) /usr/local/lib/R/site-library/'
    Rscript -e 'install.packages(c("usethis", "devtools", "testthat", "rextendr"), repos="http://cran.r-project.org")'
    # Rscript -e 'required_packages <- c("usethis", "devtools", "testthat", "rextendr"); install.packages(required_packages, repos="http://cran.r-project.org")'
    # Rscript --save -e 'required_packages <- c("usethis", "devtools", "testthat", "rextendr"); install.packages(required_packages, repos="http://cran.r-project.org", dependencies=TRUE)'
fi



Rscript -e 'rextendr::document("./r"); library(testthat); test_dir("r/tests");'
