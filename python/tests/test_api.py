from curies_rs import (
    Converter,
    Record,
    get_bioregistry_converter,
    get_go_converter,
    get_monarch_converter,
    get_obo_converter,
)


def test_converter():
    """Test the converter: create, add record, compress, expand, chain converters."""
    rec1 = Record("doid", "http://purl.obolibrary.org/obo/DOID_")

    converter = Converter()
    converter.add_record(rec1)
    assert converter.compress("http://purl.obolibrary.org/obo/DOID_1234") == "doid:1234"
    assert converter.expand("doid:1234") == "http://purl.obolibrary.org/obo/DOID_1234"
    assert converter.expand_list(["doid:1234"]) == ["http://purl.obolibrary.org/obo/DOID_1234"]
    assert converter.compress_list(["http://purl.obolibrary.org/obo/DOID_1234"]) == ["doid:1234"]

    # Test chain
    converter2 = Converter()
    converter2.add_prefix("obo", "http://purl.obolibrary.org/obo/")

    merged = converter.chain(converter2)
    assert merged.expand("doid:1234") == "http://purl.obolibrary.org/obo/DOID_1234"
    assert merged.expand("obo:1234") == "http://purl.obolibrary.org/obo/1234"
    assert len(merged.get_prefixes()) == 2
    assert len(merged.get_uri_prefixes()) == 2

    assert merged.write_extended_prefix_map().startswith("[{")
    assert merged.write_shacl().startswith("PREFIX")
    assert len(merged.write_prefix_map()) > 10 # TODO: these checks could be improved
    assert len(merged.write_jsonld()) > 10
    # print(merged.write_extended_prefix_map())
    # print(merged.write_prefix_map())
    # print(merged.write_jsonld())


def test_from_prefix_map():
    """Test creating the converter from prefix map."""
    prefix_map = """{
        "GO": "http://purl.obolibrary.org/obo/GO_",
        "DOID": "http://purl.obolibrary.org/obo/DOID_",
        "OBO": "http://purl.obolibrary.org/obo/"
    }"""
    conv = Converter.from_prefix_map(prefix_map)
    assert conv.expand("DOID:1234") == "http://purl.obolibrary.org/obo/DOID_1234"
    assert conv.compress("http://purl.obolibrary.org/obo/DOID_1234") == "DOID:1234"


def test_from_extended_prefix_map():
    """Test creating the converter from extended prefix map."""
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
    conv = Converter.from_extended_prefix_map(extended_pm)
    assert conv.expand("doid:1234") == "http://purl.obolibrary.org/obo/DOID_1234"
    assert conv.compress("http://purl.obolibrary.org/obo/DOID_1234") == "DOID:1234"


# def test_from_extended_prefix_map_dict():
#     """Test creating the converter from extended prefix map."""
#     extended_pm = [
#         {
#         "prefix": "DOID",
#         "prefix_synonyms": [
#             "doid"
#         ],
#         "uri_prefix": "http://purl.obolibrary.org/obo/DOID_",
#         "uri_prefix_synonyms": [
#             "http://bioregistry.io/DOID:"
#         ],
#         "pattern": "^\\\\d+$"
#     },
#     {
#         "prefix": "GO",
#         "prefix_synonyms": [
#             "go"
#         ],
#         "uri_prefix": "http://purl.obolibrary.org/obo/GO_",
#         "pattern": "^\\\\d{7}$"
#     },
#     {
#         "prefix": "OBO",
#         "prefix_synonyms": [
#             "obo"
#         ],
#         "uri_prefix": "http://purl.obolibrary.org/obo/"
#     }]
#     conv = Converter.from_extended_prefix_map(extended_pm)
#     assert conv.expand("doid:1234"), "http://purl.obolibrary.org/obo/DOID_1234"
#     assert conv.compress("http://purl.obolibrary.org/obo/DOID_1234"), "DOID:1234"


def test_from_jsonld():
    """Test creating the converter from JSON-LD context."""
    jsonld = """{
        "@context": {
            "GO": "http://purl.obolibrary.org/obo/GO_",
            "DOID": "http://purl.obolibrary.org/obo/DOID_",
            "OBO": "http://purl.obolibrary.org/obo/"
        }
    }"""
    conv = Converter.from_jsonld(jsonld)
    assert conv.expand("DOID:1234") == "http://purl.obolibrary.org/obo/DOID_1234"
    assert conv.compress("http://purl.obolibrary.org/obo/DOID_1234") == "DOID:1234"

def test_from_shacl():
    """Test creating the converter from SHACL prefix definition."""
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
    assert conv.expand("foaf:name") == "http://xmlns.com/foaf/0.1/name"


def test_predefined_converters():
    """Test the predefined converters."""
    obo = get_obo_converter()
    assert obo.expand("DOID:1234") == "http://purl.obolibrary.org/obo/DOID_1234"
    assert obo.compress("http://purl.obolibrary.org/obo/DOID_1234") == "DOID:1234"

    go = get_go_converter()
    assert go.expand("NCBIGene:100010") == "http://identifiers.org/ncbigene/100010"
    assert go.compress("http://identifiers.org/ncbigene/100010") == "NCBIGene:100010"

    monarch = get_monarch_converter()
    assert monarch.expand("CHEBI:24867") == "http://purl.obolibrary.org/obo/CHEBI_24867"
    assert monarch.compress("http://purl.obolibrary.org/obo/CHEBI_24867") == "CHEBI:24867"

    bioregistry = get_bioregistry_converter()
    assert bioregistry.expand("doid:1234") == "http://purl.obolibrary.org/obo/DOID_1234"
    assert bioregistry.compress("http://purl.obolibrary.org/obo/DOID_1234") == "doid:1234"

    assert bioregistry.standardize_prefix("gomf") == "go"
    assert bioregistry.standardize_curie("gomf:0032571") == "go:0032571"
    assert bioregistry.standardize_uri("http://amigo.geneontology.org/amigo/term/GO:0032571") == "http://purl.obolibrary.org/obo/GO_0032571"


def test_chain():
    converter = (
        get_obo_converter()
            .chain(get_go_converter())
            .chain(get_monarch_converter())
    )
    assert converter.expand("CHEBI:24867") == "http://purl.obolibrary.org/obo/CHEBI_24867"
    # print(len(converter))
