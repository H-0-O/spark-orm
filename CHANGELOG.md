# Changelog

## [0.3.0] - 2024-07-05

### Added
- **Observer Callbacks**: Introduced the `Observer` trait with `created`, `updated`, and `deleted` methods to allow custom behavior after document operations. The `save` and `delete` methods in the `Model` struct now trigger these observer callbacks.
    - `created`: Called when a new document is created.
    - `updated`: Called when an existing document is updated (only when using `save`).
    - `deleted`: Called when a document is deleted.
- **Method: `save`**: Now calls `M::created` and `M::updated` as appropriate to handle post-save operations.
- **Method: `delete`**: Now calls `M::deleted` to handle post-delete operations.