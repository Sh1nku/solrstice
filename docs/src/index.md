# Solrstice Docs

## Introduction

Solrstice is a library for interacting with an Apache Solr cluster
Currently version `8` and `9` is supported.
The library is written in Rust, and has a wrapper to Python. Both async and blocking are supported in both languages.

### Rust

You can install the library by putting this in your `Cargo.toml`

```toml
solrstice = { version = "0.12.0", features = ["blocking"] }
```

If the `blocking` feature is not provided, only async will work.

* [Rust mdBook docs]()
* [Rust api docs](https://docs.rs/solrstice/) on docs.rs

### Python

```bash
pip install solrstice
```

* [Python docs](https://pypi.org/project/solrstice/)

## Getting started

### Creating a client

{{#inject_docstring ../framework/src/docs/create_client_test.rs}}

### Creating a collection

{{#inject_docstring ../framework/src/docs/create_collection_test.rs}}

### Indexing data

{{#inject_docstring ../framework/src/docs/index_data_test.rs}}

### Selecting data

{{#inject_docstring ../framework/src/docs/select_data_test.rs}}

### Deleting data

{{#inject_docstring ../framework/src/docs/delete_data_test.rs}}