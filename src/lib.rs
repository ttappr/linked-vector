use core::iter::{FromIterator, FusedIterator};
use core::ops::{Index, IndexMut};
//  use core::mem;

// TODO - Implement: Ord, Hash, Debug prints, Sync, Send...
//      - Needs a README.md file.
//      - Needs a LICENSE file.
//      - Needs a CHANGELOG.md file.
//      - Needs a CONTRIBUTING.md file. (?)
//      - Header in this file describing what this is/does.

#[cfg(test)]
mod tests;

#[cfg(debug_assertions)]
use uuid::{uuid, Uuid};

#[cfg(not(debug_assertions))]
const BAD_HANDLE : HNode = HNode(usize::MAX);

#[cfg(debug_assertions)]
const BAD_HANDLE : HNode = 
        HNode(usize::MAX, uuid!("deadbeef-dead-beef-dead-beefdeadbeef"));

#[cfg(not(debug_assertions))]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HNode(usize);

#[cfg(debug_assertions)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HNode(usize, Uuid);

impl Default for HNode {
    #[inline]
    fn default() -> Self {
        BAD_HANDLE
    }
}

#[derive(Debug)]
struct Node<T> {
    value : Option<T>,
    next  : HNode,
    prev  : HNode,
}
impl<T> Node<T> {
    #[inline]
    fn new(value: T) -> Self {
        Self { 
            value : Some(value), 
            next  : BAD_HANDLE, 
            prev  : BAD_HANDLE, 
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
    bin   : HNode,
    len_  : usize,

    #[cfg(debug_assertions)]
    uuid  : Uuid,
}

impl<T> LinkedVector<T> {
    /// Creates a new, empty `LinkedVector`.
    /// 
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        { Self { 
            vec  : Vec::new(), 
            bin  : BAD_HANDLE, 
            head : BAD_HANDLE, 
            len_ : 0, 
            uuid : uuid::Uuid::new_v4() 
        } }
        #[cfg(not(debug_assertions))]
        { Self { 
            vec  : Vec::new(), 
            bin  : BAD_HANDLE, 
            head : BAD_HANDLE, 
            len_ : 0, 
        } }
    }
    /// Moves all elements from `other` into `self`, leaving `other` empty.
    /// This operation completes in O(n) time where n is the length of `other`.
    /// 
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        while let Some(value) = other.pop_front() {
            self.push_back(value);
        }
        other.clear();
    }
    /// Gives a reference to the element back element, or `None` if the list is 
    /// empty. This operation completes in O(1) time.
    /// 
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
    /// 
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.get_mut_(self.get_(self.head).prev).value.as_mut()
        }
    }

    /// Removes all elements from the list.
    /// 
    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear();
        self.len_ = 0;
        self.head = BAD_HANDLE;
        self.bin  = BAD_HANDLE;
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

    /// Gives a reference to the element front element, or `None` if the list is
    /// empty. This operation completes in O(1) time.
    /// 
    #[inline]
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.front_().unwrap().value.as_ref()
        }
    }

    /// Gives a mutable reference to the element front element, or `None` if the
    /// list is empty. This operation completes in O(1) time.
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
    /// 
    #[inline]
    pub fn front_node(&self) -> Option<HNode> {
        if self.len_ == 0 {
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
    /// 
    #[inline]
    pub fn get(&self, node: HNode) -> Option<&T> {
        #[cfg(debug_assertions)]
        debug_assert!(self.handle_is_native(node), "Alien handle.");
        debug_assert!(self.handle_is_valid(node), "Handle already removed.");
        // TODO - Returning an Option is understood that None is returned on
        //        problem. But the implementation panics on debug builds.
        //        This is inconsistent. Needs fixing.
        self.get_(node).value.as_ref()
    }

    /// Provides a mutable reference to the element indicated by the given
    /// handle, or `None` if the handle is invalid. This operation completes in
    /// O(1) time.
    /// 
    #[inline]
    pub fn get_mut(&mut self, node: HNode) -> Option<&mut T> {
        #[cfg(debug_assertions)]
        debug_assert!(self.handle_is_native(node), "Alien handle.");
        debug_assert!(self.handle_is_valid(node), "Handle already removed.");
        self.get_mut_(node).value.as_mut()
    }

    /// Returns an iterator over the handles of the vector. The handles will 
    /// reflect the order of the linked list. This operation completes in O(1) 
    /// time.
    /// 
    #[inline]
    pub fn handles(&self) -> Handles<T> {
        Handles::new(self)
    }

    /// Inserts a new element after the one indicated by the handle, `node`.
    /// Returns a handle to the newly inserted element. This operation completes
    /// in O(1) time.
    /// 
    #[inline]
    pub fn insert_after(&mut self, node: HNode, value: T) -> HNode {
        self.insert_(Some(self.get_(node).next), value)
    }

    /// Inserts a new element before the one indicated by the handle, `node`.
    /// Returns a handle to the newly inserted element. This operation completes
    /// in O(1) time.
    /// 
    #[inline]
    pub fn insert_before(&mut self, node: HNode, value: T) -> HNode {
        self.insert_(Some(node), value)
    }

    /// Returns `true` if the list contains no elements.
    /// 
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len_ == 0
    }

    /// Returns an iterator over the elements of the list.
    /// 
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    /// Returns an iterator over the elements of the list. Renders mutable
    /// references to the elements.
    /// 
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }

    /// Returns the length of the list.
    /// 
    #[inline]
    pub fn len(&self) -> usize {
        self.len_
    }

    /// Pops the last element of the vector. Returns `None` if the vector is
    /// empty. This operation completes in O(1) time.
    /// 
    #[inline]
    pub fn pop_back(&mut self) -> Option<T> {
        self.remove_(None)
    }

    /// Pops the first element of the vector. Returns `None` if the vector is
    /// empty. This operation completes in O(1) time.
    /// 
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
    /// 
    #[inline]
    pub fn push_back(&mut self, value: T) -> HNode {
        self.insert_(None, value)
    }

    /// Pushes a new element to the front of the list. Returns a handle to the
    /// newly inserted element. This operation completes in O(1) time.
    /// 
    #[inline]
    pub fn push_front(&mut self, value: T) -> HNode {
        if self.is_empty() {
            self.insert_(None, value)
        } else {
            self.insert_(Some(self.head), value)
        }
    }

    /// Removes the element indicated by the handle, `node`. Returns the element
    /// if the handle is valid, or `None` otherwise. This operation completes in
    /// O(1) time.
    /// 
    #[inline]
    pub fn remove(&mut self, node: HNode) -> Option<T> {
        self.remove_(Some(node))
    }

    /// Returns a vector containing the elements of the list. This operation
    /// completes in O(n) time.
    /// 
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

    /// Returns true if the handle within the Option is valid; false otherwise.
    /// 
    #[allow(dead_code)]
    #[inline]
    fn handleopt_is_valid(&self, node: Option<HNode>) -> bool {
        node.map_or(true, |h| self.get_(h).value.is_some())
    }

    /// Returns true if the handle within the Option belongs to this list; false 
    /// otherwise. This check is only performed in debug builds.
    /// 
    #[allow(dead_code)]
    #[cfg(debug_assertions)]
    #[inline]
    fn handleopt_is_native(&self, node: Option<HNode>) -> bool {
        node.map_or(true, |n| n.1 == self.uuid)
    }

    /// Returns true if the handle is valid; false otherwise.
    /// 
    #[inline]
    fn handle_is_valid(&self, node: HNode) -> bool {
        self.get_(node).value.is_some()
    }

    /// Returns true if the handle belongs to this list; false otherwise. This
    /// check is only performed in debug builds.
    /// 
    #[cfg(debug_assertions)]
    #[inline(always)]
    fn handle_is_native(&self, node: HNode) -> bool {
        node.1 == self.uuid
    }

    /// Inserts `value` before the element indicated by `node`. If `node` is
    /// `None`, the element is inserted at the end of the list. Returns a handle
    /// to the newly inserted element. This operation completes in O(1) time.
    /// 
    #[inline]
    fn insert_(&mut self, node: Option<HNode>, value: T) -> HNode {
        if self.is_empty() {
            assert!(node.is_none(), "Empty list has no handles.");
            let hnew = self.new_node(value);
            self.head = hnew; 
            self.get_mut_(hnew).prev = hnew;
            self.len_ += 1;
            hnew 
        } else {
            let hnew = self.new_node(value);
            if let Some(hnode) = node {
                let hprev = self.get_(hnode).prev;
                self.get_mut_(hprev).next = hnew;
                self.get_mut_(hnew).prev = hprev;
                self.get_mut_(hnew).next = hnode;
                if hnode == self.head {
                    self.head = hnew;
                }
            } else {
                let hnode = self.get_(self.head).prev;
                self.get_mut_(hnode).next = hnew;
                self.get_mut_(hnew).prev  = hnode;
                self.get_mut_(self.head).prev = hnew;
            }
            self.len_ += 1;
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
            assert!(node.is_none(), "Empty list has no handles.");
            None
        } else {
            let hnode = node.unwrap_or(self.get_(self.head).prev);
            let hprev = self.get_(hnode).prev;
            let hnext = self.get_(hnode).next;
            self.get_mut_(hprev).next = hnext;
            if hnext == BAD_HANDLE {
                self.get_mut_(self.head).prev = hprev;
            } else {
                self.get_mut_(hnext).prev = hprev;
            }
            if hnode == self.head {
                self.head = hnext;
            }
            self.len_ -= 1;
            self.push_bin(hnode);
            self.get_mut_(hnode).value.take()
        }
    }

    /// Returns a reference to the element indicated by the handle, `node`. This
    /// operation completes in O(1) time.
    /// 
    #[inline(always)]
    fn get_(&self, node: HNode) -> &Node<T> {
        &self.vec[node.0]
    }

    /// Returns a mutable reference to the element indicated by the handle,
    /// `node`. This operation completes in O(1) time.
    /// 
    #[inline(always)]
    fn get_mut_(&mut self, node: HNode) -> &mut Node<T> {
        &mut self.vec[node.0]
    }

    /// Renders a new element node and returns a handle to it. This operation
    /// completes in O(1) time.
    /// 
    #[inline]
    fn new_node(&mut self, value: T) -> HNode {
        if let Some(hnode) = self.pop_bin() {
            self.vec[hnode.0] = Node::new(value);
            hnode
        } else {
            self.vec.push(Node::new(value));
            #[cfg(debug_assertions)]
            { HNode(self.vec.len() - 1, self.uuid) }
            #[cfg(not(debug_assertions))]
            { HNode(self.vec.len() - 1) }
        }
    }

    /// Internal method that returns a handle to a useable node from the recycle
    /// bin. The node is removed from the bin.
    /// 
    #[inline]
    fn pop_bin(&mut self) -> Option<HNode> {
        if self.bin == BAD_HANDLE {
            None
        } else {
            let hnode = self.bin;
            self.bin = self.get_(hnode).next;
            self.get_mut_(hnode).next = BAD_HANDLE;
            Some(hnode)
        }
    }

    /// Pushes a recently discarded node, indicated by the handle,  back into 
    /// the recycle bin.
    /// 
    #[inline]
    fn push_bin(&mut self, node: HNode) {
        self.get_mut_(node).prev = BAD_HANDLE;
        if self.bin == BAD_HANDLE {
            self.get_mut_(node).next = BAD_HANDLE;
            self.bin = node;
        } else {
            self.get_mut_(node).next = self.bin;
            self.bin = node;
        }
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
/*
TODO - Find out if this imple is necessary. I think we can rely on the Vec's
       Drop impl to handle the nodes.
impl<T> Drop for LinkedVector<T> {
// unsafe impl<#[may_dangle] T> Drop for LinkedVector<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut LinkedVector<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                // Continue the same loop we do below. This only runs when a 
                // destructor has panicked. If another one panics this will 
                // abort.
                while self.0.pop_front().is_some() {}
            }
        }

        while let Some(node) = self.pop_front() {
            let guard = DropGuard(self);
            drop(node);
            mem::forget(guard);
        }
    }
}*/

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

