# LinkedVector

[Source Documentation](https://ttappr.github.io/linked-vector/)

`LinkedVector` is a hybrid of a vector and linked list. Items are accessible
directly in `O(1)` time, and insertions and deletions also operate in `O(1)`
time. Internally, nodes exist within a contiguous vector, with each node holding 
handles to its previous and next neighbors. So there's no shifting of data when
items are inserted or removed.

## Usage

```
[dependencies]
linked-vector = { git = "https://github.com/ttappr/linked-vector.git" }
```

## Handles

Items in a `LinkedVector` are directly accessible via the `HNode` struct. These
are returned by operations such as insert or push operations. If direct access
is required to any specific items, their handles can be stored for later use.

Internally, a handle is an index into the vector that holds the nodes. Care 
should be taken to avoid using the handles from one `LinkedVector` with another
instance. For the debug builds, handles are checked to ensure they are "native"
to the `LinkedVector` they're passed to when calling its methods. This can help
catch errors in unit tests. This checking is not done when built in release 
mode.

```rust
use LinkedVector::*;

let mut lv = LinkedVector::new();

let handle_1 = lv.push_back(1);
let handle_2 = lv.push_back(2);

*lv.get_mut(handle_1).unwrap() = 42;
```
## Recycling

Nodes within `LinkedVector` are added to a recycling list when they're popped,
or otherwise removed. If a `LinkedVector` has any nodes in this list, one will 
be used for the next insert or push operation. This strategy avoids segmenting 
the vector with dead vector cells. When a node is added to the recycling list, 
it isn't moved in the vector - its next and previous fields are updated to link
it into the recycling list.

# Examples
## Accessing Items Using Handles

Operations that alter the `LinkedVector` return handles that can be saved for
later use. These provide direct access to items in `O(1)` time.


```rust
use LinkedVector::*;

let mut lv = LinkedVector::new();

let h1 = lv.push_back(1);
let h2 = lv.push_back(2);
let h3 = lv.push_back(3);
let h4 = lv.insert_after(h1, 4);

lv.insert_after(h2, 42);
lv.remove(h1);

assert_eq!(lv.front(), Some(&2));
assert_eq!(lv.to_vec(), vec![4, 2, 42, 3]);

```

## Traversal With Cursors

A cursor can be requested from the `LinkedVector` to facilitate traversal of 
nodes. Using a handle to specify starting position, cursors can be set to the
location within the vector accordingly. They can move one position at a time, 
or several via `forward(n_times)` and `backward(n_ntimes)`.

```rust
use linked_vector::*;

let lv = LinkedVector::from([1, 2, 3]);
let mut cursor = lv.cursor(None);

assert_eq!(cursor.get(), Some(&1));

cursor.move_next();

assert_eq!(cursor.get(), Some(&2));

```
## Iterators

`LinkedVector` implements the standard set of double-ended iterators. The can
be instantiated directly vie methods such as `iter()`, or implicitly.

```rust
let mut lv1 = LinkedVector::new();
lv1.push_back(1);
lv1.push_back(2);
lv1.push_back(3);
lv1.iter_mut().zip(7..).for_each(|(a, b)| *a = b);
lv1.iter().zip(7..).for_each(|(a, b)| assert_eq!(a, &b));

for (v1, v2) in (10..).zip(&mut lv1) {
    *v2 = v1;
}
lv1.iter().zip(10..).for_each(|(a, b)| assert_eq!(a, &b));
```
