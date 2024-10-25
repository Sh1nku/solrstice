# v0.6.0
* Breaking changes to error handling. Clearer error messages.

# v0.5.0

* Add logging of solr requests
* Move most items into the top level module namespace in both Rust and Python
* Rename `SolrError` to `Error`

# v0.4.3

* Fix mypy not recognizing .pyi files
* Add static type checking test for mypy and pyright

# v0.4.2

* Switch out openssl for rustls
* Run publish CI when creating PRs

# v0.4.1

* Relax version requirements.
* Add Python 3.12 to CI
* Note: Not released to PyPi due to relying on openssl which could not run in manylinux

# v0.4.0

* Make authentication error into its own error, instead of Json decode error
* Make inherited error types transparently pass through parent error

# v0.3.2

* `num_found_exact` was introduced in Solr 8.6. This caused deserialization to fail on older versions.
  Changed so that it will be emulated as `true` for older versions.

# v0.3.1

* Fix error in python documentation

# v0.3.0

* Add Facet sets
* Add Json facets
* Be more permissive with arguments to builders, using `Into<Option>`, `Into<String` and `IntoIterator` where
  appropriate
* Rename builders removing `Builder` suffix

# v0.2.0

* Add query parsers (lucene, dismax, edismax)

# v0.1.1

* Fix error in rust setup documentation example
* Add mdbook, and pydoc documentation