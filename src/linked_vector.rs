

use core::fmt;
use core::iter::{FromIterator, FusedIterator};
use core::ops::{Index, IndexMut};
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::fmt::Formatter;
use core::fmt::Debug;

use crate::cursor::*;

#[cfg(debug_assertions)]
use uuid::{uuid, Uuid};

#[cfg(not(debug_assertions))]
pub(crate) const BAD_HANDLE : HNode = HNode(usize::MAX);

#[cfg(debug_assertions)]
pub(crate) const BAD_HANDLE : HNode = 
                    HNode(usize::MAX, 
                        usize::MAX, 
                        uuid!("deadbeef-dead-beef-dead-beefdeadbeef"));

/// A handle to a node within a `LinkedVector`. Internally, it holds an index
/// into the vector holding the LinkedVector's nodes.
/// 
#[cfg(not(debug_assertions))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HNode(usize);

/// A handle to a node within a `LinkedVector`. Internally, it holds an index
/// into the vector holding the LinkedVector's nodes.
/// 
#[cfg(debug_assertions)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HNode(usize, usize, Uuid);

impl Default for HNode {
    #[inline]
    fn default() -> Self {
        BAD_HANDLE
    }
}

/// The node type used by `LinkedVector`. It holds a value of type `T`, and 
/// handles to the next and previous nodes in the list.
/// 
pub(crate) struct Node<T> {
    value : Option<T>,
    next  : HNode,
    prev  : HNode,

    // This field is used to detect expired handles. In debug mode if
    // a handle's 2nd field doesn't match this, it's expried. When
    // a node is added to the recycle list via. `push_recyc()`, this 
    // number is incremented.
    #[cfg(debug_assertions)]
    gen   : usize,
}
impl<T> Node<T> {
    #[cfg(debug_assertions)]
    #[inline]
    fn new(value: T, gen: usize) -> Self {
        Self { 
            value : Some(value), 
            next  : BAD_HANDLE, 
            prev  : BAD_HANDLE, 
            gen,
        }
    }
    #[cfg(not(debug_assertions))]
    #[inline]
    fn new(value: T) -> Self {
        Self { 
            value : Some(value), 
            next  : BAD_HANDLE, 
            prev  : BAD_HANDLE, 
        }
    }

    #[cfg(test)]
    #[inline(always)]
    pub(crate) fn next(&self) -> HNode {
        self.next
    }

    #[cfg(test)]
    #[inline(always)]
    pub(crate) fn prev(&self) -> HNode {
        self.prev
    }
}

/// A doubly-linked list that uses handles to refer to elements that exist
/// within a vector. This allows for O(1) insertion and removal of elements
/// from the list, and O(1) access to elements by handle.
/// 
pub struct LinkedVector<T> {
    vec   : Vec<Node<T>>,
    head  : HNode,
    recyc : HNode,
    len   : usize,

    // This field is used to detect foreign handles. If a handle's
    // 3rd field doesn't match this, it's foreign.
    #[cfg(debug_assertions)]
    uuid  : Uuid,
}

