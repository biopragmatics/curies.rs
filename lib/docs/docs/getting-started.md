# 🚀 Getting started

You can easily work with `curies` from various languages.

## 📥️ Installation

Install the package for you language:

=== "Python"

    ```bash
    pip install curies-rs
    ```

=== "JavaScript"

    ```bash
    npm install --save @biopragmatics/curies
    # or
    pnpm add @biopragmatics/curies
    # or
    yarn add @biopragmatics/curies
    # or
    bun add @biopragmatics/curies
    ```

=== "Rust"

    ```bash
    cargo add curies
    ```

## 🚀 Usage

Initialize a converter, then use it to `compress` URIs to CURIEs, or `expand` CURIEs to URIs:

=== "Python"

    ```python
    from curies_rs import get_bioregistry_converter

    # Initialize converter (here we use the predefined Bioregistry converter)
    converter = get_bioregistry_converter()

    # Compress a URI, or expand a CURIE
    curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")
    uri = converter.expand("DOID:1234")

    # Compress/expand a list of URIs or CURIEs
    curies = converter.compress_list(["http://purl.obolibrary.org/obo/DOID_1234"])
    uris = converter.expand_list(["DOID:1234", "doid:1235"])

    # Standardize prefix, CURIEs, and URIs using the preferred alternative
    assert converter.standardize_prefix("gomf") == "go"
    assert converter.standardize_curie("gomf:0032571") == "go:0032571aaaaaaa"
    assert converter.standardize_uri("http://amigo.geneontology.org/amigo/term/GO:0032571") == "http://purl.obolibrary.org/obo/GO_0032571"
    ```

