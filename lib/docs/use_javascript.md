# 🟨 Use from JavaScript

[![npm](https://img.shields.io/npm/v/@biopragmatics/curies)](https://www.npmjs.com/package/@biopragmatics/curies)

You can easily work with CURIEs from JavaScript, or TypeScript with the [`@biopragmatics/curies`](https://www.npmjs.com/package/@biopragmatics/curies) NPM package.

## 📥️ Install

Install the `npm` package (use `yarn` or `pnpm` if you prefer) to use it from your favorite framework:

```bash
npm install @biopragmatics/curies
```

## 🚀 Use it in bare HTML files

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
        const rec1 = new Record("obo", "http://purl.obolibrary.org/obo/", [], []);
        console.log(rec1.toString());
        console.log(rec1.toJs());

        // Populate the Converter with records, or import existing converters:
        // const converter = new Converter();
        // converter.addRecord(rec1);

        const converter = await getOboConverter();
        console.log(converter.toString())

        const compressedUri = converter.compress("http://purl.obolibrary.org/obo/DOID_1234");
        const expandedUri = converter.expand("DOID:1234");
        document.getElementById("compressed").innerText = compressedUri;
        document.getElementById("expanded").innerText = expandedUri;
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

## ⚛️ Use from any JavaScript framework

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

3. Add code, e.g. in `src/app/page.tsx`:

    ```typescript
    'use client'
    import { useEffect, useState } from 'react';
    import init, { Converter, Record } from "@biopragmatics/curies";

    export default function Home() {
      const [output, setOutput] = useState('');
      useEffect(() => {

        // Initialize the wasm library and use it
        init().then(async () => {
          const rec1 = new Record("obo", "http://purl.obolibrary.org/obo/", [], []);
          console.log(rec1.toString());
          console.log(rec1.toJs());

          // Populate the Converter with records, or import existing converters:
          const converter = new Converter();
          converter.addRecord(rec1);
          console.log(converter.toString())

          const compressedUri = converter.compress("http://purl.obolibrary.org/obo/DOID_1234");
          const expandedUri = converter.expand("DOID:1234");
          setOutput(compressedUri);
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
