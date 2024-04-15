# 🐍 Use from Python

[![PyPI](https://img.shields.io/pypi/v/curies-rs)](https://pypi.org/project/curies-rs/)

You can easily work with `curies` from Python.

## 📥️ Install

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

# Get the list of prefixes or URI prefixes, argument include_synonyms default to False
prefixes_without_syn = converter.get_prefixes()
uri_prefixes_with_syn = converter.get_uri_prefixes(True)
```

## 🌀 Load a converter

There are many ways to load a CURIE/URI converter.

### 📦 Import a predefined converter

Easiest way to get started is to simply use one of the function available to import a converter from popular namespaces registries:

#### [Bioregistry](https://bioregistry.io/) converter

```python
from curies_rs import get_bioregistry_converter

converter = get_bioregistry_converter()
```

#### [OBO](http://obofoundry.org/) converter

```python
from curies_rs import get_obo_converter

converter = get_obo_converter()
```

#### [GO](https://geneontology.org/) converter

```python
from curies_rs import get_go_converter

converter = get_go_converter()
```

#### [Monarch Initiative](https://monarchinitiative.org/) converter

```python
from curies_rs import get_monarch_converter

converter = get_monarch_converter()
```

###  📂 Load from file

Converter can be loaded from a prefix map, an extended prefix map (which enables to provide more information for each prefix), or a JSON-LD context.

!!! tip "Support URL"

    For each `Converter.from_` function you can either provide the file content, or the URL to the file as string.


#### Load from extended prefix map

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

#### Load from prefix map

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

#### Load from JSON-LD context

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

#### Load from SHACL prefixes definition

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

### 🛠️ Build the converter programmatically

Create an empty `Converter`, and populate it with `Record`:

```python
from curies_rs import Converter, Record

rec1 = Record("doid", "http://purl.obolibrary.org/obo/DOID_", ["DOID"], ["https://identifiers.org/doid/"])
print(rec1.dict())

converter = Converter()
converter.add_record(rec1)
converter.add_prefix("obo", "http://purl.obolibrary.org/obo/")
```

### ⛓️ Chain converters

Chain together multiple converters:

```python
from curies_rs import get_obo_converter, get_go_converter, get_monarch_converter

converter = (
    get_obo_converter()
    	.chain(get_go_converter())
    	.chain(get_monarch_converter())
)
print(len(converter))
```

## ✒️ Serialize a converter

Output the converter prefix map as a string in different serialization format:

```python
from curies_rs import get_bioregistry_converter

converter = get_bioregistry_converter()

epm = converter.write_extended_prefix_map()
pm = converter.write_prefix_map()
jsonld = converter.write_jsonld()
shacl = converter.write_shacl()
```
