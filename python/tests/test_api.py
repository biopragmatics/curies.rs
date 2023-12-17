import unittest

from curies_rs import Record, Converter


class TestAPI(unittest.TestCase):
    """Test the API."""

    def test_converter(self):
        """Test the converter."""
        rec1 = Record("doid", "http://purl.obolibrary.org/obo/DOID_", [], [])

        converter = Converter()
        converter.add_record(rec1)

        uri = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")
        self.assertEqual("doid:1234", uri)
