import curies

# converter = curies.get_bioregistry_converter()
url = "https://w3id.org/biopragmatics/bioregistry.epm.json"
converter = curies.load_extended_prefix_map(url)

curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")

print(curie)
