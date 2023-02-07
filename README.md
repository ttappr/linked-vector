# LinkedVector

- [Source Documentation](https://ttappr.github.io/linked-vector/doc/linked_vector/index.html)
    - [Primary Class (LinkedVector)](https://ttappr.github.io/linked-vector/doc/linked_vector/struct.LinkedVector.html)
- [GitHub Project](https://github.com/ttappr/linked-vector.git)

`LinkedVector` is a feature packed hybrid of a vector and linked list. Items are
accessible directly in `O(1)` time, and insertions and deletions also operate in
`O(1)` time. Internally, nodes exist within a vector, with each node holding 
handles to its previous and next neighbors. So there's no shifting of data when 
items are inserted or removed.

## Usage

Edit your Cargo.toml file to include:

```rust, ignore
[dependencies]
linked-vector = "0.2"
```
Or run this in the commandline from your project folder:

```console, ignore
cargo add linked-vector
```

## Handles

Items in a `LinkedVector` are directly accessible via handles. These are 
instances of the `HNode` struct. They're returned by operations such as insert, 
push, and getters. If direct access is required to any specific items, their 
handles can be stored for later use. These handles lack the performance overhead
of smart pointers, while providing a flexible reference model.

```rust
use linked_vector::*;
let mut lv = LinkedVector::new();

let handle_1 = lv.push_back(1);
let handle_2 = lv.push_back(2);

*lv.get_mut(handle_1).unwrap() = 42;
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
- **Indexing**:  Items can be indexed as with a vector using the handles.
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
lv.remove_node(h1);

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
let lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7]);
let mut cursor = lv.cursor();

assert_eq!(cursor.get(), Some(&1));

cursor.move_next();

assert_eq!(cursor.get(), Some(&2));

let hend = cursor.move_to_end().unwrap();
let hbak = cursor.backward(3).unwrap();

assert_eq!(cursor.get(), Some(&4));
assert_eq!(lv.get(hend), Some(&7));
assert_eq!(lv.get(hbak), Some(&4));
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