impl<T> LinkedVector<T> {
    /// Creates a new, empty `LinkedVector`.
    /// 
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self { 
            vec   : Vec::new(), 
            recyc : BAD_HANDLE, 
            head  : BAD_HANDLE, 
            len   : 0, 

            #[cfg(debug_assertions)]
            uuid  : uuid::Uuid::new_v4() 
        }
    }

    /// Creates a new, empty `LinkedVector` with the specified capacity.
    /// 
    #[inline]
    #[must_use]
    pub fn with_capacity(size: usize) -> Self {
        Self { 
            vec   : Vec::with_capacity(size), 
            recyc : BAD_HANDLE, 
            head  : BAD_HANDLE, 
            len   : 0, 

            #[cfg(debug_assertions)]
            uuid  : uuid::Uuid::new_v4() 
        }
    }

    /// Moves all elements from `other` into `self`, leaving `other` empty.
    /// This operation completes in O(n) time where n is the length of `other`.
    /// ```
    /// use linked_vector::*;
    /// let mut lv1 = LinkedVector::new();
    /// let mut lv2 = LinkedVector::from([1, 2, 3]);
    /// 
    /// lv1.append(&mut lv2);
    /// 
    /// assert_eq!(lv1.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    /// assert_eq!(lv2.len(), 0);
    /// ```
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        while let Some(value) = other.pop_front() {
            self.push_back(value);
        }
        other.clear();
    }

    /// Gives a reference to the back element, or `None` if the list is  empty.
    ///  This operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// assert_eq!(lv.back(), Some(&3));
    /// ```
    #[inline]
    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.back_().unwrap().value.as_ref()
        }
    }

    /// Gives a mutable reference to the element back element, or `None` if the
    /// list is empty. This operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// 
    /// *lv.back_mut().unwrap() = 42;
    /// 
    /// assert_eq!(lv.back_mut(), Some(&mut 42));
    /// ```
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.get_mut_(self.get_(self.head).prev).value.as_mut()
        }
    }

    /// Returns the total number of elements the vector can hold without 
    /// reallocating.
    /// 
    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    /// Removes all elements from the list.
    /// 
    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear();
        self.len = 0;
        self.head = BAD_HANDLE;
        self.recyc = BAD_HANDLE;
    }
    
    /// Consumes the LinkedVector and produces a new one that has all its nodes 
    /// placed contiguously in sequential order at the front of the internal 
    /// vector. Where performance is critical and the cost of a compacting 
    /// operation is infrequent and acceptible, compacting the vector *may* give
    /// a gain in performance for certain use cases. All handles from the old 
    /// vector will not be native to the new compacted vector. `compact()` 
    /// completes in O(n) time.
    /// 
    #[inline]
    #[must_use]
    pub fn compact(self) -> Self {
        self.into_iter().collect()
    }

    /// Returns `true` if the list contains an element with the given value.
    /// This operation completes in O(n) time where n is the length of the list.
    /// 
    #[inline]
    pub fn contains(&self, value: &T) -> bool 
    where 
        T: PartialEq
    {
        self.iter().any(|v| v == value)
    }

    /// Creates a cursor that can be used to traverse the list starting at the
    /// given node. This operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// let h3 = lv.handle(2).unwrap();
    /// let mut cursor = lv.cursor(h3);
    /// 
    /// cursor.forward(3);
    /// 
    /// assert_eq!(*cursor, 6);
    /// ```
    #[inline]
    pub fn cursor(&self, node: HNode) -> Cursor<T> {
        Cursor::new(self, node)
    }

    /// Creates a cursor that holds a mutable reference to the LinkedVector that
    /// can be used to traverse the list starting at the given node. If the 
    /// vector is empty, `None` is returned. This operation completes in O(1)
    /// time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3, 4, 5, 6]);
    /// let mut cursor = lv.cursor_mut(lv.front_node().unwrap());
    /// 
    /// cursor.forward(3);
    /// 
    /// #[cfg(not(feature = "optionless-accessors"))]
    /// {
    ///     assert_eq!(cursor.get(), Some(&4));
    /// 
    ///     *cursor.get_mut().unwrap() = 42;
    /// 
    ///     assert_eq!(lv.to_vec(), vec![1, 2, 3, 42, 5, 6]);
    /// }
    /// #[cfg(feature = "optionless-accessors")]
    /// {
    ///     assert_eq!(cursor.get(), &4);
    /// 
    ///     *cursor.get_mut() = 42;
    /// 
    ///     assert_eq!(lv.to_vec(), vec![1, 2, 3, 42, 5, 6]);
    /// }
    /// ```
    #[inline]
    pub fn cursor_mut(&mut self, node: HNode) -> CursorMut<T> {
        CursorMut::new(self, node)
    }

    /// Returns a Cursor starting at the front element, or `None` if the list is
    /// empty. If the vector is empty, `None` is returned. This operation
    /// completes in O(1) time.
    /// 
    #[inline]
    pub fn cursor_back(&self) -> Option<Cursor<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self.cursor(self.back_node().unwrap()))
        }
    }

    /// Returns a Cursor starting at the back element, or `None` if the list is
    /// empty. This operation completes in O(1) time.
    /// 
    #[inline]
    pub fn cursor_back_mut(&mut self) -> Option<CursorMut<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self.cursor_mut(self.back_node().unwrap()))
        }
    }

    /// Gives a reference to the element at the front of the vector, or `None` 
    /// if the list is empty. This operation completes in O(1) time.
    /// 
    #[inline]
    pub fn cursor_front(&self) -> Option<Cursor<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self.cursor(self.front_node().unwrap()))
        }
    }

    /// Gives a mutable Cursor starting at the front of the vector, or `None` if
    /// the list is empty. This operation completes in O(1) time.
    /// 
    #[inline]
    pub fn cursor_front_mut(&mut self) -> Option<CursorMut<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self.cursor_mut(self.front_node().unwrap()))
        }
    }

    /// Gives a reference to the element at the front of the vector, or `None` 
    /// if the list is empty. This operation completes in O(1) time.
    /// 
    #[inline]
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.front_().unwrap().value.as_ref()
        }
    }

    /// Gives a mutable reference to the element at the front of the vector,
    ///  or `None` if the list is empty. This operation completes in O(1) time.
    /// 
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.get_mut_(self.head).value.as_mut()
        }
    }

    /// Returns a handle to the first node in the list, or `None` if the list is
    /// empty. This operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// let hnode = lv.push_front(42);
    /// 
    /// #[cfg(not(feature = "optionless-accessors"))]
    /// {
    ///     assert_eq!(lv.front_node(), Some(hnode));
    ///     assert_eq!(lv.front_node().map(|h| lv.get(h)).unwrap(), Some(&42));
    /// }
    /// #[cfg(feature = "optionless-accessors")]
    /// {
    ///     assert_eq!(lv.front_node(), Some(hnode));
    ///     assert_eq!(lv.front_node().map(|h| lv.get(h)), Some(&42));
    /// }
    /// ```
    #[inline]
    pub fn front_node(&self) -> Option<HNode> {
        if self.len == 0 {
            None
        } else {
            Some(self.head)
        }
    }

    /// Returns a handle to the last node in the list, or `None` if the list is
    /// empty. This operation completes in O(1) time.
    /// 
    #[inline]
    pub fn back_node(&self) -> Option<HNode> {
        self.front_().map(|node| node.prev)
    }

    /// Provides a reference to the element indicated by the given handle. This
    /// operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// let hnode = lv.push_front(42);
    /// 
    /// assert_eq!(lv.get(hnode), &42);
    /// ```
    #[inline]
    #[cfg(feature = "optionless-accessors")]
    pub fn get(&self, node: HNode) -> &T {
        self.get_(node).value.as_ref().unwrap()
    }

    /// Provides a reference to the element indicated by the given handle, or
    /// `None` if the handle is invalid. This operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// let hnode = lv.push_front(42);
    /// 
    /// assert_eq!(lv.get(hnode), Some(&42));
    /// ```    
    #[inline]
    #[cfg(not(feature = "optionless-accessors"))]
    pub fn get(&self, node: HNode) -> Option<&T> {
        self.get_(node).value.as_ref()
    }

    /// Provides a mutable reference to the element indicated by the given
    /// handle. This operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// let hnode = lv.push_front(0);
    /// 
    /// *lv.get_mut(hnode) = 42;
    /// 
    /// assert_eq!(lv.get(hnode), &42);
    /// ```
    #[inline]
    #[cfg(feature = "optionless-accessors")]
    pub fn get_mut(&mut self, node: HNode) -> &mut T {
        self.get_mut_(node).value.as_mut().unwrap()
    }

    /// Provides a mutable reference to the element indicated by the given
    /// handle, or `None` if the handle is invalid. This operation completes in
    /// O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// let hnode = lv.push_front(0);
    /// 
    /// *lv.get_mut(hnode).unwrap() = 42;
    /// 
    /// assert_eq!(lv[hnode], 42);
    /// ```
    #[inline]
    #[cfg(not(feature = "optionless-accessors"))]
    pub fn get_mut(&mut self, node: HNode) -> Option<&mut T> {
        self.get_mut_(node).value.as_mut()
    }

    /// Returns the handle to the node at the given index, or `None` if the
    /// index is out of bounds. If `index > self.len / 2`, the search starts
    /// from the end of the list. This operation performs in O(n / 2) time
    /// worst case.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// let h1 = lv.push_front(1);
    /// let h2 = lv.push_front(2);
    /// let h3 = lv.push_front(3);
    /// 
    /// assert_eq!(lv.handle(1), Some(h2));
    /// assert_eq!(lv.handle(3), None);
    /// assert_eq!(lv.handle(2), Some(h1));
    /// ```
    #[inline]
    pub fn handle(&self, index: usize) -> Option<HNode> {
        if index <= self.len / 2 {
            self.handles().nth(index)
        } else if index >= self.len {
            None
        } else {
            self.handles().rev().nth(self.len - index - 1)
        }
    }

    /// Returns an iterator over the handles of the vector. The handles will 
    /// reflect the order of the linked list. This operation completes in O(1) 
    /// time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// 
    /// let h1 = lv.push_back(42);
    /// let h2 = lv.push_back(43);
    /// let h3 = lv.push_back(44);
    /// 
    /// let iter = lv.handles();
    /// 
    /// assert_eq!(iter.collect::<Vec<_>>(), vec![h1, h2, h3]);
    /// ```
    #[inline]
    pub fn handles(&self) -> Handles<T> {
        Handles::new(self)
    }

    /// Inserts a new element at the position indicated by the handle, `node`.
    /// Returns a handle to the newly inserted element. This operation completes
    /// in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// 
    /// let h1 = lv.push_back(42);
    /// let h2 = lv.insert(h1, 43);
    /// 
    /// assert_eq!(lv.next_node(h2), Some(h1));
    /// assert_eq!(lv[h1], 42);
    /// ```
    #[inline]
    pub fn insert(&mut self, node: HNode, value: T) -> HNode {
        self.insert_(Some(node), value)
    }

    /// Inserts a new element after the one indicated by the handle, `node`.
    /// Returns a handle to the newly inserted element. This operation completes
    /// in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// 
    /// let h1 = lv.push_back(42);
    /// let h2 = lv.insert_after(h1, 43);
    /// 
    /// assert_eq!(lv.next_node(h1), Some(h2));
    /// assert_eq!(lv[h2], 43);
    /// ```
    #[inline]
    pub fn insert_after(&mut self, node: HNode, value: T) -> HNode {
        if let Some(next) = self.next_node(node) {
            self.insert_(Some(next), value)
        } else {
            self.insert_(None, value)
        }
    }

    /// Returns `true` if the list contains no elements.
    /// 
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns an iterator over the elements of the list.
    /// 
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    /// Returns an iterator over the elements of the list. Renders mutable
    /// references to the elements.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// 
    /// lv.iter_mut().for_each(|x| *x += 1);
    /// 
    /// assert_eq!(lv, LinkedVector::from([2, 3, 4]));
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }

    /// Returns the length of the list.
    /// 
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns a handle to the next node in the list, or `None` if the given
    /// handle is the last node in the list. This operation completes in O(1)
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// 
    /// let h1 = lv.push_back(42);
    /// let h2 = lv.push_back(43);
    /// 
    /// assert_eq!(lv.next_node(h1), Some(h2));
    /// ```
    #[inline]
    pub fn next_node(&self, node: HNode) -> Option<HNode> {
        let next = self.get_(node).next;
        if next == BAD_HANDLE {
            None
        } else {
            Some(next)
        }
    }    

    /// Returns a reference to the next element's value in the list, or `None` 
    /// if the given handle is the last node in the list. This operation 
    /// completes in O(1) time.
    /// 
    #[inline]
    pub fn next_value(&self, node: HNode) -> Option<&T> {
        self.next_node(node).and_then(|n| self.get_(n).value.as_ref())
    }

    /// Returns a mutable reference to the next element's value in the list, or
    /// `None` if the given handle is the last node in the list. This operation
    /// completes in O(1) time.
    /// 
    #[inline]
    pub fn next_value_mut(&mut self, node: HNode) -> Option<&mut T> {
        self.next_node(node).and_then(move |n| self.get_mut_(n).value.as_mut())
    }

    /// Returns a handle to the previous node in the list, or `None` if the 
    /// given handle is the first node in the list. This operation completes in
    /// O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// 
    /// let h1 = lv.push_back(42);
    /// let h2 = lv.push_back(43);
    /// 
    /// assert_eq!(lv.prev_node(h2), Some(h1));
    /// ```
    #[inline]
    pub fn prev_node(&self, node: HNode) -> Option<HNode> {
        if node != self.head {
            Some(self.get_(node).prev)
        } else {
            None
        }
    }

    /// Returns a reference to the previous element's value in the list, or
    /// `None` if the given handle is the first node in the list. This operation
    /// completes in O(1) time.
    /// 
    #[inline]
    pub fn prev_value(&self, node: HNode) -> Option<&T> {
        self.prev_node(node).and_then(|n| self.get_(n).value.as_ref())
    }

    /// Returns a mutable reference to the previous element's value in the list,
    /// or `None` if the given handle is the first node in the list. This
    /// operation completes in O(1) time.
    /// 
    #[inline]
    pub fn prev_value_mut(&mut self, node: HNode) -> Option<&mut T> {
        self.prev_node(node).and_then(move |n| self.get_mut_(n).value.as_mut())
    }

    /// Pops the last element of the vector. Returns `None` if the vector is
    /// empty. This operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// 
    /// assert_eq!(lv.pop_back(), Some(3));
    /// ```
    #[inline]
    pub fn pop_back(&mut self) -> Option<T> {
        self.remove_(None)
    }

    /// Pops the first element of the vector. Returns `None` if the vector is
    /// empty. This operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// 
    /// assert_eq!(lv.pop_front(), Some(1));
    /// ```
    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.remove_(Some(self.head))
        }
    }

    /// Pushes a new element to the back of the list. Returns a handle to the
    /// newly inserted element. This operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// 
    /// let h1 = lv.push_back(42);
    /// let h2 = lv.push_back(43);
    /// 
    /// assert_eq!(lv.next_node(h1), Some(h2));
    /// ```
    #[inline]
    pub fn push_back(&mut self, value: T) -> HNode {
        self.insert_(None, value)
    }

    /// Pushes a new element to the front of the list. Returns a handle to the
    /// newly inserted element. This operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// 
    /// let h1 = lv.front_node().unwrap();
    /// let h2 = lv.push_front(42);
    /// 
    /// assert_eq!(lv.next_node(h2), Some(h1));
    /// ```
    #[inline]
    pub fn push_front(&mut self, value: T) -> HNode {
        if self.is_empty() {
            self.insert_(None, value)
        } else {
            self.insert_(Some(self.head), value)
        }
    }

    /// Removes the element indicated by the handle, `node`. Returns the element
    /// if the handle is valid, or panics otherwise. This operation completes in
    /// O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// let handles = lv.handles().collect::<Vec<_>>();
    /// 
    /// lv.remove(handles[1]);
    /// 
    /// assert_eq!(lv, LinkedVector::from([1, 3]));
    /// ```
    #[inline]
    #[cfg(feature = "optionless-accessors")]
    pub fn remove(&mut self, node: HNode) -> T {
        self.remove_(Some(node)).unwrap()
    }

    /// Removes the element indicated by the handle, `node`. Returns the element
    /// if the handle is valid, or `None` otherwise. This operation completes in
    /// O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// let handles = lv.handles().collect::<Vec<_>>();
    /// 
    /// lv.remove(handles[1]);
    /// 
    /// assert_eq!(lv, LinkedVector::from([1, 3]));
    /// ```    
    #[inline]
    #[cfg(not(feature = "optionless-accessors"))]
    pub fn remove(&mut self, node: HNode) -> Option<T> {
        self.remove_(Some(node))
    }

    /// Sorts the elemements in place in ascending order. Previously held 
    /// handles will still be valid and reference the same elements (with the 
    /// same values) as before.  Only the `next` and `prev` fields of the nodes 
    /// are modified in the list. Uses Rust's stable sort internally and
    /// requires some auxiliary memory for a temporary handle list.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// let h1 = lv.push_back(3);
    /// let h2 = lv.push_back(2);
    /// let h3 = lv.push_back(1);
    /// 
    /// lv.extend([7, 11, 4, 6, 8, 13, 12, 9, 14, 5, 10]);
    /// 
    /// lv.sort();
    /// 
    /// assert_eq!(lv.to_vec(), (1..15).collect::<Vec<_>>());
    /// assert_eq!(lv[h1], 3);
    /// assert_eq!(lv[h2], 2);
    /// assert_eq!(lv[h3], 1);
    /// ```
    #[inline]
    pub fn sort(&mut self) 
    where
        T: Ord
    {
        self.sort_by_(|a, b| a.cmp(b), true);
    }

    /// Sorts the elemements in place using the provided comparison function.
    /// See [sort()](LinkedVector::sort) for more details.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3, 4, 5]);
    /// 
    /// lv.sort_by(|a, b| b.cmp(a));
    /// 
    /// assert_eq!(lv.to_vec(), vec![5, 4, 3, 2, 1]);
    /// ```
    #[inline]
    pub fn sort_by<F>(&mut self, compare: F) 
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.sort_by_(compare, true)
    }

    /// Sorts the elemements in place in using the provided key extraction
    /// function. See [sort()](LinkedVector::sort) for more details.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3, 4, 5]);
    /// 
    /// lv.sort_by_key(|k| -k);
    /// 
    /// assert_eq!(lv.to_vec(), vec![5, 4, 3, 2, 1]);
    /// ```
    #[inline]
    pub fn sort_by_key<K, F>(&mut self, mut key: F) 
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        self.sort_by_(|a, b| key(a).cmp(&key(b)), true);
    }

    /// Sorts the elemements in place in ascending order. Previously held
    /// handles will still be valid and reference the same elements (with the
    /// same values) as before.  Only the `next` and `prev` fields of the nodes
    /// are modified in the list. Uses Rust's unstable sort internally and
    /// requires some auxiliary memory for a temporary handle list.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([5, 4, 3, 2, 1, 0]);
    /// 
    /// lv.sort_unstable();
    /// 
    /// assert_eq!(lv.to_vec(), vec![0, 1, 2, 3, 4, 5]);
    /// ```
    #[inline]
    pub fn sort_unstable(&mut self) 
    where
        T: Ord
    {
        self.sort_by_(|a, b| a.cmp(b), false);
    }

    /// Sorts the elemements in place using the provided comparison function.
    /// See [sort_unstable()](LinkedVector::sort_unstable) for more details.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3, 4, 5]);
    /// 
    /// lv.sort_unstable_by(|a, b| b.cmp(a));
    /// 
    /// assert_eq!(lv.to_vec(), vec![5, 4, 3, 2, 1]);
    /// ```
    #[inline]
    pub fn sort_unstable_by<F>(&mut self , compare: F) 
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.sort_by_(compare, false);
    }

    /// Sorts the elemements in place in using the provided key extraction
    /// function. See [sort_unstable()](LinkedVector::sort_unstable) for more
    /// details.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3, 4, 5]);
    /// 
    /// lv.sort_unstable_by_key(|k| -k);
    /// 
    /// assert_eq!(lv.to_vec(), vec![5, 4, 3, 2, 1]);
    /// ```
    #[inline]
    pub fn sort_unstable_by_key<K, F>(&mut self, mut key: F) 
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        self.sort_by_(|a, b| key(a).cmp(&key(b)), false);
    }

    /// Returns a vector containing the elements of the list. This operation
    /// completes in O(n) time.
    /// ```
    /// use linked_vector::*;
    /// let lv = LinkedVector::from([1, 2, 3]);
    /// 
    /// assert_eq!(lv.to_vec(), vec![1, 2, 3]);
    /// ```
    #[inline]
    pub fn to_vec(&self) -> Vec<T> 
    where
        T: Clone
    {
        self.iter().cloned().collect()
    }

    /// Returns a reference to the last node. Returns `None` if the list is
    /// empty. This operation completes in O(1) time.
    /// 
    #[inline]
    fn back_(&self) -> Option<&Node<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self.get_(self.get_(self.head).prev))
        }
    }

    /// returns a reference to the first node. Returns `None` if the list is
    /// empty. This operation completes in O(1) time.
    /// 
    #[inline]
    fn front_(&self) -> Option<&Node<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self.get_(self.head))
        }
    }

    /// Inserts `value` before the element indicated by `node`. If `node` is
    /// `None`, the element is inserted at the end of the list. Returns a handle
    /// to the newly inserted element. This operation completes in O(1) time.
    /// 
    #[inline]
    pub(crate) fn insert_(&mut self, node: Option<HNode>, value: T) -> HNode {
        if self.is_empty() {
            #[cfg(debug_assertions)]
            assert!(node.is_none(), "Empty list has no handles.");
            let hnew = self.new_node(value);
            self.head = hnew; 
            self.get_mut_(hnew).prev = hnew;
            self.len += 1;
            hnew 
        } else {
            let hnew = self.new_node(value);
            if let Some(hnode) = node {
                let hprev = self.get_(hnode).prev;
                self.get_mut_(hnew).prev = hprev;
                self.get_mut_(hnew).next = hnode;
                self.get_mut_(hnode).prev = hnew;
                if hnode == self.head {
                    self.head = hnew;
                } else {
                    self.get_mut_(hprev).next = hnew;
                }
            } else {
                let hnode = self.get_(self.head).prev;
                self.get_mut_(hnode).next = hnew;
                self.get_mut_(hnew).prev  = hnode;
                self.get_mut_(self.head).prev = hnew;
            }
            self.len += 1;
            hnew
        }
    }

    /// Removes the element indicated by the handle, `node`. Returns the element
    /// if the handle is valid, or `None` otherwise. This operation completes in
    /// O(1) time.
    /// 
    #[inline]
    pub(crate) fn remove_(&mut self, node: Option<HNode>) -> Option<T> {
        if self.is_empty() {
            #[cfg(debug_assertions)]
            assert!(node.is_none(), "Empty list has no handles.");
            None
        } else {
            let hnode = node.unwrap_or(self.get_(self.head).prev);
            if self.len > 1 {
                let hprev = self.get_(hnode).prev;
                let hnext = self.get_(hnode).next;
                if hnext == BAD_HANDLE {
                    self.get_mut_(self.head).prev = hprev;
                } else {
                    self.get_mut_(hnext).prev = hprev;
                }
                if hnode == self.head {
                    self.head = hnext;
                } else {
                    self.get_mut_(hprev).next = hnext;
                }
            } else {
                self.head = BAD_HANDLE;
            }
            self.len -= 1;
            let value = self.get_mut_(hnode).value.take();
            self.push_recyc(hnode);
            value
        }
    }

    /// Returns a reference to the element indicated by the handle, `node`. This
    /// operation completes in O(1) time.
    /// 
    #[inline(always)]
    pub(crate) fn get_(&self, node: HNode) -> &Node<T> {
        #[cfg(debug_assertions)]
        self.check_handle(node);
        
        &self.vec[node.0]
    }

    /// Returns a mutable reference to the element indicated by the handle,
    /// `node`. This operation completes in O(1) time.
    /// 
    #[inline(always)]
    pub(crate) fn get_mut_(&mut self, node: HNode) -> &mut Node<T> {
        #[cfg(debug_assertions)]
        self.check_handle(node);

        &mut self.vec[node.0]
    }

    #[cfg(debug_assertions)]
    pub(crate) fn check_handle(&self, node: HNode) {
        assert!(node.0 != BAD_HANDLE.0, "Handle is invalid.");
        assert!(node.2 == self.uuid, "Handle is not native."); 
        assert!(node.1 == self.vec[node.0].gen, "Handle has expired.");
    }

    /// Renders a new element node and returns a handle to it. This operation
    /// completes in O(1) time.
    /// 
    #[inline]
    fn new_node(&mut self, value: T) -> HNode {
        if let Some(hnode) = self.pop_recyc() {
            #[cfg(debug_assertions)]
            {
                let gen = self.vec[hnode.0].gen;
                self.vec[hnode.0] = Node::new(value, gen);
                let mut hnode = hnode;
                hnode.1 = gen;
                hnode 
            }
            #[cfg(not(debug_assertions))]
            { 
                self.vec[hnode.0] = Node::new(value);
                hnode
            }
        } else {
            #[cfg(debug_assertions)]
            { 
                self.vec.push(Node::new(value, 0));
                HNode(self.vec.len() - 1, 0, self.uuid) 
            }
            #[cfg(not(debug_assertions))]
            { 
                self.vec.push(Node::new(value));
                HNode(self.vec.len() - 1) 
            }
        }
    }

    /// Internal method that returns a handle to a useable node from the recycle
    /// bin. The node is removed from the bin. Only new_node() should call this.
    /// Use new_node() if you need a new node instead of this.
    /// 
    #[inline]
    fn pop_recyc(&mut self) -> Option<HNode> {
        if self.recyc == BAD_HANDLE {
            None
        } else {
            let hnode = self.recyc;
            self.recyc = self.vec[hnode.0].next;
            self.vec[hnode.0].next = BAD_HANDLE;
            Some(hnode) 
        }
    }

    /// Pushes a recently discarded node, indicated by the handle,  back into 
    /// the recycle bin. This can be called by any method that discards a node.
    /// 
    #[inline]
    fn push_recyc(&mut self, node: HNode) {
        self.get_mut_(node).prev = BAD_HANDLE;
        if self.recyc == BAD_HANDLE {
            self.vec[node.0].next = BAD_HANDLE;
            self.recyc = node;
        } else {
            self.vec[node.0].next = self.recyc;
            self.recyc = node;
        }
        #[cfg(debug_assertions)]
        { self.vec[node.0].gen += 1; }
    }

    /// Sorts the list by the given comparison function. This operation 
    /// completes in O(2n + n log n) time.
    /// 
    fn sort_by_<F>(&mut self, mut compare: F, stable: bool) 
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        if self.len < 2 { return; }
        let mut handles = self.handles().collect::<Vec<_>>();
        if stable {
            handles.sort_by(|h1, h2| {
                compare(self.vec[h1.0].value.as_ref().unwrap(), 
                        self.vec[h2.0].value.as_ref().unwrap())
            });
        } else {
            handles.sort_unstable_by(|h1, h2| {
                compare(self.vec[h1.0].value.as_ref().unwrap(), 
                        self.vec[h2.0].value.as_ref().unwrap())
            });
        }
        for i in 0..self.len - 1 {
            self.vec[handles[i].0].next = handles[i + 1];
            self.vec[handles[i + 1].0].prev = handles[i];
        }
        let tail = *handles.last().unwrap();
        self.head = handles[0];
        self.vec[self.head.0].prev = tail;
        self.vec[tail.0].next = BAD_HANDLE;
    }
}

