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
