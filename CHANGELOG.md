# Changelog

## Upcoming

- When the interface to `LinkedVector` has no more immediate changes needed,
  the first major release `v1.0.0` will follow. This should happen soon. The
  SemVer version numbering conventions will be more closely adhered to 
  thereafter.

## v0.2.0 - ??
---

### Added

- A `remove()` that takes the index of the item to remove.
- An `insert()` that takes the index of the position to insert to, and the value
  to insert.
- `get_handle()`, which returns the handle of the `index`'th item.

### Changed

- Changed the name of `remove()` to `remove_value()`. Code that depended on
  the old version will break with this change. Please update to the new 
  function.

## v0.1.5 - 2023-02-07
---

### Fixes

- Fixed issue with detection of expired handles.

## v0.1.4 - 2023-02-07
---

### Added

- Changelog created.
- Detection of expired handles for debug build.

### Changed

- README.md updated.
- Comments updated with more tests/examples for sorting.