impl<T> Clone for LinkedVector<T> 
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        let mut lv = Self::new();
        for v in self.iter() {
            lv.push_back(v.clone());
        }
        lv
    }
}

impl<T> Debug for LinkedVector<T> 
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "LinkedVector(")?;
        f.debug_list().entries(self.iter()).finish()?;
        write!(f, ")")?;
        Ok(())
    }
}

impl<T> Default for LinkedVector<T> {
    /// Renders the default value for an HNode. This will internally be set
    /// to `BAD_HANDLE` which is a handle that is invalid.
    /// 
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq> Eq for LinkedVector<T> {}

impl<'a, T> Extend<&'a T> for LinkedVector<T> 
where
    T: Clone,
{   
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        for v in iter {
            self.push_back(v.clone());
        }
    }
}

impl<T> Extend<T> for LinkedVector<T> {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for v in iter {
            self.push_back(v);
        }
    }
}

impl<T, const N: usize> From<[T; N]> for LinkedVector<T> {
    #[inline]
    fn from(arr: [T; N]) -> Self {
        let mut lv = Self::new();
        for v in arr {
            lv.push_back(v);
        }
        lv
    }
}

impl<T> FromIterator<T> for LinkedVector<T> {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut lv = Self::new();
        for v in iter {
            lv.push_back(v);
        }
        lv
    }
}

