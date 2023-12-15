# 🐍 Use from Python

[![PyPI](https://img.shields.io/pypi/v/curies-rs)](https://pypi.org/project/curies-rs/)

You can easily work with `curies` from Python.

```admonish warning title="Work in progress"
This package is a work in progress. This documentation might not be always up-to-date.
```

## 📥️ Install

Install the `pip` package:

```bash
pip install curies-rs
```

## 🚀 Use

Create a `Converter`, and expand/compress:

```python
from curies_rs import Record, Converter

rec1 = Record("doid", "http://purl.obolibrary.org/obo/DOID_", [], [])

converter = Converter()
converter.add_record(rec1)

uri = converter.compress("http://purl.obolibrary.org/obo/DOID_1234")

print(uri)

print(rec1.dict())
```

Run the script:

```bash
python curies.py
```
