#![allow(unused)]

use std::{marker::PhantomData, iter::FusedIterator};

use uuid::{uuid, Uuid};

fn main() {
    println!("Hello, world!");
}

#[cfg(not(debug_assertions))]
const BAD_HANDLE : HNode = HNode(usize::MAX);

#[cfg(debug_assertions)]
const BAD_HANDLE : HNode = 
        HNode(usize::MAX, uuid!("deadbeef-dead-beef-dead-beefdeadbeef"));

#[cfg(not(debug_assertions))]
#[transparent]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HNode(usize);

#[cfg(debug_assertions)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HNode(usize, Uuid);

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
            self.get_(self.get_(self.head).prev).value.as_ref()
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
            self.get_(self.head).value.as_ref()
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

    /// Returns an iterator over the handles of the vector. This operation
    /// completes in O(1) time.
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

    /// Returns an iterator over the elements of the list.
    /// 
    #[inline]
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
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
            return self.insert_(None, value);
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

    /// Returns true if the handle within the Option is valid; false otherwise.
    /// 
    #[inline]
    fn handleopt_is_valid(&self, node: Option<HNode>) -> bool {
        node.map_or(true, |h| self.get_(h).value.is_some())
    }

    /// Returns true if the handle within the Option belongs to this list; false 
    /// otherwise. This check is only performed in debug builds.
    /// 
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
impl<T> Drop for LinkedVector<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}
*/
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

pub struct Handles<'a, T> {
    lv   : &'a LinkedVector<T>,
    node : HNode,
}

impl<'a, T> Handles<'a, T> {
    #[inline]
    pub fn new(lv: &'a LinkedVector<T>) -> Self {
        Self {
            node : lv.head,
            lv   : lv,
        }
    }
}

impl<'a, T> Iterator for Handles<'a, T> {
    type Item = HNode;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.node == BAD_HANDLE {
            None
        } else {
            let hnode = self.node;
            self.node = self.lv.get_(hnode).next;
            Some(hnode)
        }
    }
}

pub struct Iter<'a, T> {
    lv   : &'a LinkedVector<T>,
    node : HNode,
}
impl<'a, T> Iter<'a, T> {
    #[inline]
    pub fn new(lv: &'a LinkedVector<T>) -> Self {
        Self {
            node : lv.head,
            lv   : lv,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.node == BAD_HANDLE {
            None
        } else {
            let hnode = self.node;
            self.node = self.lv.get_(hnode).next;
            self.lv.get(hnode)
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedVector<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            node : self.head,
            lv   : self,
        }
    }
}

pub struct IterMut<'a, T> {
    lv    : &'a mut LinkedVector<T>,
    hnode : HNode,
}

