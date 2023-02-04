# LinkedVector

`LinkedVector` is a hybrid of a vector and linked list. Items are accessible
directly in `O(1)` time, and insertions and deletions also operate in `O(1)`
time. Internally, nodes exist within a contiguous vector, with each node holding 
handles to its previous and next neighbors.

## Usage

```
[dependencies]
linked-vector = "0.1"
```

## Handles

Items in a `LinkedVector` are directly accessible via the `HNode` struct. These
are returned by operations such as insert or push operations. If direct access
is required to any specific items, their handles can be stored for later use.

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

---
## Examples
### Accessing Items Using Handles

### Traversal With Cursors

### Iterators

### Least Frequently Used Cache