
use crate::*;

/// A cursor is a position within a linked vector. It can be used to traverse
/// the list in either direction, and to access the element at the current
/// position.
/// 
pub trait CursorBase<T> {
    /// Returns a reference to the element at the cursor's current position.
    /// 
    fn get(&self) -> Option<&T>;

    /// Returns a mutable reference to the element at the cursor's current
    /// position.
    fn get_mut(&mut self) -> Option<&mut T>;

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
    pub(crate) fn new(lvec: &'a LinkedVector<T>) -> Self {
        let handle = lvec.front_node()
                         .expect("Cursor::new() called on empty LinkedVector.");
        Self {
            lvec,
            handle,
        }
    }
    pub(crate) fn new_at(lvec   : &'a LinkedVector<T>, 
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

    fn get_mut(&mut self) -> Option<&mut T> {
        panic!("CursorBase::get_mut() called on Cursor which has an immutable 
                reference to LinkedVector.")
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
    pub(crate) fn new(lvec: &'a mut LinkedVector<T>) -> Self {
        let handle = lvec.front_node()
                         .expect("CursorMut::new() called on 
                                  empty LinkedVector.");
        Self {
            lvec,
            handle,
        }
    }
    pub(crate) fn new_at(lvec   : &'a mut LinkedVector<T>, 
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
}

impl<'a, T> CursorBase<T> for CursorMut<'a, T> {
    fn get(&self) -> Option<&T> {
        self.lvec.get(self.handle)
    }

    fn get_mut(&mut self) -> Option<&mut T> {
        self.lvec.get_mut(self.handle)
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