
use crate::*;

/// A cursor is a position within a linked vector. It can be used to traverse
/// the list in either direction, and to access the element at the current
/// position.
/// 
pub trait CursorBase<T> {
    /// Returns a reference to the element at the cursor's current position.
    /// 
    fn get(&self) -> Option<&T>;

    /// Moves the cursor to the specified handle. Returns true if the cursor
    /// was moved, false if the handle was invalid.
    /// 
    fn move_to(&mut self, handle: HNode) -> bool;

    /// Moves the cursor to the next element. Returns the handle of the next
    /// element if the cursor was moved, None if the cursor was already at the
    /// end of the list.
    /// 
    fn move_next(&mut self) -> Option<HNode>;

    /// Moves the cursor to the previous element. Returns the handle of the
    /// previous element if the cursor was moved, None if the cursor was
    /// already at the start of the list.
    /// 
    fn move_prev(&mut self) -> Option<HNode>;

    /// Moves the cursor to the start of the list. Returns the handle of the
    /// first element if the cursor was moved, None if the list is empty.
    /// 
    fn move_to_start(&mut self) -> Option<HNode>;

    /// Moves the cursor to the end of the list. Returns the handle of the
    /// last element if the cursor was moved, None if the list is empty.
    /// 
    fn move_to_end(&mut self) -> Option<HNode>;

    /// Moves the cursor forward by the specified number of elements. Returns
    /// the handle of the element at the new position if the cursor was moved,
    /// Err(handle) if the cursor was already at the end of the list.
    /// 
    fn forward(&mut self, n: usize) -> Result<HNode, HNode>;

    /// Moves the cursor backward by the specified number of elements. Returns
    /// the handle of the element at the new position if the cursor was moved,
    /// Err(handle) if the cursor was already at the start of the list.
    /// 
    fn backward(&mut self, n: usize) -> Result<HNode, HNode>;
}

/// A cursor which can only read the elements of the list.
/// 
pub struct Cursor<'a, T> {
    lvec   : &'a LinkedVector<T>,
    handle : HNode,
}

impl<'a, T> Cursor<'a, T> {
    pub(crate) fn new(lvec   : &'a LinkedVector<T>, 
                      handle : HNode) 
        -> Self 
    {
        lvec.get(handle).expect("Cursor::new_at() called with invalid handle.");
        Self {
            lvec,
            handle,
        }
    }
}
impl<'a, T> CursorBase<T> for Cursor<'a, T> {
    fn get(&self) -> Option<&T> {
        self.lvec.get(self.handle)
    }

    fn move_to(&mut self, handle: HNode) -> bool {
        if self.lvec.get(handle).is_some() {
            self.handle = handle;
            true
        } else {
            false
        }
    }

    fn move_next(&mut self) -> Option<HNode> {
        self.lvec.next_node(self.handle).map(|hnext| {
            self.handle = hnext;
            hnext
        })
    }

    fn move_prev(&mut self) -> Option<HNode> {
        self.lvec.prev_node(self.handle).map(|hprev| {
            self.handle = hprev;
            hprev
        })
    }

    fn move_to_start(&mut self) -> Option<HNode> {
        self.lvec.front_node().map(|hstart| {
            self.handle = hstart;
            hstart
        })
    }

    fn move_to_end(&mut self) -> Option<HNode> {
        self.lvec.back_node().map(|hend| {
            self.handle = hend;
            hend
        })
    }
    fn forward(&mut self, n: usize) ->Result<HNode, HNode> {
        for _ in 0..n {
            if self.move_next().is_none() {
                return Err(self.handle);
            }
        }
        Ok(self.handle)
    }
    fn backward(&mut self, n: usize) -> Result<HNode, HNode> {
        for _ in 0..n {
            if self.move_prev().is_none() {
                return Err(self.handle);
            }
        }
        Ok(self.handle)
    }
}

/// A cursor which can read and write the elements of the list.
/// 
pub struct CursorMut<'a, T> {
    lvec   : &'a mut LinkedVector<T>,
    handle : HNode,
}

impl<'a, T> CursorMut<'a, T> {

    pub(crate) fn new(lvec   : &'a mut LinkedVector<T>, 
                      handle : HNode) 
        -> Self 
    {
        lvec.get(handle)
            .expect("CursorMut::new_at() called with invalid handle.");
        Self {
            lvec,
            handle,
        }
    }
    /// Returns a mutable reference to the element at the cursor's current
    /// position.
    /// 
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.lvec.get_mut(self.handle)
    }

    /// Inserts a new element at the cursor's current position. The cursor
    /// will be moved to the new element. Returns the handle of the new
    /// element.
    /// 
    pub fn insert(&mut self, value: T) -> HNode {
        self.handle = self.lvec.insert(self.handle, value);
        self.handle
    }

    /// Inserts a new element after the cursor's current position. The cursor
    /// will still be at the same position. Returns the handle of the new
    /// element.
    /// 
    pub fn insert_after(&mut self, value: T) -> HNode {
        self.lvec.insert_after(self.handle, value)
    }

    /// Removes the element at the current position and returns its value. The 
    /// cursor will be moved to the next element if not at the end of the 
    /// vector, otherwise it moves to the new end. If there was only one item
    /// in the vector, the cursor's position is set to `BAD_HANDLE` and should
    /// no longer be used, or will cause panics.
    /// 
    pub fn remove(&mut self) -> Option<T> {
        let hrem = self.handle;
        if let Some(hnext) = self.lvec.next_node(self.handle) {
            self.handle = hnext;
        } else if let Some(hprev) = self.lvec.prev_node(self.handle) {
            self.handle = hprev;
        } else {
            self.handle = BAD_HANDLE;
        }
        self.lvec.remove(hrem)
    }
}

impl<'a, T> CursorBase<T> for CursorMut<'a, T> {
    fn get(&self) -> Option<&T> {
        self.lvec.get(self.handle)
    }

    fn move_to(&mut self, handle: HNode) -> bool {
        if self.lvec.get(handle).is_some() {
            self.handle = handle;
            true
        } else {
            false
        }
    }

    fn move_next(&mut self) -> Option<HNode> {
        self.lvec.next_node(self.handle).map(|hnext| {
            self.handle = hnext;
            hnext
        })
    }

    fn move_prev(&mut self) -> Option<HNode> {
        self.lvec.prev_node(self.handle).map(|hprev| {
            self.handle = hprev;
            hprev
        })
    }

    fn move_to_start(&mut self) -> Option<HNode> {
        self.lvec.front_node().map(|hstart| {
            self.handle = hstart;
            hstart
        })
    }

    fn move_to_end(&mut self) -> Option<HNode> {
        self.lvec.back_node().map(|hend| {
            self.handle = hend;
            hend
        })
    }

    fn forward(&mut self, n: usize) -> Result<HNode, HNode> {
        for _ in 0..n {
            if self.move_next().is_none() {
                return Err(self.handle);
            }
        }
        Ok(self.handle)
    }

    fn backward(&mut self, n: usize) -> Result<HNode, HNode> {
        for _ in 0..n {
            if self.move_prev().is_none() {
                return Err(self.handle);
            }
        }
        Ok(self.handle)
    }
}