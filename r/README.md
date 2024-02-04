# CURIES R package

https://cran.r-project.org/web/packages/rextendr/vignettes/package.html

Library: https://github.com/extendr/extendr

Complete example: https://github.com/extendr/helloextendr

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

Compile:

```r
rextendr::document("./r")
```

Run tests:

```r
CMD check .
```

Load R package:

```r
devtools::load_all("./r")
```

## helloextendr example

Start R shell:

```bash
R
```

Compile and install:

``` r
rextendr::document("r")
```

After installation, the following should work:
```r
library(helloextendr)

hello_world()
#> [1] "Hello world!"
```

The R code for our converter should look like this:

```r
library(helloextendr)
converter <- ConverterR$new()
curie <- converter$compress("http://purl.obolibrary.org/obo/DOID_1234")
print(curie)
```
