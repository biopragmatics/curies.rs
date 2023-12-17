from curies_rs import Record, Converter

rec1 = Record("doid", "http://purl.obolibrary.org/obo/DOID_", [], [])

converter = Converter()
converter.add_record(rec1)

uri = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")

print(uri)

print(rec1.dict())
