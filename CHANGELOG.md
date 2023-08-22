# v0.3.2
* `num_found_exact` was introduced in Solr 8.6. This caused deserialization to fail on older versions.
  Changed so that it will be emulated as `true` for older versions.

# v0.3.1
* Fix error in python documentation
* 
# v0.3.0
* Add Facet sets
* Add Json facets
* Be more permissive with arguments to builders, using `Into<Option>`, `Into<String` and `IntoIterator` where appropriate
* Rename builders removing `Builder` suffix

# v0.2.0
* Add query parsers (lucene, dismax, edismax)

# v0.1.1
* Fix error in rust setup documentation example
* Add mdbook, and pydoc documentation