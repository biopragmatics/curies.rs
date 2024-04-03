# 🟨 Use from JavaScript

[![npm](https://img.shields.io/npm/v/@biopragmatics/curies)](https://www.npmjs.com/package/@biopragmatics/curies)

You can easily work with CURIEs in the browser or NodeJS, from JavaScript or TypeScript, with the [`@biopragmatics/curies`](https://www.npmjs.com/package/@biopragmatics/curies) NPM package.

## 📥️ Install

Install the `npm` package (use `yarn` or `pnpm` if you prefer) to use it from your favorite framework:

```bash
npm install @biopragmatics/curies
# or
pnpm add @biopragmatics/curies
# or
yarn add @biopragmatics/curies
# or
bun add @biopragmatics/curies
```

## 🟢 Use in a NodeJS environment

There are multiple methods available for creating or importing converters:

```javascript
import {Record, Converter, getOboConverter, getBioregistryConverter} from "@biopragmatics/curies";

async function main() {
  // Populate from Records
  const rec1 = new Record("obo", "http://purl.obolibrary.org/obo/", [], []);
  console.log(rec1.toString());
  console.log(rec1.toJs());
  const converter = new Converter();
  converter.addRecord(rec1);

  // Load from a prefix map json (string or URI)
  const converterFromMap = await Converter.fromPrefixMap(`{
    "doid": "http://purl.obolibrary.org/obo/MY_DOID_"
  }`);

  // Load from an extended prefix map (string or URI)
  const converterFromUrl = await Converter.fromExtendedPrefixMap("https://w3id.org/biopragmatics/bioregistry.epm.json")

  // Load from a JSON-LD context (string or URI)
  const converterFromJsonld = await Converter.fromJsond("https://purl.obolibrary.org/meta/obo_context.jsonld");

  // Load from one of the predefined source
  const converterFromSource = await getBioregistryConverter();

  // Chain multiple converters in one
  const converter = converterFromMap
    .chain(converterFromUrl)
    .chain(converterFromSource)

  // Expand CURIE and compress URI
  const curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234");
  const uri = converter.expand("doid:1234");

  // Expand and compress list of CURIEs and URIs
  const curies = converter.compressList(["http://purl.obolibrary.org/obo/DOID_1234"]);
  const uris = converter.expandList(["doid:1234"]);
}
main();
```

## 🦊 Use it in a browser

When using in a client browser you will need to initialize the wasm binary with `await init()`, after that you can use the same functions as in the NodeJS environments.

!!! bug "Use HTTPS"

    When importing converters from URLs in JS always prefer importing from HTTPS URLs, otherwise you will face `Mixed Content` errors.


!!! warning "CORS exists"

    When executing JS in the browser we are bound to the same rules as everyone on the web, such as CORS. If CORS are not enabled on the server you are fetching the converter from, then you will need to use a proxy such as [corsproxy.io](https://corsproxy.io).
