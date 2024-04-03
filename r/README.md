# CURIES R package

`rextendr`docs (to scaffold project with `extendr` bindings): https://cran.r-project.org/web/packages/rextendr/vignettes/package.html

`extendr` library: https://github.com/extendr/extendr

`extendr` API docs: https://extendr.github.io/extendr/extendr_api

Complete example: https://github.com/extendr/helloextendr

## Install dependencies

Start R shell:

```bash
R
```

Install dev packages:

```r
install.packages("usethis")
install.packages("rextendr")
install.packages("devtools")
install.packages("testthat")
```

Or install from GitHub:

``` r
remotes::install_github("extendr/rextendr")
```

## Build

Compile:

```r
rextendr::document("./r")
```

Run tests:

```r
library(testthat); test_dir("r/tests");
```

Load R package:

```r
devtools::load_all("./r")
```

## curiesr example

Start R shell:

```bash
R
```

Compile and install:

```r
rextendr::document("r")
```

After installation, the following should work:

```r
library(curies)

converter <- Converter$new()
curie <- converter$compress("http://purl.obolibrary.org/obo/DOID_1234")
print(curie)
```
