# Changelog

## Upcoming

- First major release `v1.0.0` coming soon.

## v0.2.1 - ????-??-??

### Added

- Items can now be positionally indexed by usize.

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