# ⚛️ Use from any JavaScript framework

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

    ```typescript title="src/app/page.tsx"
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