impl<T> Hash for LinkedVector<T> 
where
    T: Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for v in self.iter() {
            v.hash(state);
        }
    }
}

impl<T> Index<HNode> for LinkedVector<T> {
    type Output = T;

    #[inline]
    fn index(&self, handle: HNode) -> &Self::Output {
        #[cfg(feature = "optionless-accessors")]
        { self.get(handle) }

        #[cfg(not(feature = "optionless-accessors"))]
        { self.get(handle).unwrap() }
    }
}

impl<T> IndexMut<HNode> for LinkedVector<T> {
    #[inline]
    fn index_mut(&mut self, handle: HNode) -> &mut Self::Output {
        #[cfg(feature = "optionless-accessors")]
        { self.get_mut(handle) }

        #[cfg(not(feature = "optionless-accessors"))]
        { self.get_mut(handle).unwrap() }
    }
}

impl<T> Index<usize> for LinkedVector<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.handle(index)
            .and_then(|h| self.vec[h.0].value.as_ref())
            .expect("Invalid index")
    }
}

impl<T> IndexMut<usize> for LinkedVector<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.handle(index)
            .and_then(|h| self.vec[h.0].value.as_mut())
            .expect("Invalid index")
    }
}

impl<T> PartialEq for LinkedVector<T> 
where 
    T: PartialEq
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

