import time
from curies_rs import Converter

url = "https://raw.githubusercontent.com/biopragmatics/bioregistry/main/exports/contexts/bioregistry.epm.json"
converter = Converter.load_extended_prefix_map(url)

start_time = time.time()
curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")
print(f"Compress took {(time.time() - start_time) * 1000}ms")

print(curie)
