# 🧰 Tools for Developers and Semantic Engineers

## 🪄 Working with strings that might be a URI or a CURIE

Sometimes, it’s not clear if a string is a CURIE or a URI. While the [SafeCURIE syntax](https://www.w3.org/TR/2010/NOTE-curie-20101216/#P_safe_curie) is intended to address this, it’s often overlooked.

### ☑️ CURIE and URI Checks

The first way to handle this ambiguity is to be able to check if the string is a CURIE or a URI. Therefore, each `Converter` comes with functions for checking if a string is a CURIE (`converter.is_curie()`) or a URI (`converter.is_uri()`) under its definition.

```python
from curies_rs import get_obo_converter

converter = get_obo_converter()

assert converter.is_curie("GO:1234567")
assert not converter.is_curie("http://purl.obolibrary.org/obo/GO_1234567")
# This is a valid CURIE, but not under this converter's definition
assert not converter.is_curie("pdb:2gc4")

assert converter.is_uri("http://purl.obolibrary.org/obo/GO_1234567")
assert not converter.is_uri("GO:1234567")
# This is a valid URI, but not under this converter's definition
assert not converter.is_uri("http://proteopedia.org/wiki/index.php/2gc4")
```

### 🗜️ Standardized Expansion and Compression

The `converter.expand_or_standardize()` function extends the CURIE expansion function to handle the situation where you might get passed a CURIE or a URI. If it’s a CURIE, expansions happen with the normal rules. If it’s a URI, it tries to standardize it.

```python
from curies_rs import Converter

converter = Converter.from_extended_prefix_map("""[{
  "prefix": "CHEBI",
  "prefix_synonyms": ["chebi"],
  "uri_prefix": "http://purl.obolibrary.org/obo/CHEBI_",
  "uri_prefix_synonyms": ["https://identifiers.org/chebi:"]
}]""")

# Expand CURIEs
assert converter.expand_or_standardize("CHEBI:138488") == 'http://purl.obolibrary.org/obo/CHEBI_138488'
assert converter.expand_or_standardize("chebi:138488") == 'http://purl.obolibrary.org/obo/CHEBI_138488'

# standardize URIs
assert converter.expand_or_standardize("http://purl.obolibrary.org/obo/CHEBI_138488") == 'http://purl.obolibrary.org/obo/CHEBI_138488'
assert converter.expand_or_standardize("https://identifiers.org/chebi:138488") == 'http://purl.obolibrary.org/obo/CHEBI_138488'

# Handle cases that aren't valid w.r.t. the converter
try:
  converter.expand_or_standardize("missing:0000000")
  converter.expand_or_standardize("https://example.com/missing:0000000")
except Exception as e:
  print(e)
```

A similar workflow is implemented in `converter.compress_or_standardize()` for compressing URIs where a CURIE might get passed.

```python
from curies_rs import Converter

converter = Converter.from_extended_prefix_map("""[{
  "prefix": "CHEBI",
  "prefix_synonyms": ["chebi"],
  "uri_prefix": "http://purl.obolibrary.org/obo/CHEBI_",
  "uri_prefix_synonyms": ["https://identifiers.org/chebi:"]
}]""")

# Compress URIs
assert converter.compress_or_standardize("http://purl.obolibrary.org/obo/CHEBI_138488") == 'CHEBI:138488'
assert converter.compress_or_standardize("https://identifiers.org/chebi:138488") == 'CHEBI:138488'

# standardize CURIEs
assert converter.compress_or_standardize("CHEBI:138488") == 'CHEBI:138488'
assert converter.compress_or_standardize("chebi:138488") == 'CHEBI:138488'

# Handle cases that aren't valid w.r.t. the converter
try:
  converter.compress_or_standardize("missing:0000000")
  converter.compress_or_standardize("https://example.com/missing:0000000")
except Exception as e:
  print(e)
  print(type(e))
```

## 🚚 Bulk operations

You can use the `expand_list()` and `compress_list()` functions to processes many URIs or CURIEs at once..

For example to create a new `URI` column in a pandas dataframe from a `CURIE` column:

```python
import pandas as pd
from curies_rs import get_bioregistry_converter

converter = get_bioregistry_converter()
df = pd.DataFrame({'CURIE': ['doid:1234', 'doid:5678', 'doid:91011']})

# Expand the list of CURIEs to URIs
df['URI'] = converter.expand_list(df['CURIE'])
print(df)
```

## 🧩 Integrating with [`rdflib`](https://rdflib.readthedocs.io/en/stable/apidocs/rdflib.html#module-rdflib)

RDFlib is a pure Python package for manipulating RDF data. The following example shows how to bind the extended prefix map from a `Converter` to a graph ([`rdflib.Graph`](https://rdflib.readthedocs.io/en/stable/apidocs/rdflib.html#rdflib.Graph)).

```python
import curies_rs, rdflib, rdflib.namespace, json

converter = curies_rs.get_obo_converter()
g = rdflib.Graph()

for prefix, uri_prefix in json.loads(converter.write_prefix_map()).items():
    g.bind(prefix, rdflib.Namespace(uri_prefix))
```

A more flexible approach is to instantiate a namespace manager ([`rdflib.namespace.NamespaceManager`](https://rdflib.readthedocs.io/en/stable/apidocs/rdflib.namespace.html#rdflib.namespace.NamespaceManager)) and bind directly to that.

```python
import curies_rs, rdflib, json

converter = curies_rs.get_obo_converter()
namespace_manager = rdflib.namespace.NamespaceManager(rdflib.Graph())

for prefix, uri_prefix in json.loads(converter.write_prefix_map()).items():
    namespace_manager.bind(prefix, rdflib.Namespace(uri_prefix))
```

URI references for use in RDFLib’s graph class can be constructed from CURIEs using a combination of `converter.expand()` and [`rdflib.URIRef`](https://rdflib.readthedocs.io/en/stable/apidocs/rdflib.html#rdflib.URIRef).

```python
import curies_rs, rdflib

converter = curies_rs.get_obo_converter()

uri_ref = rdflib.URIRef(converter.expand("CHEBI:138488"))
```

<!-- TODO: Reusable data structures for references? -->
