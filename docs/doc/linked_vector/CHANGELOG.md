# Changelog

## v1.0.0 - 2023-02-09

### Added

- `compact()` method added to `LinkedVector` that produces a new compacted 
  vector.
- `LinkedVector` gets new stable sort methods: `sort()`, `sort_by()`, and 
  `sort_by_key()`.

### Changed

- The unstable sort methods reimplemented to use Rust's standard library's built 
  in sorting routines.

### Removed

- `LinkedVector::swap()` removed since the sort methods no longer need it.

### Fixed

- Issue where expired handles weren't detected if the node had been recycled.

## v0.3.0 - 2023-02-08

### Added

- `CursorMut::insert()` & `CursorMut::insert_after()`
- `CursorMut::remove()`

### Changed

- Methods that take a `usize` index have been removed to encourage the O(1) use
  of handles to locate data, and to simplify the naming conventions of methods.  
- The following methods have been changed:
  - `handle_at()` is now just `handle()`
  - `cursor_at()` is now `cursor()`, the old `cursor()` is removed.
  - `cursor_at_mut()` is now `cursor_mut()`, the old `cursor_mut()` is removed.
  - `insert_before()` is now `insert()`, the old `insert()` is removed.
  - `remove_node()` is now `remove()`, the old `remove()` is removed.
  - `remove_value()` is removed.

## v0.2.0 - 2023-02-07

### Added

- A new `remove()` that takes the index of the item to remove.
- An `insert()` that takes the index of the position to insert to, and the value
  to insert.
- `get_handle()`, which returns the handle of the `index`'th item.

### Changed

- Changed the name of `remove()` to `remove_value()`. 

## v0.1.5 - 2023-02-07

### Fixes

- Fixed issue with detection of expired handles.

## v0.1.4 - 2023-02-07

### Added

- Changelog created.
- Detection of expired handles for debug build.

### Changed

- README.md updated.
- Comments updated with more tests/examples for sorting.