/// An iterator over the elements of a `LinkedVector`. Yields the handles of
/// each element.
/// 
pub struct Handles<'a, T> {
    lv    : &'a LinkedVector<T>,
    hnode : HNode,
    hrev  : HNode,
    len   : usize,
}

impl<'a, T> Handles<'a, T> {
    #[inline]
    pub fn new(lv: &'a LinkedVector<T>) -> Self {
        Self {
            hnode : lv.head,
            hrev  : lv.front_().map(|h| h.prev).unwrap_or(BAD_HANDLE),
            len   : lv.len(),
            lv,
        }
    }
}

impl<'a, T> Iterator for Handles<'a, T> {
    type Item = HNode;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let hnode = self.hnode;
            self.hnode = self.lv.get_(hnode).next;
            self.len -= 1;
            Some(hnode)
        } else {
            None
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.lv.front_().map(|h| h.prev)
    }
}

impl<'a, T> DoubleEndedIterator for Handles<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let hrev = self.hrev;
            let node = self.lv.get_(hrev);
            self.hrev = node.prev;
            self.len -= 1;
            Some(hrev)
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for Handles<'_, T> {}

impl<T> FusedIterator for Handles<'_, T> {}

/// The basic iterator class of `LinkedVector`. Yields references to the 
/// elements of the vector.
/// 
pub struct Iter<'a, T> {
    lv    : &'a LinkedVector<T>,
    hnode : HNode,
    hrev  : HNode,
    len   : usize,
}
impl<'a, T> Iter<'a, T> {
    #[inline]
    pub fn new(lv: &'a LinkedVector<T>) -> Self {
        Self {
            hnode : lv.head,
            hrev  : lv.front_().map(|h| h.prev).unwrap_or(BAD_HANDLE),
            len   : lv.len(),
            lv,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let hnode = self.hnode;
            self.hnode = self.lv.get_(hnode).next;
            self.len -= 1;
            #[cfg(feature = "optionless-accessors")]
            { Some(self.lv.get(hnode)) }

            #[cfg(not(feature = "optionless-accessors"))]
            { self.lv.get(hnode) }
        } else {
            None
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.lv.back()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let hrev = self.hrev;
            let node = self.lv.get_(hrev);
            self.hrev = node.prev;
            self.len -= 1;
            #[cfg(feature = "optionless-accessors")]
            { Some(self.lv.get(hrev)) }

            #[cfg(not(feature = "optionless-accessors"))]
            { self.lv.get(hrev) }
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

impl<T> FusedIterator for Iter<'_, T> {}

impl<'a, T> IntoIterator for &'a LinkedVector<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            hnode : self.head,
            hrev  : self.front_().map(|h| h.prev).unwrap_or(BAD_HANDLE),
            len   : self.len(),
            lv    : self,
        }
    }
}