=== "JavaScript"

    ```javascript
    import {getBioregistryConverter} from "@biopragmatics/curies";

    async function main() {
        // Initialize converter (here we use the predefined Bioregistry converter)
        const converter = await getBioregistryConverter();

        // Compress a URI, or expand a CURIE
        const curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234");
        const uri = converter.expand("doid:1234");

        // Compress/expand a list of URIs or CURIEs
        const curies = converter.compressList(["http://purl.obolibrary.org/obo/DOID_1234"]);
        const uris = converter.expandList(["doid:1234"]);

        // Standardize prefix, CURIEs, and URIs using the preferred alternative
        console.log(converter.standardizePrefix("gomf"))
        console.log(converter.standardizeCurie("gomf:0032571"))
        console.log(converter.standardizeUri("http://amigo.geneontology.org/amigo/term/GO:0032571"))
    }
    main();
    ```

    !!! warning "Running in the browser requires initialization"

        When writing code that will be executed in the browser you need to first initialize the Wasm binary:

        ```javascript
        import init, { Record, Converter, getOboConverter } from "@biopragmatics/curies";

        async function main() {
            await init();
            const converter = await getOboConverter();
            const uri = converter.expand("DOID:1234");
        }
        main();
        ```

        This is not required when running JavaScript code on server-side, e.g. using NodeJS.

    !!! danger "CORS exists"

        When executing JS in the browser we are bound to the same rules as everyone on the web, such as CORS. If CORS are not enabled on the server you are fetching the converter from, then you will need to use a proxy such as [corsproxy.io](https://corsproxy.io).

    !!! bug "Use HTTPS when importing"

        When loading converters from URLs in JS always prefer using HTTPS URLs, otherwise you will face `Mixed Content` errors.


=== "Rust"

    ```rust
    use curies::sources::get_bioregistry_converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize converter (here we use the predefined Bioregistry converter)
        let converter = get_bioregistry_converter().await?;

        // Compress a URI, or expand a CURIE
        let uri = converter.expand("doid:1234")?;
        let curie = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")?;

        // Compress/expand a list of URIs or CURIEs
        let uris = converter.expand_list(vec!["doid:1234"]);
        let curies = converter.compress_list(vec!["http://purl.obolibrary.org/obo/DOID_1234"]);

        // Standardize prefix, CURIEs, and URIs using the preferred alternative
        assert_eq!(converter.standardize_prefix("gomf").unwrap(), "go");
        assert_eq!(converter.standardize_curie("gomf:0032571").unwrap(), "go:0032571");
        assert_eq!(converter.standardize_uri(
            "http://amigo.geneontology.org/amigo/term/GO:0032571").unwrap(),
            "http://purl.obolibrary.org/obo/GO_0032571",
        );
        Ok(())
    }
    ```

## 🌀 Loading a Context

There are several ways to load a context with this package, including:

1. pre-defined contexts
2. contexts encoded in the standard prefix map format
3. contexts encoded in the standard JSON-LD context format
4. contexts encoded in the extended prefix map format

### 📦 Loading a predefined context

Easiest way to get started is to simply use one of the function available to import a converter from popular namespaces registries:

**[Bioregistry](https://bioregistry.io/) converter**

=== "Python"

    ```python
    from curies_rs import get_bioregistry_converter

    converter = get_bioregistry_converter()
    ```

=== "JavaScript"

    ```javascript
    import {getBioregistryConverter} from "@biopragmatics/curies";

    async function main() {
        const converter = await getBioregistryConverter();
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::sources::get_bioregistry_converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = get_bioregistry_converter().await?;
        Ok(())
    }
    ```

**[OBO](http://obofoundry.org/) converter**

=== "Python"

    ```python
    from curies_rs import get_obo_converter

    converter = get_obo_converter()
    ```

=== "JavaScript"

    ```javascript
    import {getOboConverter} from "@biopragmatics/curies";

    async function main() {
        const converter = await getOboConverter();
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::sources::get_obo_converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = get_obo_converter().await?;
        Ok(())
    }
    ```


**[GO](https://geneontology.org/) converter**

=== "Python"

    ```python
    from curies_rs import get_go_converter

    converter = get_go_converter()
    ```

=== "JavaScript"

    ```javascript
    import {getGoConverter} from "@biopragmatics/curies";

    async function main() {
        const converter = await getGoConverter();
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::sources::get_go_converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = get_go_converter().await?;
        Ok(())
    }
    ```


**[Monarch Initiative](https://monarchinitiative.org/) converter**

=== "Python"

    ```python
    from curies_rs import get_monarch_converter

    converter = get_monarch_converter()
    ```

=== "JavaScript"

    ```javascript
    import {getMonarchConverter} from "@biopragmatics/curies";

    async function main() {
        const converter = await getMonarchConverter();
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::sources::get_monarch_converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = get_monarch_converter().await?;
        Ok(())
    }
    ```

### 🗺️ Loading Extended Prefix Maps

Enable to provide prefix/URI synonyms and ID RegEx pattern for each record:

=== "Python"

    ```python
    from curies_rs import Converter

    extended_pm = """[
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
            "prefix": "OBO",
            "prefix_synonyms": [
                "obo"
            ],
            "uri_prefix": "http://purl.obolibrary.org/obo/"
        }
    ]"""
    converter = Converter.from_extended_prefix_map(extended_pm)
    ```

=== "JavaScript"

    ```javascript
    import {Converter} from "@biopragmatics/curies";

    async function main() {
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
                "prefix": "OBO",
                "prefix_synonyms": [
                    "obo"
                ],
                "uri_prefix": "http://purl.obolibrary.org/obo/"
            }
        ]`)
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::Converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = Converter::from_extended_prefix_map(r#"[
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
                "prefix": "OBO",
                "prefix_synonyms": [
                    "obo"
                ],
                "uri_prefix": "http://purl.obolibrary.org/obo/"
            }
        ]"#).await?;
        Ok(())
    }
    ```

!!! tip "Support URL"

    For all `Converter.from` functions you can either provide the file content, or the URL to the file as string.

### 📍 Loading Prefix Maps

A simple dictionary without synonyms information:

=== "Python"

    ```python
    from curies_rs import Converter

    prefix_map = """{
        "GO": "http://purl.obolibrary.org/obo/GO_",
        "DOID": "http://purl.obolibrary.org/obo/DOID_",
        "OBO": "http://purl.obolibrary.org/obo/"
    }"""
    converter = Converter.from_prefix_map(prefix_map)
    ```

=== "JavaScript"

    ```javascript
    import {Converter} from "@biopragmatics/curies";

    async function main() {
        const converter = await Converter.fromPrefixMap(`{
            "GO": "http://purl.obolibrary.org/obo/GO_",
            "DOID": "http://purl.obolibrary.org/obo/DOID_",
            "OBO": "http://purl.obolibrary.org/obo/"
        }`)
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::Converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = Converter::from_prefix_map(r#"{
            "GO": "http://purl.obolibrary.org/obo/GO_",
            "DOID": "http://purl.obolibrary.org/obo/DOID_",
            "OBO": "http://purl.obolibrary.org/obo/"
        }"#).await?;
        Ok(())
    }
    ```

### 📄 Loading JSON-LD contexts

=== "Python"

    ```python
    from curies_rs import Converter

    jsonld = """{
        "@context": {
            "GO": "http://purl.obolibrary.org/obo/GO_",
            "DOID": "http://purl.obolibrary.org/obo/DOID_",
            "OBO": "http://purl.obolibrary.org/obo/"
        }
    }"""
    converter = Converter.from_jsonld(jsonld)
    ```

    Or directly use a URL:

    ```python
    from curies_rs import Converter

    converter = Converter.from_jsonld("https://purl.obolibrary.org/meta/obo_context.jsonld")
    ```

=== "JavaScript"

    ```javascript
    import {Converter} from "@biopragmatics/curies";

    async function main() {
        const converter = await Converter.fromJsonld(`{
            "@context": {
                "GO": "http://purl.obolibrary.org/obo/GO_",
                "DOID": "http://purl.obolibrary.org/obo/DOID_",
                "OBO": "http://purl.obolibrary.org/obo/"
            }
        }`)
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::Converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = Converter::from_jsonld(r#"{
            "@context": {
                "GO": "http://purl.obolibrary.org/obo/GO_",
                "DOID": "http://purl.obolibrary.org/obo/DOID_",
                "OBO": "http://purl.obolibrary.org/obo/"
            }
        }"#).await?;
        Ok(())
    }
    ```

### 🔗 Loading SHACL prefixes definitions

=== "Python"

    ```python
    from curies_rs import Converter

    shacl = """@prefix sh: <http://www.w3.org/ns/shacl#> .
    @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
    [
    sh:declare
        [ sh:prefix "dc" ; sh:namespace "http://purl.org/dc/elements/1.1/"^^xsd:anyURI  ],
        [ sh:prefix "dcterms" ; sh:namespace "http://purl.org/dc/terms/"^^xsd:anyURI  ],
        [ sh:prefix "foaf" ; sh:namespace "http://xmlns.com/foaf/0.1/"^^xsd:anyURI  ],
        [ sh:prefix "xsd" ; sh:namespace "http://www.w3.org/2001/XMLSchema#"^^xsd:anyURI  ]
    ] ."""
    conv = Converter.from_shacl(shacl)
    ```

=== "JavaScript"

    ```javascript
    import {Converter} from "@biopragmatics/curies";

    async function main() {
        const converter = await Converter.fromShacl(`@prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        [
        sh:declare
            [ sh:prefix "dc" ; sh:namespace "http://purl.org/dc/elements/1.1/"^^xsd:anyURI  ],
            [ sh:prefix "dcterms" ; sh:namespace "http://purl.org/dc/terms/"^^xsd:anyURI  ],
            [ sh:prefix "foaf" ; sh:namespace "http://xmlns.com/foaf/0.1/"^^xsd:anyURI  ],
            [ sh:prefix "xsd" ; sh:namespace "http://www.w3.org/2001/XMLSchema#"^^xsd:anyURI  ]
        ] .`)
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::Converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = Converter::from_shacl(r#"@prefix sh: <http://www.w3.org/ns/shacl#> .
        @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        [
        sh:declare
            [ sh:prefix "dc" ; sh:namespace "http://purl.org/dc/elements/1.1/"^^xsd:anyURI  ],
            [ sh:prefix "dcterms" ; sh:namespace "http://purl.org/dc/terms/"^^xsd:anyURI  ],
            [ sh:prefix "foaf" ; sh:namespace "http://xmlns.com/foaf/0.1/"^^xsd:anyURI  ],
            [ sh:prefix "xsd" ; sh:namespace "http://www.w3.org/2001/XMLSchema#"^^xsd:anyURI  ]
        ] ."#).await?;
        Ok(())
    }
    ```

## 🔎 Introspecting on a Context

After loading a context, it’s possible to get certain information out of the converter. For example, if you want to get all of the CURIE prefixes from the converter, you can use `converter.get_prefixes()`:

=== "Python"

    ```python
    from curies_rs import get_bioregistry_converter

    converter = get_bioregistry_converter()

    prefixes = converter.get_prefixes()
    assert 'chebi' in prefixes
    assert 'CHEBIID' not in prefixes, "No synonyms are included by default"

    prefixes = converter.get_prefixes(include_synonyms=True)
    assert 'chebi' in prefixes
    assert 'CHEBIID' in prefixes
    ```

=== "JavaScript"

    ```javascript
    import {getBioregistryConverter} from "@biopragmatics/curies";

    async function main() {
        const converter = await getBioregistryConverter();

        const prefixes = converter.getPrefixes();
        // Synonyms are not included by default
        const prefixes_incl_syn = converter.getPrefixes(true);
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::sources::get_bioregistry_converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = get_bioregistry_converter().await?;

        // Argument to include or not synonyms
        let prefixes = converter.get_prefixes(false)
        let prefixes_incl_syn = converter.get_prefixes(true)
        Ok(())
    }
    ```

Similarly, the URI prefixes can be extracted with `Converter.get_uri_prefixes()` like in:

=== "Python"

    ```python
    from curies_rs import get_bioregistry_converter

    converter = get_bioregistry_converter()

    uri_prefixes = converter.get_uri_prefixes()
    assert 'http://purl.obolibrary.org/obo/CHEBI_' in uri_prefixes
    assert 'https://bioregistry.io/chebi:' not in uri_prefixes, "No synonyms are included by default"

    uri_prefixes = converter.get_uri_prefixes(include_synonyms=True)
    assert 'http://purl.obolibrary.org/obo/CHEBI_' in uri_prefixes
    assert 'https://bioregistry.io/chebi:' in uri_prefixes
    ```

=== "JavaScript"

    ```javascript
    import {getBioregistryConverter} from "@biopragmatics/curies";

    async function main() {
        const converter = await getBioregistryConverter();

        const prefixes = converter.getUriPrefixes();
        // Synonyms are not included by default
        const prefixes_incl_syn = converter.getUriPrefixes(true);
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::sources::get_bioregistry_converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = get_bioregistry_converter().await?;

        // Argument to include or not synonyms
        let prefixes = converter.get_uri_prefixes(false)
        let prefixes_incl_syn = converter.get_uri_prefixes(true)
        Ok(())
    }
    ```

It’s also possible to get a bijective prefix map, i.e., a dictionary from primary CURIE prefixes to primary URI prefixes. This is useful for compatibility with legacy systems which assume simple prefix maps. This can be done with the `write_prefix_map()` function like in the following:

=== "Python"

    ```python
    import json
    from curies_rs import get_bioregistry_converter

    converter = get_bioregistry_converter()

    prefix_map = json.loads(converter.write_prefix_map())
    assert prefix_map['chebi'] == 'http://purl.obolibrary.org/obo/CHEBI_'
    ```

=== "JavaScript"

    ```javascript
    import {getBioregistryConverter} from "@biopragmatics/curies";

    async function main() {
        const converter = await getBioregistryConverter();

        prefix_map = JSON.parse(converter.writePrefixMap());
        console.log(prefix_map['chebi']);
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::sources::get_bioregistry_converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = get_bioregistry_converter().await?;

        let prefix_map = converter.write_prefix_map();
        // This returns a HashMap
        match prefix_map.get("chebi") {
            Some(value) => println!("{}", value),
            None => println!("Key not found"),
        }
        Ok(())
    }
    ```

## 🛠️ Modifying a Context

### 🔨 Incremental Converters

New data can be added to an existing converter with either `converter.add_prefix()` or `converter.add_record()`. For example, a CURIE and URI prefix for HGNC can be added to the OBO Foundry converter with the following:

=== "Python"

    ```python
    from curies_rs import get_obo_converter

    converter = get_obo_converter()
    converter.add_prefix("hgnc", "https://bioregistry.io/hgnc:")
    ```

=== "JavaScript"

    ```javascript
    import {Converter, Record} from "@biopragmatics/curies";

    async function main() {
        // Populate from Records
        const rec1 = new Record("obo", "http://purl.obolibrary.org/obo/", [], []);

        console.log(rec1.toString());
        console.log(rec1.toJs());
        const converter = new Converter();
        converter.addRecord(rec1);
        converter.addPrefix("hgnc", "https://bioregistry.io/hgnc:");
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::sources::get_obo_converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut converter = get_obo_converter().await?;

        converter.add_prefix("hgnc", "https://bioregistry.io/hgnc:")?;
        Ok(())
    }
    ```

Alternatively you can construct a `Record` object, which allows to pass synonyms lists, and start from a blank `Converter`:

=== "Python"

    ```python
    from curies_rs import Converter, Record

    converter = Converter()
    record = Record(
        prefix="hgnc",
        uri_prefix="https://bioregistry.io/hgnc:",
        prefix_synonyms=["HGNC"],
        uri_prefix_synonyms=["https://identifiers.org/hgnc/"],
    )
    converter.add_record(record)
    ```

=== "JavaScript"

    ```javascript
    import {Converter, Record} from "@biopragmatics/curies";

    async function main() {
        const converter = new Converter();
        const record = new Record("hgnc", "https://bioregistry.io/hgnc:", ["HGNC"], ["https://identifiers.org/hgnc/"]);
        converter.addRecord(record);
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::{Converter, Record};
    use std::collections::HashSet;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut converter = Converter::default();

        let record = Record {
            prefix: "hgnc".to_string(),
            uri_prefix: "https://bioregistry.io/hgnc:".to_string(),
            prefix_synonyms: HashSet::from(["HGNC".to_string()]),
            uri_prefix_synonyms: HashSet::from(["https://identifiers.org/hgnc/"].map(String::from)),
            pattern: None,
        };
        converter.add_record(record)?;
        Ok(())
    }
    ```

By default, both of these operations will fail if the new content conflicts with existing content. If desired, the `merge` argument can be set to true to enable merging. Further, checking for conflicts and merging can be made to be case insensitive by setting `case_sensitive` to false.

Such a merging strategy is the basis for wholesale merging of converters, described below.

### ⛓️ Chaining and merging

Chain together multiple converters, prioritizes based on the order given. Therefore, if two prefix maps having the same prefix but different URI prefixes are given, the first is retained. The second is retained as a synonym

=== "Python"

    ```python
    from curies_rs import get_obo_converter, get_go_converter, get_monarch_converter

    converter = (
        get_obo_converter()
            .chain(get_go_converter())
            .chain(get_monarch_converter())
    )
    ```

=== "JavaScript"

    ```javascript
    import {getOboConverter, getGoConverter, getMonarchConverter} from "@biopragmatics/curies";

    async function main() {
        const converter = await getOboConverter()
            .chain(await getGoConverter())
            .chain(await getMonarchConverter());
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::Converter;
    use curies::sources::{get_obo_converter, get_go_converter, get_monarch_converter};

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = Converter::chain(vec![
            get_obo_converter().await?,
            get_go_converter().await?,
            get_monarch_converter().await?,
        ])?;;
        Ok(())
    }
    ```

<!-- TODO: Subsetting? -->

## ✒️ Writing a Context

Write the converter prefix map as a string in different serialization format:

=== "Python"

    ```python
    from curies_rs import get_bioregistry_converter

    converter = get_bioregistry_converter()

    epm = converter.write_extended_prefix_map()
    pm = converter.write_prefix_map()
    jsonld = converter.write_jsonld()
    shacl = converter.write_shacl()
    ```

=== "JavaScript"

    ```javascript
    import {getBioregistryConverter} from "@biopragmatics/curies";

    async function main() {
        const converter = await getBioregistryConverter();

        const epm = converter.writeExtendedPrefixMap()
        const pm = converter.writePrefixMap()
        const jsonld = converter.writeJsonld()
        const shacl = converter.writeShacl()
    }
    main();
    ```

=== "Rust"

    ```rust
    use curies::sources::get_bioregistry_converter;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let converter = get_bioregistry_converter().await?;

        let epm = converter.write_extended_prefix_map()?;
        let pm = converter.write_prefix_map();
        let jsonld = converter.write_jsonld();
        let shacl = converter.write_shacl()?;
        Ok(())
    }
    ```
