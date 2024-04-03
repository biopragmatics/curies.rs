# 🚀 Example in bare HTML files

When using the library directly in the client browser you will need to initialize the wasm binary with `await init()`, after that you can use the same functions as in the NodeJS environments.

You can easily import the NPM package from a CDN, and work with `curies` directly in a simple `index.html` file:

```html title="index.html"
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
      import init, { Record, Converter, getOboConverter } from "https://unpkg.com/@biopragmatics/curies";

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

Then just start the web server from the directory where the `index.html` file is with:

```bash
npx http-server
# Or:
python -m http.server
```
