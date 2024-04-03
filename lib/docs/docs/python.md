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
uris = converter.expand_list(["DOID:1234"])
```

## 🌀 Converter initialization

There are many ways to initialize a CURIE/URI converter.

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

###  📂 Load converter from prefix map

Converter can be loaded from a prefix map, an extended prefix map (which enables to provide more information for each prefix), or a JSON-LD context.

For each function you can either provide the string to the prefix map JSON, or the URL to it.

#### Load from prefix map

```python
from curies_rs import Converter

prefix_map = """{
    "GO": "http://purl.obolibrary.org/obo/GO_",
    "DOID": "http://purl.obolibrary.org/obo/DOID_",
    "OBO": "http://purl.obolibrary.org/obo/"
}"""
converter = Converter.from_prefix_map(prefix_map)
```

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

### 🛠️ Build the converter programmatically

Create an empty `Converter`, and populate it with `Record`:

```python
from curies_rs import Converter, Record

rec1 = Record("doid", "http://purl.obolibrary.org/obo/DOID_", ["DOID"], ["https://identifiers.org/doid/"])
rec2 = Record("obo", "http://purl.obolibrary.org/obo/")
print(rec1.dict())

converter = Converter()
converter.add_record(rec1)
converter.add_record(rec2)
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
