# 📈 Use from R

[![PyPI](https://img.shields.io/pypi/v/curies-rs)](https://pypi.org/project/curies-rs/)

!!! warning "Work in progress"

    R bindings are not yet fully published. Checkout the [Development](contributing.md) section to try it by building from source!

You can easily work with `curies` from R.

## 📥️ Install

Install the R package:

```bash
Rscript -e 'rextendr::document("./r")'
```

## 🚀 Usage

Initialize a converter, then use it to `compress` URIs to CURIEs, or `expand` CURIEs to URIs:

```r title="curies_conversion.R"
library(curiesr)

converter <- Converter$new()

curie <- converter$compress("http://purl.obolibrary.org/obo/DOID_1234")
uri <- converter$expand("doid:1234")

print(curie)
print(uri)
```
