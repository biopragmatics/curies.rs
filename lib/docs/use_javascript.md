# 🟨 Use from JavaScript

[![npm](https://img.shields.io/npm/v/@biopragmatics/curies)](https://www.npmjs.com/package/@biopragmatics/curies)

You can easily work with CURIEs from JavaScript, or TypeScript with the [`@biopragmatics/curies`](https://www.npmjs.com/package/@biopragmatics/curies) NPM package.

## 📥️ Install

Install the `npm` package (use `yarn` or `pnpm` if you prefer) to use it from your favorite framework:

```bash
npm install @biopragmatics/curies
```

## 🟢 Use it in a NodeJS environment

There are multiple methods available for creating or importing converters:

```ts
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
  const converterFromUrl = await Converter.fromExtendedPrefixMap("https://raw.githubusercontent.com/biopragmatics/bioregistry/main/exports/contexts/bioregistry.epm.json")

  // Load from a JSON-LD context (string or URI)
  const converterFromJsonld = await Converter.fromJsond("http://purl.obolibrary.org/meta/obo_context.jsonld");

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

### 🚀 In bare HTML files

You can easily import the NPM package from a CDN, and work with `curies` from a simple `index.html` file:

```html
<!DOCTYPE html>
<html lang="en-US">
  <head>
    <meta charset="utf-8" />
    <title>CURIEs example</title>
  </head>

  <body>
    <p id="compressed"></p>
    <p id="expanded"></p>

    <script type="module">
      import init, { Record, Converter, getOboConverter } from "./pkg/web.js";

      async function main() {
        await init();
        const converter = await getOboConverter();

        const curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234");
        const uri = converter.expand("DOID:1234");
        document.getElementById("compressed").innerText = curie;
        document.getElementById("expanded").innerText = uri;
      }
      main();
    </script>
  </body>
</html>
```

Then just start the web server from the directory where the HTML file is with:

```bash
npx http-server
# Or:
python -m http.server
```

### ⚛️ From any JavaScript framework

It can be used from any JavaScript framework, or NodeJS.

For example, to use it in a nextjs react app:

1. Create the project and `cd` into your new app folder

    ```bash
    npx create-next-app@latest --typescript
    ```

2. Add the `@biopragmatics/curies` dependency to your project:

    ```bash
    npm install --save @biopragmatics/curies
    ```

3. Add code, e.g. in `src/app/page.tsx` running on the client:

    ```typescript
    'use client'
    import { useEffect, useState } from 'react';
    import init, { getBioregistryConverter } from "@biopragmatics/curies";

    export default function Home() {
      const [output, setOutput] = useState('');
      useEffect(() => {

        // Initialize the wasm library and use it
        init().then(async () => {
          const converter = await getBioregistryConverter();
          const curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234");
          const uri = converter.expand("doid:1234");
          setOutput(`${curie}: ${uri}`);
        });
      }, []);

      return (
        <main>
          <p>{output}</p>
        </main>
      );
    }
    ```

4. Start in dev:

    ```bash
    npm run dev
    ```
