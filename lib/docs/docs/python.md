# 🐍 Use from Python

[![PyPI](https://img.shields.io/pypi/v/curies-rs)](https://pypi.org/project/curies-rs/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/curies-rs.svg?logo=python&label=Python&logoColor=silver)](https://pypi.org/project/curies-rs)

You can easily work with `curies` from Python.

## 📥️ Installation

Install the `pip` package:

```bash
pip install curies-rs
```

## 🚀 Usage

Initialize a converter, then use it to `compress` URIs to CURIEs, or `expand` CURIEs to URIs:

```python title="curies_conversion.py"
from curies_rs import get_bioregistry_converter

# Initialize converter (here we use the predefined Bioregistry converter)
converter = get_bioregistry_converter()

# Compress a URI, or expand a CURIE
curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")
uri = converter.expand("DOID:1234")

# Compress/expand a list
curies = converter.compress_list(["http://purl.obolibrary.org/obo/DOID_1234"])
uris = converter.expand_list(["DOID:1234", "doid:1235"])

# Standardize prefix, CURIEs, and URIs using the preferred alternative
assert converter.standardize_prefix("gomf") == "go"
assert converter.standardize_curie("gomf:0032571") == "go:0032571"
assert converter.standardize_uri("http://amigo.geneontology.org/amigo/term/GO:0032571") == "http://purl.obolibrary.org/obo/GO_0032571"
```

## 🌀 Loading a Context

There are several ways to load a context with this package, including:

1. pre-defined contexts
2. contexts encoded in the standard prefix map format
3. contexts encoded in the standard JSON-LD context format
4. contexts encoded in the extended prefix map format

### 📦 Loading a predefined context

Easiest way to get started is to simply use one of the function available to import a converter from popular namespaces registries:

**[Bioregistry](https://bioregistry.io/) converter**

```python
from curies_rs import get_bioregistry_converter

converter = get_bioregistry_converter()
```

**[OBO](http://obofoundry.org/) converter**

```python
from curies_rs import get_obo_converter

converter = get_obo_converter()
```

**[GO](https://geneontology.org/) converter**

```python
from curies_rs import get_go_converter

converter = get_go_converter()
```

**[Monarch Initiative](https://monarchinitiative.org/) converter**

```python
from curies_rs import get_monarch_converter

converter = get_monarch_converter()
```

### 🗺️ Loading Extended Prefix Maps

Enable to provide prefix/URI synonyms and ID RegEx pattern for each record:

```python
from curies_rs import Converter

extended_pm = """[
    {
    "prefix": "DOID",
    "prefix_synonyms": [
        "doid"
    ],
    "uri_prefix": "http://purl.obolibrary.org/obo/DOID_",
    "uri_prefix_synonyms": [
        "http://bioregistry.io/DOID:"
    ],
    "pattern": "^\\\\d+$"
},
{
    "prefix": "GO",
    "prefix_synonyms": [
        "go"
    ],
    "uri_prefix": "http://purl.obolibrary.org/obo/GO_",
    "pattern": "^\\\\d{7}$"
},
{
    "prefix": "OBO",
    "prefix_synonyms": [
        "obo"
    ],
    "uri_prefix": "http://purl.obolibrary.org/obo/"
}]"""
converter = Converter.from_extended_prefix_map(extended_pm)
```

!!! tip "Support URL"

    For all `Converter.from_` functions you can either provide the file content, or the URL to the file as string.

### 📍 Loading Prefix Maps

A simple dictionary without synonyms information:

```python
from curies_rs import Converter

prefix_map = """{
    "GO": "http://purl.obolibrary.org/obo/GO_",
    "DOID": "http://purl.obolibrary.org/obo/DOID_",
    "OBO": "http://purl.obolibrary.org/obo/"
}"""
converter = Converter.from_prefix_map(prefix_map)
```

### 📄 Loading JSON-LD contexts

```python
from curies_rs import Converter

jsonld = """{
    "@context": {
        "GO": "http://purl.obolibrary.org/obo/GO_",
        "DOID": "http://purl.obolibrary.org/obo/DOID_",
        "OBO": "http://purl.obolibrary.org/obo/"
    }
}"""
converter = Converter.from_jsonld(jsonld)
```

Or directly use a URL:

```python
from curies_rs import Converter

converter = Converter.from_jsonld("https://purl.obolibrary.org/meta/obo_context.jsonld")
```

### 🔗 Loading SHACL prefixes definitions

```python
from curies_rs import Converter

shacl = """@prefix sh: <http://www.w3.org/ns/shacl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
[
  sh:declare
    [ sh:prefix "dc" ; sh:namespace "http://purl.org/dc/elements/1.1/"^^xsd:anyURI  ],
    [ sh:prefix "dcterms" ; sh:namespace "http://purl.org/dc/terms/"^^xsd:anyURI  ],
    [ sh:prefix "foaf" ; sh:namespace "http://xmlns.com/foaf/0.1/"^^xsd:anyURI  ],
    [ sh:prefix "xsd" ; sh:namespace "http://www.w3.org/2001/XMLSchema#"^^xsd:anyURI  ]
] ."""
conv = Converter.from_shacl(shacl)
```

## 🔎 Introspecting on a Context

After loading a context, it’s possible to get certain information out of the converter. For example, if you want to get all of the CURIE prefixes from the converter, you can use `converter.get_prefixes()`:

```python
from curies_rs import get_bioregistry_converter

converter = get_bioregistry_converter()

prefixes = converter.get_prefixes()
assert 'chebi' in prefixes
assert 'CHEBIID' not in prefixes, "No synonyms are included by default"

prefixes = converter.get_prefixes(include_synonyms=True)
assert 'chebi' in prefixes
assert 'CHEBIID' in prefixes
```

Similarly, the URI prefixes can be extracted with `Converter.get_uri_prefixes()` like in:

```python
from curies_rs import get_bioregistry_converter

converter = get_bioregistry_converter()

uri_prefixes = converter.get_uri_prefixes()
assert 'http://purl.obolibrary.org/obo/CHEBI_' in uri_prefixes
assert 'https://bioregistry.io/chebi:' not in uri_prefixes, "No synonyms are included by default"

uri_prefixes = converter.get_uri_prefixes(include_synonyms=True)
assert 'http://purl.obolibrary.org/obo/CHEBI_' in uri_prefixes
assert 'https://bioregistry.io/chebi:' in uri_prefixes
```

It’s also possible to get a bijective prefix map, i.e., a dictionary from primary CURIE prefixes to primary URI prefixes. This is useful for compatibility with legacy systems which assume simple prefix maps. This can be done with the `bimap` property like in the following:

```python
import json
from curies_rs import get_bioregistry_converter

converter = get_bioregistry_converter()

prefix_map = json.loads(converter.write_prefix_map())
assert prefix_map['chebi'] == 'http://purl.obolibrary.org/obo/CHEBI_'
```

## 🛠️ Modifying a Context

### 🔨 Incremental Converters

New data can be added to an existing converter with either `converter.add_prefix()` or `converter.add_record()`. For example, a CURIE and URI prefix for HGNC can be added to the OBO Foundry converter with the following:

```python
from curies_rs import get_obo_converter

converter = get_obo_converter()
converter.add_prefix("hgnc", "https://bioregistry.io/hgnc:")
```

Similarly, an empty converter can be instantiated using an empty list for the records argument and prefixes can be added one at a time (note this currently does not allow for adding synonyms separately):

```python
from curies_rs import Converter, Record

rec1 = Record("doid", "http://purl.obolibrary.org/obo/DOID_", ["DOID"], ["https://identifiers.org/doid/"])
# print(rec1.dict())

converter = Converter()
converter.add_record(rec1)
```

A more flexible version of this operation first involves constructing a `Record` object:

```python
from curies_rs import get_obo_converter, Record

converter = get_obo_converter()
record = Record(prefix="hgnc", uri_prefix="https://bioregistry.io/hgnc:")
converter.add_record(record)
```

By default, both of these operations will fail if the new content conflicts with existing content. If desired, the `merge` argument can be set to true to enable merging. Further, checking for conflicts and merging can be made to be case insensitive by setting `case_sensitive` to false.

Such a merging strategy is the basis for wholesale merging of converters, described below.

### ⛓️ Chaining and merging

Chain together multiple converters, prioritizes based on the order given. Therefore, if two prefix maps having the same prefix but different URI prefixes are given, the first is retained. The second is retained as a synonym

```python
from curies_rs import get_obo_converter, get_go_converter, get_monarch_converter

converter = (
    get_obo_converter()
    	.chain(get_go_converter())
    	.chain(get_monarch_converter())
)
```

<!-- TODO: Subsetting? -->

## ✒️ Writing a Context

Write the converter prefix map as a string in different serialization format:

```python
from curies_rs import get_bioregistry_converter

converter = get_bioregistry_converter()

epm = converter.write_extended_prefix_map()
pm = converter.write_prefix_map()
jsonld = converter.write_jsonld()
shacl = converter.write_shacl()
```
