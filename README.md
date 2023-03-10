# LinkedVector

`LinkedVector` is a feature packed hybrid of a vector and linked list. Items are
accessible directly in `O(1)` time, and insertions and deletions also operate in
`O(1)` time. Internally, nodes exist within a vector, with each node holding 
handles to its previous and next neighbors. So there's no shifting of data when 
items are inserted or removed.

## LFU Cache Example

An [example project](https://github.com/ttappr/lfu-cache.git) that demonstrates 
use of the `linked-vector` crate is available. The project is for a Least
Frequently Used Cache. `LinkedVector`'s are used to implement its frequency 
count queues.

## Updates

Version `v1.2.x` is a minor revision backward compatible with prior `v1.x.x`
versions. Users, however, must enable the `"cursor-remove"` feature explicitly.
This turns on the `CursorMut::remove()` method. If you weren't using 
`Cursor::remove()` before, then nothing needs to be done. Otherwise, you can 
update your `Cargo.toml` file to include the feature, 
[see usage notes below](#usage). 

### Feature: "optionless-accessors"

Version `v1.2.0` added a new `"optionless-accessors"` feature that can 
be enabled which implements some minor changes to a few existing methods for 
`LinkedVector` and `Cursor`. It is encouraged that this feature be enabled as 
it addresses certain nonsensical aspects of a few API methods.

With this feature enabled, methods such as `get(hnode)` and `get_mut(hnode)`
that take a handle return direct references to their values instead of an 
`Option` variant. These commands would fail on a bad handle anyway, so it 
doesn't make sense to return an `Option`. This feature is disabled by default 
so as not to break backward compatibility, but can be easily turned on, 
[see Usage notes](#usage)

### Feature: "cursor-remove"

The `LinkedVector` API disallows creating a cursor for an empty vector. If you 
have a cursor to a vector, then it's assumed it has items to traverse and/or
modify. Removing items can pose a slight danger in that the cursor's internal
reference to the current node becomes meaningless if all the items are removed.

So to ensure users are aware of this, the `"cursor-remove"` feature needs to be 
explicitly turned on. To verify whether you've emptied a vector through a 
cursor, the cursor provides an `is_empty()` method. Also the `remove()` method
returns an `Option` where a `None` indicates there are no more items to remove.


### Versioning Conventions:
- MAJOR version indicates incompatible API changes with previous major version.
- MINOR version indicates added functionality in a backwards-compatible manner.
- PATCH version indicates backwards-compatible bug fixes.

[Change Log](https://github.com/ttappr/linked-vector/blob/master/CHANGELOG.md)

## Usage

To use the `"optionless-accessors"` and `"cursor-remove"` features, edit your 
Cargo.toml file to include:

```rust, ignore
[dependencies]
linked-vector = { version = "1.2", features = ["cursor-remove", "optionless-accessors"] }
```

Or, to use `v1.2.0` with backward compatibility with existing `v1.1.0` code 
include:

```rust, ignore
[dependencies]
linked-vector = "1.2"
```

Or run this on the command line from your project folder:

```console, ignore
cargo add linked-vector --features "cursor-remove, optionless-accessors"
```
or without the new features:

```console, ignore
cargo add linked-vector
```

# Feature Summary
## Handles

Items in a `LinkedVector` are directly accessible via handles, which are 
instances of the `HNode` struct. These are returned by operations such as insert 
or push, or other accessor methods. If direct access is required to any specific 
items, their handles can be stored for later use. These handles lack the 
performance overhead of smart pointers, while providing a flexible reference 
model.

```rust
use linked_vector::*;
let mut lv = LinkedVector::new();

let handle_1 = lv.push_back(1);
let handle_2 = lv.push_back(2);

lv[handle_1] = 42;
lv[handle_2] = 99;

assert_eq!(lv[handle_1], 42);
assert_eq!(lv[handle_2], 99);

```
## Recycling

Nodes within `LinkedVector` are added to a recycling list when they're popped,
or otherwise removed. If a `LinkedVector` has any nodes in this list, one will 
be used for the next insert or push operation. This strategy avoids segmenting 
the vector with dead vector cells. When a node is added to the recycling list, 
it isn't moved in the vector - its next and previous fields are updated to link
it into the recycling list.

## Debug Features

For release builds, the checks described in this section are excluded to ensure 
fast performance. In release, handles are simply transparent `usize` indexes 
into the `LinkedVector`'s internal vector.

When run with the debug build, handles have additional fields added: a UUID 
field, and a generation ID. The UUID field is used to verify handles are native 
to the `LinkedVector` they're passed to. And the generation ID is used to detect
expired handles. 

These features should help ensure that projects that use this crate don't have 
elusive bugs in scenarios such as passing an old handle to a vector for a node 
that had been popped earlier, or obtaining a handle from one vector and 
accidentally passing it to another.

## Economy

`LinkedVector`'s struct is implemented in a minimalistic manner. It contains
only 4 fields: one for the internal vector, another that holds a handle to the
head node, another with a handle to the recycling list, and lastly the length
field. 

There are no dummy nodes in the vector - all active nodes are data, and there's
no field in the `LinkedVector` struct for a tail handle, although the vector
does indeed have a tial node accessible in `O(1)` time.

## Other Features

- **Cursors**:   The Cursor interface facilitates traversing the vector from any 
                 point.
- **Indexing**:  `Index<HNode>` and `Index<usize>` are implemented, enabling 
                 items to be accessed directly.
- **Iterators**: The standard assortment of double-ended iterators are 
                 implemented.
- **Sorting**:   In-place sorting of elements is supported in `O(n log n)` time.


# Examples
## Handles

Operations that alter the `LinkedVector` return handles that can be saved for
later use. These provide direct access to items in `O(1)` time.


```rust
use linked_vector::*;
let mut lv = LinkedVector::new();

let h1 = lv.push_back(1);
let h2 = lv.push_back(2);
let h3 = lv.push_back(3);
let h4 = lv.insert_after(h1, 4);

lv.insert_after(h2, 42);
lv.remove(h1);

assert_eq!(lv.front(), Some(&4));
assert_eq!(lv.to_vec(), vec![4, 2, 42, 3]);

```

## Cursors

A cursor can be requested from the `LinkedVector` to facilitate traversal of 
nodes. Using a handle to specify starting position, cursors can be set to the
location within the vector accordingly. They can move one position at a time, 
or several via `forward(n_times)` and `backward(n_ntimes)`.

```rust
use linked_vector::*;
let lv     = LinkedVector::from([1, 2, 3, 4, 5, 6, 7]);
let hfront = lv.front_node().unwrap();

let mut cursor = lv.cursor(hfront);

assert_eq!(*cursor, 1);

cursor.move_next();

assert_eq!(*cursor, 2);

let hend = cursor.move_to_back().expect("Moving to end");
let hbak = cursor.backward(3).expect("Moving back 3");

assert_eq!(*cursor, 4);
assert_eq!(lv[hend], 7);
assert_eq!(lv[hbak], 4);

```
## Iterators

`LinkedVector` implements the standard set of double-ended iterators. They can
be instantiated directly via methods such as `iter()`, or implicitly.

```rust
use linked_vector::*;
let mut lv1 = LinkedVector::from([1, 2, 3]);

lv1.iter_mut().zip(7..).for_each(|(a, b)| *a = b);
lv1.iter().zip(7..).for_each(|(a, b)| assert_eq!(a, &b));

for (v1, v2) in (10..).zip(&mut lv1) {
    *v2 = v1;
}
lv1.iter().zip(10..).for_each(|(a, b)| assert_eq!(a, &b));
```