impl<'a, T> IterMut<'a, T> {
    #[inline]
    pub fn new(lv: &'a mut LinkedVector<T>) -> Self {
        Self {
            hnode : lv.head,
            lv    : lv,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.hnode == BAD_HANDLE {
            None
        } else {
            let hnode  = self.hnode;
            self.hnode = self.lv.get_(self.hnode).next;
            self.lv.get_mut(hnode).map(|p| unsafe { &mut *(p as *mut T) })
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.lv.len();
        (len, Some(len))
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.lv.back_mut()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let prev = self.lv.get_(self.hnode).prev;
        if self.lv.get_(prev).next == BAD_HANDLE {
            None
        } else {
            let hnode  = self.hnode;
            self.hnode = prev;
            self.lv.get_mut(hnode).map(|p| unsafe { &mut *(p as *mut T) })
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
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
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
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() || self.iter().ne(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn append() {
        let mut lv1 = LinkedVector::new();
        let mut lv2 = LinkedVector::new();
        for val in [1, 2, 3] {
            lv1.push_back(val);
        }
        for val in [4, 5, 6] {
            lv2.push_back(val);
        }
        lv1.append(&mut lv2);

        lv1.iter().zip(1..).for_each(|(a, b)| assert_eq!(a, &b));
        assert_eq!(lv2.is_empty(), true);
    }

    #[test]
    fn back() {
        let mut lv1 = LinkedVector::new();
        lv1.push_back(1);
        lv1.push_back(2);
        lv1.push_back(3);
        assert_eq!(lv1.back(), Some(&3));
    }

    #[test]
    fn back_mut() {
        let mut lv1 = LinkedVector::new();
        lv1.push_back(1);
        lv1.push_back(2);
        lv1.push_back(3);
        *lv1.back_mut().unwrap() = 4;
        assert_eq!(lv1.back(), Some(&4));
    }

    #[test]
    fn clear() {
        let mut lv1 = LinkedVector::new();
        lv1.push_back(1);
        lv1.push_back(2);
        lv1.push_back(3);
        lv1.clear();
        assert_eq!(lv1.is_empty(), true);
    }

    #[test]
    fn contains() {
        let mut lv1 = LinkedVector::new();
        lv1.push_back(1);
        lv1.push_back(2);
        lv1.push_back(3);
        assert_eq!(lv1.contains(&2), true);
        assert_eq!(lv1.contains(&4), false);
    }

    #[test]
    fn front() {
        let mut lv1 = LinkedVector::new();
        lv1.push_back(1);
        lv1.push_back(2);
        lv1.push_back(3);
        assert_eq!(lv1.front(), Some(&1));
    }

    #[test]
    fn front_mut() {
        let mut lv1 = LinkedVector::new();
        lv1.push_back(1);
        lv1.push_back(2);
        lv1.push_back(3);
        *lv1.front_mut().unwrap() = 4;
        assert_eq!(lv1.front(), Some(&4));
    }

    #[test]
    fn get() {
        let mut lv1 = LinkedVector::new();
        let h1 = lv1.push_back(1);
        let h2 = lv1.push_back(2);
        let h3 = lv1.push_back(3);
        assert_eq!(lv1.get(h1), Some(&1));
        assert_eq!(lv1.get(h2), Some(&2));
        assert_eq!(lv1.get(h3), Some(&3));
    }

    #[test]
    fn insert_after() {
        let mut lv1 = LinkedVector::new();
        let h1 = lv1.push_back(1);
        let h2 = lv1.push_back(2);
        let h3 = lv1.push_back(3);
        let h4 = lv1.insert_after(h1, 4);
        assert_eq!(lv1.front(), Some(&1));
        assert_eq!(lv1.back(), Some(&3));
        assert_eq!(lv1.get_(h1).next, h4);
        assert_eq!(lv1.get_(h4).next, h2);
        assert_eq!(lv1.get_(h4).prev, h1);
    }

    #[test]
    fn insert_before() {
        let mut lv1 = LinkedVector::new();
        let h1 = lv1.push_back(1);
        let h2 = lv1.push_back(2);
        let h3 = lv1.push_back(3);
        let h4 = lv1.insert_before(h3, 4);
        assert_eq!(lv1.front(), Some(&1));
        assert_eq!(lv1.back(), Some(&3));
        assert_eq!(lv1.get_(h4).next, h3);
    }
    #[test]
    fn into_iter() {
        let mut lv1 = LinkedVector::new();
        lv1.push_back(1);
        lv1.push_back(2);
        lv1.push_back(3);
        lv1.into_iter().zip(1..).for_each(|(a, b)| assert_eq!(a, b));

        let mut lv2 = LinkedVector::new();
        (0..100).for_each(|n| { lv2.push_back(n); });

        assert_eq!(lv2.len(), 100);

        for (v1, v2) in (0..).zip(lv2) {
            assert_eq!(v1, v2);
        }
    }

    #[test]
    fn is_empty() {
        let mut lv1 = LinkedVector::new();
        assert_eq!(lv1.is_empty(), true);
        lv1.push_back(1);
        assert_eq!(lv1.is_empty(), false);
    }

    #[test]
    fn iter() {
        let mut lv1 = LinkedVector::new();
        lv1.push_back(1);
        lv1.push_back(2);
        lv1.push_back(3);
        lv1.iter().zip(1..).for_each(|(a, b)| assert_eq!(a, &b));

        for (v1, v2) in (1..).zip(&lv1) {
            assert_eq!(v1, *v2);
        }
    }

    #[test]
    fn iter_mut() {
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
    }

    #[test]
    fn len() {
        let mut lv1 = LinkedVector::new();
        assert_eq!(lv1.len(), 0);
        lv1.push_back(1);
        assert_eq!(lv1.len(), 1);
        lv1.push_back(2);
        assert_eq!(lv1.len(), 2);
        lv1.push_back(3);
        assert_eq!(lv1.len(), 3);
        lv1.pop_front();
        assert_eq!(lv1.len(), 2);
        lv1.pop_back();
        assert_eq!(lv1.len(), 1);
        lv1.pop_back();
        assert_eq!(lv1.len(), 0);
    }

    #[test]
    fn pop_back() {
        let mut lv1 = LinkedVector::new();
        lv1.push_back(1);
        lv1.push_back(2);
        lv1.push_back(3);
        assert_eq!(lv1.pop_back(), Some(3));
        assert_eq!(lv1.pop_back(), Some(2));
        assert_eq!(lv1.pop_back(), Some(1));
        assert_eq!(lv1.pop_back(), None);
    }

    #[test]
    fn pop_front() {
        let mut lv1 = LinkedVector::new();
        lv1.push_back(1);
        lv1.push_back(2);
        lv1.push_back(3);
        assert_eq!(lv1.pop_front(), Some(1));
        assert_eq!(lv1.pop_front(), Some(2));
        assert_eq!(lv1.pop_front(), Some(3));
        assert_eq!(lv1.pop_front(), None);
    }

    #[test]
    fn push_back() {
        let mut lv1 = LinkedVector::new();
        lv1.push_back(1);
        lv1.push_back(2);
        lv1.push_back(3);
        assert_eq!(lv1.front(), Some(&1));
        assert_eq!(lv1.back(), Some(&3));
    }

    #[test]
    fn push_front() {
        let mut lv1 = LinkedVector::new();
        lv1.push_front(1);
        lv1.push_front(2);
        lv1.push_front(3);
        assert_eq!(lv1.front(), Some(&3));
        assert_eq!(lv1.back(), Some(&1));
    }

    #[test]
    fn  iinsert() {
        let mut lv1 = LinkedVector::new();

        let h1 = lv1.insert_(None, 1);
        let h2 = lv1.insert_(Some(h1), 2);

        assert_eq!(lv1.front(), Some(&2));
        assert_eq!(lv1.back(), Some(&1));

        let h3 = lv1.insert_(None, 3);

        assert_eq!(lv1.back(), Some(&3));
        assert_eq!(lv1.front(), Some(&2));
    }
}