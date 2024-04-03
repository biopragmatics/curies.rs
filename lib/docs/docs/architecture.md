# 🗺️ Architecture details

This page presents the project architecture and some technical details.

### ✨ Features

List of features availability per language binding, based on features defined in [curies.readthedocs.io](https://curies.readthedocs.io):

| Feature                                          | Rust (core) | Python | JS   | R    |
| ------------------------------------------------ | ----------- | ------ | ---- | ---- |
| compress                                         | ✅           | ✅      | ✅    | ✅    |
| expand                                           | ✅           | ✅      | ✅    | ✅    |
| compress_list                                    | ✅           | ✅      | ✅    |      |
| expand_list                                      | ✅           | ✅      | ✅    |      |
| standardize (prefix, curie, uri)                 |             |        |      |      |
| chain converters                                 | ✅           | ✅      | ✅    |      |
| Record object and converter.add_record()         | ✅           | ✅      | ✅    |      |
| converter.add_prefix(prefix, ns)                 | ✅           |        | ✅    |      |
| converter.get_prefixes() and .get_uri_prefixes() | ✅           |        |      |      |
| Load from prefix map                             | ✅           | ✅      | ✅    |      |
| Load from extended prefix map                    | ✅           | ✅      | ✅    |      |
| Load from JSON-LD                                | ✅           | ✅      | ✅    |      |
| Load from SHACL shape                            |             |        |      |      |
| Load OBO converter                               | ✅           | ✅      | ✅    |      |
| Load GO converter                                | ✅           | ✅      | ✅    |      |
| Load Bioregistry converter                       | ✅           | ✅      | ✅    | ✅    |
| Load Monarch converter                           | ✅           | ✅      | ✅    |      |
| Write converter to prefix map                    | ✅           |        |      |      |
| Write converter to extended prefix map           | ✅           |        |      |      |
| Write converter to JSON-LD                       | ✅           |        |      |      |
| Prefixes discovery                               |             |        |      |      |

⚠️ Important differences between rust core and bindings:

1. **Load prefix map**, extended prefix map and JSON-LD can take `HashMap` as input in rust. But for JS and python, we currently need to pass it as `String` (we need to figure out how to pass arbitrary objects). You can pass either a URL or a JSON object as string, the lib will automatically retrieve the content of the URL if it is one. The original python lib was taking directly JSON objects for all loaders, apart from SHACL which takes a URL (which was not convenient when wanting to provide a local SHACL file)
2. In rust **chain()** is a static function taking a list of converters, `chained = Converter::chain([conv1, conv2])`. In JS and python we cannot easily pass a list of complex objects like converters, so chain is a normal function that takes 1 converter to chain: `chained = conv1.chain(conv2)`
3. Currently **write** prefix map returns a HashMap, write extended map returns a JSON as string, and write JSON-LD returns `serde::json` type. In the original python lib it was writing to a file.

### 🗃️ Folder structure

```
curies.rs/
├── lib/
│   ├── src/
│   │   └── 🦀 Source code for the core Rust crate.
│   ├── tests/
│   │   └── 🧪 Tests for the core Rust crate.
│   └── docs/
│       └── 📖 Markdown and HTML files for the documentation website.
├── python/
│   └── 🐍 Python bindings
├── js/
│   └── 🟨 JavaScript bindings
├── r/
│   └── 📈 R bindings
├── scripts/
│   └── 🛠️ Development scripts (build, test, gen docs).
└── .github/
    └── workflows/
        └── ⚙️ Automated CI/CD workflows.
```

###
