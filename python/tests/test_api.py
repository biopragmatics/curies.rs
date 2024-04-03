import unittest

from curies_rs import Record, Converter, get_obo_converter, get_bioregistry_converter, get_monarch_converter, get_go_converter


class TestAPI(unittest.TestCase):
    """Test the API."""

    def test_converter(self):
        """Test the converter: create, add record, compress, expand, chain converters."""
        rec1 = Record("doid", "http://purl.obolibrary.org/obo/DOID_")

        converter = Converter()
        converter.add_record(rec1)

        self.assertEqual(converter.compress("http://purl.obolibrary.org/obo/DOID_1234"), "doid:1234")
        self.assertEqual(converter.expand("doid:1234"), "http://purl.obolibrary.org/obo/DOID_1234")

        self.assertEqual(converter.expand_list(["doid:1234"]), ["http://purl.obolibrary.org/obo/DOID_1234"])
        self.assertEqual(converter.compress_list(["http://purl.obolibrary.org/obo/DOID_1234"]), ["doid:1234"])

        # Test chain
        rec2 = Record("obo", "http://purl.obolibrary.org/obo/", [], [])
        converter2 = Converter()
        converter2.add_record(rec2)

        merged = converter.chain(converter2)
        self.assertEqual(merged.expand("doid:1234"), "http://purl.obolibrary.org/obo/DOID_1234")
        self.assertEqual(merged.expand("obo:1234"), "http://purl.obolibrary.org/obo/1234")


    def test_from_prefix_map(self):
        """Test creating the converter from prefix map."""
        prefix_map = """{
            "GO": "http://purl.obolibrary.org/obo/GO_",
            "DOID": "http://purl.obolibrary.org/obo/DOID_",
            "OBO": "http://purl.obolibrary.org/obo/"
        }"""
        conv = Converter.from_prefix_map(prefix_map)
        self.assertEqual(conv.expand("DOID:1234"), "http://purl.obolibrary.org/obo/DOID_1234")
        self.assertEqual(conv.compress("http://purl.obolibrary.org/obo/DOID_1234"), "DOID:1234")


    def test_from_extended_prefix_map(self):
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
        self.assertEqual(conv.expand("doid:1234"), "http://purl.obolibrary.org/obo/DOID_1234")
        self.assertEqual(conv.compress("http://purl.obolibrary.org/obo/DOID_1234"), "DOID:1234")


    def test_from_jsonld(self):
        """Test creating the converter from JSON-LD context."""
        jsonld = """{
            "@context": {
                "GO": "http://purl.obolibrary.org/obo/GO_",
                "DOID": "http://purl.obolibrary.org/obo/DOID_",
                "OBO": "http://purl.obolibrary.org/obo/"
            }
        }"""
        conv = Converter.from_jsonld(jsonld)
        self.assertEqual(conv.expand("DOID:1234"), "http://purl.obolibrary.org/obo/DOID_1234")
        self.assertEqual(conv.compress("http://purl.obolibrary.org/obo/DOID_1234"), "DOID:1234")


    def test_predefined_converters(self):
        """Test the predefined converters."""
        obo = get_obo_converter()
        self.assertEqual(obo.expand("DOID:1234"), "http://purl.obolibrary.org/obo/DOID_1234")
        self.assertEqual(obo.compress("http://purl.obolibrary.org/obo/DOID_1234"), "DOID:1234")

        bioregistry = get_bioregistry_converter()
        self.assertEqual(bioregistry.expand("doid:1234"), "http://purl.obolibrary.org/obo/DOID_1234")
        self.assertEqual(bioregistry.compress("http://purl.obolibrary.org/obo/DOID_1234"), "doid:1234")

        go = get_go_converter()
        self.assertEqual(go.expand("NCBIGene:100010"), "http://identifiers.org/ncbigene/100010")
        self.assertEqual(go.compress("http://identifiers.org/ncbigene/100010"), "NCBIGene:100010")

        monarch = get_monarch_converter()
        self.assertEqual(monarch.expand("CHEBI:24867"), "http://purl.obolibrary.org/obo/CHEBI_24867")
        self.assertEqual(monarch.compress("http://purl.obolibrary.org/obo/CHEBI_24867"), "CHEBI:24867")

    def test_chain(self):
        converter = (
            get_obo_converter()
                .chain(get_go_converter())
                .chain(get_monarch_converter())
        )
        self.assertEqual(converter.expand("CHEBI:24867"), "http://purl.obolibrary.org/obo/CHEBI_24867")
        print(len(converter))
