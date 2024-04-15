import {describe, expect, test} from '@jest/globals';
import {Record, Converter, getOboConverter, getBioregistryConverter, getMonarchConverter, getGoConverter} from "../pkg/node";

describe('Tests for the curies npm package', () => {
  // NOTE: `await init()` only needed in browser environment

  test('from empty converter', async () => {
    const converter = new Converter();
    const record1 = new Record("DOID", "http://purl.obolibrary.org/obo/DOID_", [], [])
    converter.addRecord(record1);
    converter.addCurie("OBO", "http://purl.obolibrary.org/obo/");
    expect(converter.compress("http://purl.obolibrary.org/obo/DOID_1234")).toBe("DOID:1234");
    expect(converter.expand("OBO:1234")).toBe("http://purl.obolibrary.org/obo/1234");
    expect(converter.expandList(["OBO:1234", "DOID:1234", "Wrong:1"])).toEqual([
      "http://purl.obolibrary.org/obo/1234",
      "http://purl.obolibrary.org/obo/DOID_1234",
      undefined
    ]);
    expect(converter.compress("http://purl.obolibrary.org/obo/1234")).toBe("OBO:1234");
    expect(converter.compressList([
      "http://purl.obolibrary.org/obo/1234",
      "http://purl.obolibrary.org/obo/DOID_1234",
      "http://identifiers.org/DOID:1234"
    ])).toEqual(["OBO:1234", "DOID:1234", undefined]);
    expect(converter.getPrefixes().length).toBe(2)
    expect(converter.getUriPrefixes().length).toBe(2)

    console.log(converter.writeExtendedPrefixMap())
    expect(converter.writeExtendedPrefixMap().startsWith("[{")).toBe(true);
    expect(converter.writeShacl().startsWith("PREFIX")).toBe(true);
    expect(converter.writePrefixMap().length).toBeGreaterThan(10);
    expect(converter.writeJsonld().length).toBeGreaterThan(10);
  });

  test('from prefix map', async () => {
    const converter = await Converter.fromPrefixMap(`{
      "GO": "http://purl.obolibrary.org/obo/GO_",
      "DOID": "http://purl.obolibrary.org/obo/DOID_",
      "OBO": "http://purl.obolibrary.org/obo/"
    }`);
    expect(converter.compress("http://purl.obolibrary.org/obo/DOID_1234")).toBe("DOID:1234");
    expect(converter.expand("DOID:1234")).toBe("http://purl.obolibrary.org/obo/DOID_1234");
    expect(converter.expandList(["OBO:1234", "DOID:1234", "Wrong:1"])).toEqual([
      "http://purl.obolibrary.org/obo/1234",
      "http://purl.obolibrary.org/obo/DOID_1234",
      undefined
    ]);
    expect(converter.compressList([
      "http://purl.obolibrary.org/obo/1234",
      "http://purl.obolibrary.org/obo/DOID_1234",
      "http://identifiers.org/DOID:1234"
    ])).toEqual(["OBO:1234", "DOID:1234", undefined]);
  });

  test('from JSON-LD', async () => {
    const converter = await Converter.fromJsonld(`{
      "@context": {
          "GO": "http://purl.obolibrary.org/obo/GO_",
          "DOID": "http://purl.obolibrary.org/obo/DOID_",
          "OBO": "http://purl.obolibrary.org/obo/"
      }
    }`);
    expect(converter.compress("http://purl.obolibrary.org/obo/DOID_1234")).toBe("DOID:1234");
    expect(converter.expand("DOID:1234")).toBe("http://purl.obolibrary.org/obo/DOID_1234");
  });

  test('from SHACL', async () => {
    const converter = await Converter.fromShacl(`@prefix sh: <http://www.w3.org/ns/shacl#> .
    @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
    [
      sh:declare
        [ sh:prefix "dc" ; sh:namespace "http://purl.org/dc/elements/1.1/"^^xsd:anyURI  ],
        [ sh:prefix "dcterms" ; sh:namespace "http://purl.org/dc/terms/"^^xsd:anyURI  ],
        [ sh:prefix "foaf" ; sh:namespace "http://xmlns.com/foaf/0.1/"^^xsd:anyURI  ],
        [ sh:prefix "xsd" ; sh:namespace "http://www.w3.org/2001/XMLSchema#"^^xsd:anyURI  ]
    ] .`);
    expect(converter.expand("foaf:name")).toBe("http://xmlns.com/foaf/0.1/name");
  });

  test('from extended prefix map', async () => {
    const converter = await Converter.fromExtendedPrefixMap(`[
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
      }
    ]`);
    expect(converter.compress("http://bioregistry.io/DOID:1234")).toBe("DOID:1234");
    expect(converter.expand("doid:1234")).toBe("http://purl.obolibrary.org/obo/DOID_1234");
  });

  test('get OBO converter', async () => {
    const converter = await getOboConverter();
    expect(converter.compress("http://purl.obolibrary.org/obo/DOID_1234")).toBe("DOID:1234");
    expect(converter.expand("DOID:1234")).toBe("http://purl.obolibrary.org/obo/DOID_1234");
  });

  test('get Bioregistry converter', async () => {
    const converter = await getBioregistryConverter();
    expect(converter.compress("http://purl.obolibrary.org/obo/DOID_1234")).toBe("doid:1234");
    expect(converter.expand("doid:1234")).toBe("http://purl.obolibrary.org/obo/DOID_1234");
    expect(converter.standardizePrefix("gomf")).toBe("go");
    expect(converter.standardizeCurie("gomf:0032571")).toBe("go:0032571");
    expect(converter.standardizeUri("http://amigo.geneontology.org/amigo/term/GO:0032571")).toBe("http://purl.obolibrary.org/obo/GO_0032571");
  });

  test('get GO converter', async () => {
    const converter = await getGoConverter();
    expect(converter.compress("http://identifiers.org/ncbigene/100010")).toBe("NCBIGene:100010");
    expect(converter.expand("NCBIGene:100010")).toBe("http://identifiers.org/ncbigene/100010");
  });

  test('get Monarch converter', async () => {
    const converter = await getMonarchConverter();
    expect(converter.compress("http://purl.obolibrary.org/obo/CHEBI_24867")).toBe("CHEBI:24867");
    expect(converter.expand("CHEBI:24867")).toBe("http://purl.obolibrary.org/obo/CHEBI_24867");
  });

  test('chain converters', async () => {
    const customConverter1 = await Converter.fromPrefixMap(`{
      "DOID": "http://purl.obolibrary.org/obo/SPECIAL_DOID_"
    }`);
    const customConverter2 = await Converter.fromPrefixMap(`{
      "GO": "http://purl.obolibrary.org/obo/SPECIAL_GO_",
      "DOID": "http://purl.obolibrary.org/obo/DOID_"
    }`);
    const bioregistryConverter = await getBioregistryConverter();
    const converter = bioregistryConverter
      .chain(customConverter1)
      .chain(customConverter2)
    expect(converter.compress("http://purl.obolibrary.org/obo/SPECIAL_DOID_1234")).toBe("DOID:1234");
  });

});
