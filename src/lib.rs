
#![doc = include_str!("../README.md")]

use core::iter::{FromIterator, FusedIterator};
use core::ops::{Index, IndexMut};
use std::cmp::Ordering;
use std::mem::swap;
pub use cursor::*;

#[cfg(test)]
mod tests;
mod cursor;

#[cfg(debug_assertions)]
use uuid::{uuid, Uuid};

#[cfg(not(debug_assertions))]
const BAD_HANDLE : HNode = HNode(usize::MAX);

#[cfg(debug_assertions)]
const BAD_HANDLE : HNode = 
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
#[derive(Debug)]
struct Node<T> {
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
    #[inline]
    fn new(value: T) -> Self {
        Self { 
            value : Some(value), 
            next  : BAD_HANDLE, 
            prev  : BAD_HANDLE, 

            #[cfg(debug_assertions)]
            gen   : 0,
        }
    }
}

/// A doubly-linked list that uses handles to refer to elements that exist
/// within a vector. This allows for O(1) insertion and removal of elements
/// from the list, and O(1) access to elements by handle.
/// 
#[derive(Debug)]
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

    /// Creates a cursor that can be used to traverse the list.
    /// ```
    /// use linked_vector::*;
    /// let lv = LinkedVector::from([1, 2, 3]);
    /// let mut cursor = lv.cursor();
    /// 
    /// assert_eq!(cursor.get(), Some(&1));
    /// 
    /// cursor.move_next();
    /// 
    /// assert_eq!(cursor.get(), Some(&2));
    /// ```
    pub fn cursor(&self) -> Cursor<T> {
        Cursor::new(self)
    }

    /// Creates a cursor that holds a mutable reference to the LinkedVector that 
    /// can be used to traverse the list.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3, 4, 5, 6]);
    /// let mut cursor = lv.cursor_mut();
    /// 
    /// cursor.forward(3);
    /// 
    /// assert_eq!(cursor.get(), Some(&4));
    /// 
    /// *cursor.get_mut().unwrap() = 42;
    /// 
    /// assert_eq!(lv.to_vec(), vec![1, 2, 3, 42, 5, 6]);
    /// ```
    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut::new(self)
    }

    /// Creates a cursor that can be used to traverse the list starting at the
    /// given node. This operation completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// let h  = lv.find_node(&3).unwrap();
    /// let mut cursor = lv.cursor_at(h);
    /// 
    /// cursor.forward(3);
    /// 
    /// assert_eq!(cursor.get(), Some(&6));
    /// ```
    pub fn cursor_at(&self, hnode: HNode) -> Cursor<T> {
        Cursor::new_at(self, hnode)
    }

    /// Creates a cursor that holds a mutable reference to the LinkedVector that
    /// can be used to traverse the list starting at the given node.
    /// 
    pub fn cursor_at_mut(&mut self, hnode: HNode) -> CursorMut<T> {
        CursorMut::new_at(self, hnode)
    }

    /// Returns the handle to the first node with the given value. If no such
    /// node exists, `None` is returned. This operation completes in O(n) time.
    /// ```
    /// use linked_vector::*;
    /// let lv = LinkedVector::from([1, 2, 3, 4, 5, 6]);
    /// let h  = lv.find_node(&3).unwrap();
    /// 
    /// assert_eq!(lv.get(h), Some(&3));
    /// assert_eq!(lv.find_node(&42), None);
    /// ```
    #[inline]
    pub fn find_node(&self, value: &T) -> Option<HNode>
    where
        T: PartialEq
    {
        for (h, v) in self.handles().zip(self.iter()) {
            if v == value {
                return Some(h);
            }
        }
        None
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
    /// assert_eq!(lv.front_node(), Some(hnode));
    /// assert_eq!(lv.front_node().and_then(|h| lv.get(h)), Some(&42));
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
    pub fn get(&self, node: HNode) -> Option<&T> {
        self.get_(node).value.as_ref()
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
    /// assert_eq!(lv.get(hnode), Some(&42));
    /// ```
    #[inline]
    pub fn get_mut(&mut self, node: HNode) -> Option<&mut T> {
        self.get_mut_(node).value.as_mut()
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
    /// assert_eq!(lv.get(h2), Some(&43));
    /// ```
    #[inline]
    pub fn insert_after(&mut self, node: HNode, value: T) -> HNode {
        if let Some(next) = self.next_node(node) {
            self.insert_(Some(next), value)
        } else {
            self.insert_(None, value)
        }
    }

    /// Inserts a new element before the one indicated by the handle, `node`.
    /// Returns a handle to the newly inserted element. This operation completes
    /// in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// 
    /// let h1 = lv.push_back(42);
    /// let h2 = lv.insert_before(h1, 43);
    /// 
    /// assert_eq!(lv.next_node(h2), Some(h1));
    /// assert_eq!(lv.get(h1), Some(&42));
    /// ```
    #[inline]
    pub fn insert_before(&mut self, node: HNode, value: T) -> HNode {
        self.insert_(Some(node), value)
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

    /// Removes the first element with the indicated value. Returns the element
    /// if it is found, or `None` otherwise. This operation completes in O(n)
    /// time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// 
    /// assert_eq!(lv.remove(&2), Some(2));
    /// assert_eq!(lv, LinkedVector::from([1, 3]));
    /// ```
    #[inline]
    pub fn remove(&mut self, value: &T) -> Option<T> 
    where 
        T: PartialEq
    {
        for (h, v) in self.handles().zip(self.iter()) {
            if v == value {
                return self.remove_(Some(h));
            }
        }
        None
    }

    /// Removes the element indicated by the handle, `node`. Returns the element
    /// if the handle is valid, or `None` otherwise. This operation completes in
    /// O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3]);
    /// let handles = lv.handles().collect::<Vec<_>>();
    /// 
    /// lv.remove_node(handles[1]);
    /// 
    /// assert_eq!(lv, LinkedVector::from([1, 3]));
    /// ```
    #[inline]
    pub fn remove_node(&mut self, node: HNode) -> Option<T> {
        self.remove_(Some(node))
    }

    /// Sorts the elemements in place in ascending order. Previously held 
    /// handles will still be valid and reference the same elements (with the 
    /// same values) as before.  Quicksort is used with the Lomuto partition 
    /// scheme. Only the `next` and `prev` fields of the nodes are modified in 
    /// the list. This operation completes in `O(n log n)` time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// let h1 = lv.push_back(3);
    /// let h2 = lv.push_back(2);
    /// let h3 = lv.push_back(1);
    /// 
    /// lv.sort_unstable();
    /// 
    /// assert_eq!(lv.to_vec(), vec![1, 2, 3]);
    /// assert_eq!(lv.get(h1), Some(&3));
    /// assert_eq!(lv.get(h2), Some(&2));
    /// assert_eq!(lv.get(h3), Some(&1));
    /// ```
    pub fn sort_unstable(&mut self) 
    where
        T: Ord
    {
        self.sort_unstable_by(|a, b| a.cmp(b));
    }

    /// Sorts the elemements in place in using the provided comparison function.
    /// See [LinkedVector::sort_unstable()] for more details.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3, 4, 5]);
    /// 
    /// lv.sort_unstable_by(|a, b| b.cmp(a));
    /// 
    /// assert_eq!(lv.to_vec(), vec![5, 4, 3, 2, 1]);
    /// ```
    pub fn sort_unstable_by<F>(&mut self, mut compare: F) 
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        use Ordering::Less;
        if self.len < 2 { return; }
        if let (Some(lo), Some(hi)) = (self.front_node(), self.back_node()) {
            let mut stack = vec![(lo, hi)];

            while let Some((mut lo, mut hi)) = stack.pop() {
                let     p = hi;
                let mut i = lo;
                let mut j = lo;

                while j != hi {
                    if compare(self.vec[j.0].value.as_ref().unwrap(), 
                               self.vec[p.0].value.as_ref().unwrap()) == Less {
                        if i != j {
                            if i == lo {
                                self.swap(&mut i, &mut j);
                                lo = i;
                            } else {
                                self.swap(&mut i, &mut j);
                            }
                        }
                        i = self.vec[i.0].next;
                    }
                    j = self.vec[j.0].next;
                }
                if i == lo { 
                    self.swap(&mut i, &mut hi);
                    lo = i;
                } else {
                    self.swap(&mut i, &mut hi);
                }
                if lo != hi {
                    if i != lo {
                        stack.push((lo, self.vec[i.0].prev));
                    }
                    if i != hi {
                        stack.push((self.vec[i.0].next, hi));
                    }
                }
            }
        }
    }

    /// Sorts the elemements in place in using the provided key extraction
    /// function. See [LinkedVector::sort_unstable()] for more details.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::from([1, 2, 3, 4, 5]);
    /// 
    /// lv.sort_unstable_by_key(|k| -k);
    /// 
    /// assert_eq!(lv.to_vec(), vec![5, 4, 3, 2, 1]);
    /// ```
    pub fn sort_unstable_by_key<K, F>(&mut self, mut key: F) 
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        self.sort_unstable_by(|a, b| key(a).cmp(&key(b)));
    }

    /// Swaps the elements indicated by the handles, `hnode1` and `hnode2`. Only
    /// the next and prev fields of nodes are altered. `hnode1` and `hnode2` 
    /// will be updated to reference the swapped values. This operation 
    /// completes in O(1) time.
    /// ```
    /// use linked_vector::*;
    /// let mut lv = LinkedVector::new();
    /// 
    /// let mut h1 = lv.push_back(42);
    /// let mut h2 = lv.push_back(43);
    /// 
    /// let h1_bak = h1;
    /// let h2_bak = h2;
    /// 
    /// lv.swap(&mut h1, &mut h2);
    /// 
    /// assert_eq!(lv[h1], 43);
    /// assert_eq!(lv[h2], 42);
    /// assert_eq!(lv.next_node(h1), Some(h2));
    /// assert_eq!(lv.next_node(h2_bak), Some(h1_bak));
    /// assert_eq!(lv.get(h1_bak), Some(&42));
    /// assert_eq!(lv.get(h2_bak), Some(&43));
    /// ```
    #[inline]
    pub fn swap(&mut self, hnode1: &mut HNode, hnode2: &mut HNode) {
        let h1 = *hnode1;
        let h2 = *hnode2;
        let prev1 = self.get_(h1).prev;
        let next1 = self.get_(h1).next;

        let prev2 = self.get_(h2).prev;
        let next2 = self.get_(h2).next;
        
        self.get_mut_(prev1).next = h2;
        self.get_mut_(prev2).next = h1;

        if next1 != BAD_HANDLE {
            self.get_mut_(next1).prev = h2;
        }
        if next2 != BAD_HANDLE {
            self.get_mut_(next2).prev = h1;
        }
        if prev1 == h2 { self.get_mut_(h2).prev = h1;    }
        else           { self.get_mut_(h2).prev = prev1; }
        if prev2 == h1 { self.get_mut_(h1).prev = h2;    }
        else           { self.get_mut_(h1).prev = prev2; }
        if next1 == h2 { self.get_mut_(h2).next = h1;    }
        else           { self.get_mut_(h2).next = next1; }
        if next2 == h1 { self.get_mut_(h1).next = h2;    }
        else           { self.get_mut_(h1).next = next2; }

        if      self.head == h1 { self.head = h2; } 
        else if self.head == h2 { self.head = h1; }

        if      next1 == BAD_HANDLE { self.get_mut_(self.head).prev = h2; }
        else if next2 == BAD_HANDLE { self.get_mut_(self.head).prev = h1; }

        swap(hnode1, hnode2);
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
    fn insert_(&mut self, node: Option<HNode>, value: T) -> HNode {
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
    fn remove_(&mut self, node: Option<HNode>) -> Option<T> {
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
    fn get_(&self, node: HNode) -> &Node<T> {
        #[cfg(debug_assertions)]
        { assert!(node.0 != BAD_HANDLE.0, "Handle is invalid.");
          assert!(node.2 == self.uuid, "Handle is not native.");
          assert!(node.1 == self.vec[node.0].gen, "Handle is expired."); }
        &self.vec[node.0]
    }

    /// Returns a mutable reference to the element indicated by the handle,
    /// `node`. This operation completes in O(1) time.
    /// 
    #[inline(always)]
    fn get_mut_(&mut self, node: HNode) -> &mut Node<T> {
        #[cfg(debug_assertions)]
        { assert!(node.0 != BAD_HANDLE.0, "Handle is invalid.");
          assert!(node.2 == self.uuid, "Handle is not native."); 
          assert!(node.1 == self.vec[node.0].gen, "Handle is expired."); }
        &mut self.vec[node.0]
    }

    /// Renders a new element node and returns a handle to it. This operation
    /// completes in O(1) time.
    /// 
    #[inline]
    fn new_node(&mut self, value: T) -> HNode {
        if let Some(hnode) = self.pop_recyc() {
            self.vec[hnode.0] = Node::new(value);
            #[cfg(debug_assertions)]
            {
                let mut hnode = hnode;
                hnode.1 = self.vec[hnode.0].gen;
                hnode 
            }
            #[cfg(not(debug_assertions))]
            { hnode }
        } else {
            self.vec.push(Node::new(value));
            #[cfg(debug_assertions)]
            { HNode(self.vec.len() - 1, 0, self.uuid) }
            #[cfg(not(debug_assertions))]
            { HNode(self.vec.len() - 1) }
        }
    }

    /// Internal method that returns a handle to a useable node from the recycle
    /// bin. The node is removed from the bin.
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
    /// the recycle bin.
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

impl<T> Default for LinkedVector<T> {
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

impl<T> Index<HNode> for LinkedVector<T> {
    type Output = T;

    #[inline]
    fn index(&self, handle: HNode) -> &Self::Output {
        self.get(handle).expect("Invalid handle")
    }
}

impl<T> IndexMut<HNode> for LinkedVector<T> {
    #[inline]
    fn index_mut(&mut self, handle: HNode) -> &mut Self::Output {
        self.get_mut(handle).expect("Invalid handle")
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
            self.lv.get(hnode)
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
            self.lv.get(hrev)
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
            self.lv.get_mut(hnode).map(|p| unsafe { &mut *(p as *mut T) })
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
            self.lv.get_mut(hrev).map(|p| unsafe { &mut *(p as *mut T) })
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