/// The basic iterator class of `LinkedVector`. Yields mutable references to
/// the elements of the vector.
/// 
pub struct IterMut<'a, T> {
    lv    : &'a mut LinkedVector<T>,
    hnode : HNode,
    hrev  : HNode,
    len   : usize,
}

impl<'a, T> IterMut<'a, T> {
    #[inline]
    pub fn new(lv: &'a mut LinkedVector<T>) -> Self {
        Self {
            hnode : lv.head,
            hrev  : lv.front_().map(|h| h.prev).unwrap_or(BAD_HANDLE),
            len   : lv.len(),
            lv,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let hnode = self.hnode;
            self.hnode = self.lv.get_(hnode).next;
            self.len -= 1;
            #[cfg(feature = "optionless-accessors")]
            { Some(unsafe { &mut *(self.lv.get_mut(hnode) as *mut T) }) }

            #[cfg(not(feature = "optionless-accessors"))]
            { 
            Some(unsafe { &mut *(self.lv.get_mut(hnode).unwrap() as *mut T)}) 
            }
        } else {
            None
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
    #[inline]
    fn last(self) -> Option<Self::Item> {
        self.lv.back_mut()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let hrev = self.hrev;
            let node = self.lv.get_(hrev);
            self.hrev = node.prev;
            self.len -= 1;
            #[cfg(feature = "optionless-accessors")]
            { Some(unsafe { &mut *(self.lv.get_mut(hrev) as *mut T) }) }

            // TODO - compare this version with 1.1 version.
            #[cfg(not(feature = "optionless-accessors"))]
            {
            Some(unsafe { &mut *(self.lv.get_mut(hrev).unwrap() as *mut T) })
            }
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {}

impl<T> FusedIterator for IterMut<'_, T> {}

impl<'a, T> IntoIterator for &'a mut LinkedVector<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            hnode : self.head,
            hrev  : self.front_().map(|h| h.prev).unwrap_or(BAD_HANDLE),
            len   : self.len(),
            lv    : self,
        }
    }
}

/// The consuming iterator class of `LinkedVector`. Yields owned elements of the
/// vector.
/// 
pub struct IntoIter<T>(LinkedVector<T>);

impl<T> IntoIterator for LinkedVector<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len(), Some(self.0.len()))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> FusedIterator for IntoIter<T> {}

