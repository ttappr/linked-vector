#![allow(unused_variables)]

use core::cmp::Reverse;
use std::collections::HashMap;

use super::*;

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn expired_handles_1() {
    let mut lv = LinkedVector::new();
    let h1 = lv.push_back(1);
    let h2 = lv.push_back(2);
    let h3 = lv.push_back(3);

    lv.remove(h2);

    lv.push_back(4); // This will recycle node pointed to by h2.

    lv.get(h2);
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn expired_handles_2() {
    let mut lv = LinkedVector::new();
    let h1 = lv.push_back(1);
    let h2 = lv.push_back(2);
    let h3 = lv.push_back(3);

    lv.remove(h2);

    lv.get(h2);
}

#[test]
#[cfg(debug_assertions)]
fn expired_handles_3() {
    let mut lv = LinkedVector::new();
    let h1 = lv.push_back(1);
    let h2 = lv.push_back(2);
    let h3 = lv.push_back(3);

    lv.remove(h2);

    let h4 = lv.push_back(4); // This will recycle node pointed to by h2.

    lv.get(h4); // The new handle should work.

    lv.remove(h4);

    let h5 = lv.push_back(9);

    lv.get(h5); // Same node recycled twice now. h5 should work as it has
                // generation == 2 same as the node.

    lv.get(h1); // These should still be good.
    lv.get(h3);
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn foreign_handles() {
    let mut lv1 = LinkedVector::new();
    let     lv2 = LinkedVector::from([1, 2, 3]);
    let h1 = lv1.push_back(1);
    let h2 = lv1.push_back(2);
    let h3 = lv1.push_back(3);

    lv2.get(h1); // h1 belongs to lv1.
}

#[test] 
fn append() {
    let mut lv1 = LinkedVector::from([1, 2, 3]);
    let mut lv2 = LinkedVector::from([4, 5, 6]);

    lv1.append(&mut lv2);

    assert_eq!(lv1.to_vec(), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn back() {
    let mut lv1 = LinkedVector::new();
    lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    assert_eq!(lv1.back(), Some(&3));
    assert_eq!(lv1.len(), 3);
}

#[test]
fn back_mut() {
    let mut lv1 = LinkedVector::new();
    lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    *lv1.back_mut().unwrap() = 4;
    assert_eq!(lv1.back(), Some(&4));
    assert_eq!(lv1.len(), 3);
}

#[test]
fn clear() {
    let mut lv1 = LinkedVector::new();
    lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    lv1.clear();
    assert_eq!(lv1.is_empty(), true);
    assert_eq!(lv1.len(), 0);
}

#[test]
fn clone() {
    let mut lv1 = LinkedVector::new();
    lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    let lv2 = lv1.clone();
    assert_eq!(lv1, lv2); // Also tests PartialEq.
    assert_eq!(lv1.len(), lv2.len());
}

#[test]
fn compact() {
    let lv1 = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let lv2 = lv1.compact();
    assert_eq!(lv2.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
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
fn cursor() {
    let lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7]);
    let mut cursor = lv.cursor(lv.front_node().unwrap());
    
    assert_eq!(cursor.get(), Some(&1));
    
    cursor.move_next();
    
    assert_eq!(cursor.get(), Some(&2));
    
    let hend = cursor.move_to_end().unwrap();
    let hbak = cursor.backward(2).unwrap();
    
    assert_eq!(cursor.get(), Some(&5));
    assert_eq!(lv.get(hend), Some(&7));
    assert_eq!(lv.get(hbak), Some(&5));
    
    let mut cursor = lv.cursor(hbak);
    
    match cursor.backward(20) {
        Ok(handle) => panic!("Should move to beginning on overshoot."),
        Err(handle) => assert_eq!(lv.get(handle), Some(&1)),
    }

    match cursor.forward(20) {
        Ok(handle) => panic!("Should move to end on overshoot."),
        Err(handle) => assert_eq!(lv.get(handle), Some(&7)),
    }
}

#[test]
fn cursor_2() {
    let lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let h5 = lv.handle(4).unwrap();
    let mut cursor = lv.cursor(h5);

    assert_eq!(cursor.get(), Some(&5));
    
    let h6 = cursor.move_next().unwrap();

    assert_eq!(lv[h6], 6);

    cursor.forward(2).expect("Should move forward 2.");
    assert_eq!(cursor.get(), Some(&8));
}

#[test]
fn cursor_insert() {
    let mut lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let h5 = lv.handle(4).unwrap();
    let mut cursor = lv.cursor_mut(h5);

    cursor.insert(10);
    assert_eq!(lv.len(), 10);
    assert_eq!(lv.to_vec(), vec![1, 2, 3, 4, 10, 5, 6, 7, 8, 9]);
}

#[test]
fn cursor_remove() {
    let mut lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let h5 = lv.handle(4).unwrap();
    
    let mut cursor = lv.cursor_mut(h5);

    cursor.remove();
    assert_eq!(cursor.get(), Some(&6));

    cursor.move_to_end().unwrap();
    cursor.remove();
    assert_eq!(cursor.get(), Some(&8));
    assert_eq!(lv.to_vec(), vec![1, 2, 3, 4, 6, 7, 8]);
}

#[test]
fn debug() {
    let lv1 = LinkedVector::from([1, 2, 3]);
    assert_eq!(format!("{:?}", lv1), "LinkedVector([1, 2, 3])");

    let lv2 = LinkedVector::from(["foo", "bar", "baz"]);
    assert_eq!(format!("{:?}", lv2), 
                       "LinkedVector([\"foo\", \"bar\", \"baz\"])");
}

#[test]
fn default() {
    let lv1 = LinkedVector::<i32>::default();
    assert_eq!(lv1.is_empty(), true);
}

#[test]
fn eq() {
    let mut lv1 = LinkedVector::new();
    let mut lv2 = LinkedVector::new();
    lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    lv2.push_back(1);
    lv2.push_back(2);
    lv2.push_back(3);
    assert_eq!(lv1, lv2);

    lv2.push_back(4);

    assert_ne!(lv1, lv2);
}

#[test]
fn extend() {
    let mut lv1 = LinkedVector::from([1, 2, 3]);
    let     lv2 = LinkedVector::from([4, 5, 6]);

    lv1.extend(lv2.iter());

    assert_eq!(lv1.to_vec(), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(lv2.len(), 3);
    assert_eq!(lv1.len(), 6);
}

#[test]
fn test_drop() {
    let mut lv1 = LinkedVector::new();
    lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    drop(lv1); // Should not panic.
}

#[test]
fn from_array() {
    let lv1 = LinkedVector::from([1, 2, 3]);
    lv1.iter().zip(1..).for_each(|(a, b)| assert_eq!(a, &b));
    assert_eq!(lv1.len(), 3);
}

#[test]
fn from_iter() {
    let lv1 = LinkedVector::from_iter(1..4);
    lv1.iter().zip(1..).for_each(|(a, b)| assert_eq!(a, &b));
    assert_eq!(lv1.len(), 3);

    let lv2 : LinkedVector<_> = (1..4).collect();
    lv2.iter().zip(1..).for_each(|(a, b)| assert_eq!(a, &b));
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
    assert_eq!(lv1.len(), 3);
}

#[test]
fn front_node() {
    let mut lv1 = LinkedVector::new();
    let h1 = lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    assert_eq!(lv1.front_node(), Some(h1));
}

#[test]
fn back_node() {
    let mut lv1 = LinkedVector::new();
    lv1.push_back(1);
    lv1.push_back(2);
    let h3 = lv1.push_back(3);
    assert_eq!(lv1.back_node(), Some(h3));
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
    assert_eq!(lv1.len(), 3);
}

#[test]
fn get_mut() {
    let mut lv1 = LinkedVector::new();
    let h1 = lv1.push_back(1);
    let h2 = lv1.push_back(2);
    let h3 = lv1.push_back(3);
    *lv1.get_mut(h1).unwrap() = 4;
    *lv1.get_mut(h2).unwrap() = 5;
    *lv1.get_mut(h3).unwrap() = 6;
    assert_eq!(lv1.get(h1), Some(&4));
    assert_eq!(lv1.get(h2), Some(&5));
    assert_eq!(lv1.get(h3), Some(&6));
    assert_eq!(lv1.len(), 3);
}

#[test]
fn get_handle() {
    let mut lv1 = LinkedVector::new();
    let h1 = lv1.push_back(1);
    let h2 = lv1.push_back(2);
    let h3 = lv1.push_back(3);
    assert_eq!(lv1.handle(0), Some(h1));
    assert_eq!(lv1.handle(1), Some(h2));
    assert_eq!(lv1.handle(2), Some(h3));
    assert_eq!(lv1.handle(3), None);
}

#[test]
fn handles() {
    let mut lv1 = LinkedVector::new();
    let h1 = lv1.push_back(1);
    let h2 = lv1.push_back(2);
    let h3 = lv1.push_back(3);
    
    let mut it = lv1.handles();
    
    assert_eq!(it.len(), 3);
    assert_eq!(it.next(), Some(h1));
    assert_eq!(it.len(), 2);
    assert_eq!(it.next(), Some(h2));
    assert_eq!(it.next(), Some(h3));

    let mut it = lv1.handles().rev();
    
    assert_eq!(it.next(), Some(h3));
    assert_eq!(it.next(), Some(h2));
    assert_eq!(it.next(), Some(h1));
    assert!(it.next().is_none());
}

#[test]
fn hashing() {
    let mut map = HashMap::new();
    let     lv1 = LinkedVector::from([1, 2, 3]);
    let     lv2 = LinkedVector::from([3, 4, 5]);

    map.insert(lv1.clone(), 1);
    map.insert(lv2.clone(), 2);

    assert_eq!(map.get(&lv1), Some(&1));
    assert_eq!(map.get(&lv2), Some(&2));
}

#[test]
fn indexing() {
    let mut lv1 = LinkedVector::new();
    let h1 = lv1.push_back(1);
    let h2 = lv1.push_back(2);
    let h3 = lv1.push_back(3);
    assert_eq!(lv1[h1], 1);
    assert_eq!(lv1[h2], 2);
    assert_eq!(lv1[h3], 3);
}

#[test]
fn indexing_mut() {
    let mut lv1 = LinkedVector::new();
    let h1 = lv1.push_back(1);
    let h2 = lv1.push_back(2);
    let h3 = lv1.push_back(3);
    lv1[h1] = 4;
    lv1[h2] = 5;
    lv1[h3] = 6;
    assert_eq!(lv1[h1], 4);
    assert_eq!(lv1[h2], 5);
    assert_eq!(lv1[h3], 6);
    assert_eq!(lv1.len(), 3);
}

#[test]
fn index_usize() {
    let mut lv1 = LinkedVector::new();
    let h1 = lv1.push_back(1);
    let h2 = lv1.push_back(2);
    let h3 = lv1.push_back(3);
    assert_eq!(lv1[0], 1);
    assert_eq!(lv1[1], 2);
    assert_eq!(lv1[2], 3);
}

#[test]
fn index_mut_usize() {
    let mut lv1 = LinkedVector::new();
    let h1 = lv1.push_back(1);
    let h2 = lv1.push_back(2);
    let h3 = lv1.push_back(3);
    lv1[0] = 4;
    lv1[1] = 5;
    lv1[2] = 6;
    assert_eq!(lv1[0], 4);
    assert_eq!(lv1[1], 5);
    assert_eq!(lv1[2], 6);
    assert_eq!(lv1.len(), 3);
}

#[test]
fn  insert_() {
    let mut lv1 = LinkedVector::new();

    let h1 = lv1.insert_(None, 1);
    let h2 = lv1.insert_(Some(h1), 2);

    assert_eq!(lv1.front(), Some(&2));
    assert_eq!(lv1.back(), Some(&1));

    let h3 = lv1.insert_(None, 3);

    assert_eq!(lv1.back(), Some(&3));
    assert_eq!(lv1.front(), Some(&2));
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
    assert_eq!(lv1.len(), 4);
}

#[test]
fn insert_before() {
    let mut lv1 = LinkedVector::new();
    let h1 = lv1.push_back(1);
    let h2 = lv1.push_back(2);
    let h3 = lv1.push_back(3);
    let h4 = lv1.insert(h3, 4);
    assert_eq!(lv1.front(), Some(&1));
    assert_eq!(lv1.back(), Some(&3));
    assert_eq!(lv1.get_(h4).next, h3);
    assert_eq!(lv1.len(), 4);
}

#[test]
fn test_insertions_deletions_etc() {
    let mut lv1 = LinkedVector::new();
    let mut hs  = vec![];

    for i in 0..100 {
        hs.push(lv1.push_back(i));
    }

    for &h in hs.iter().step_by(2) {
        lv1.remove(h);
    }

    for (&h1, h2) in hs.iter().skip(1).step_by(2).zip(lv1.handles()) {
        assert_eq!(h1, h2);
    }
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
fn into_iter_rev() {
    let mut lv1 = LinkedVector::new();

    lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);

    let lv2 = lv1.clone();

    let mut it = lv1.into_iter();

    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));

    let mut it = lv2.into_iter().rev();

    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(1));
    assert!(it.next().is_none());
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
fn iter_rev() {
    let mut lv1 = LinkedVector::new();
    lv1.push_back(3);
    lv1.push_back(2);
    lv1.push_back(1);
    lv1.iter().rev().zip(1..).for_each(|(a, b)| assert_eq!(a, &b));

    for (v1, v2) in (1..).zip(lv1.iter().rev()) {
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
fn iter_mut_rev() {
    let mut lv1 = LinkedVector::new();
    lv1.push_back(3);
    lv1.push_back(2);
    lv1.push_back(1);
    
    lv1.iter_mut().rev().zip(7..).for_each(|(a, b)| *a = b);

    let mut it = lv1.iter();

    assert_eq!(it.next(), Some(&9));
    assert_eq!(it.next(), Some(&8));
    assert_eq!(it.next(), Some(&7));

    for (v1, v2) in (10..).zip(lv1.iter_mut().rev()) {
        *v2 = v1;
    }
    lv1.iter().zip((10..=12).rev()).for_each(|(a, b)| assert_eq!(a, &b));
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
    assert_eq!(lv1.len(), 0);
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
    assert_eq!(lv1.len(), 0);
}

#[test]
fn push_back() {
    let mut lv1 = LinkedVector::new();
    lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    assert_eq!(lv1.front(), Some(&1));
    assert_eq!(lv1.back(), Some(&3));
    assert_eq!(lv1.len(), 3);
}

#[test]
fn push_front() {
    let mut lv1 = LinkedVector::new();
    lv1.push_front(1);
    lv1.push_front(2);
    lv1.push_front(3);
    assert_eq!(lv1.front(), Some(&3));
    assert_eq!(lv1.back(), Some(&1));
    assert_eq!(lv1.len(), 3);
}

#[test]
fn remove() {
    let mut lv1 = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let h = lv1.handle(6).unwrap();
    assert_eq!(lv1.remove(h), Some(7));
    assert_eq!(lv1.to_vec(), vec![1, 2, 3, 4, 5, 6, 8, 9]);
}

#[test]
fn sort() {
    let mut lv1 = LinkedVector::from([2, 1, 6, 7, 4, 8, 5, 3]);
    lv1.sort();
    assert_eq!(lv1.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8]);

    lv1 = LinkedVector::from([3, 1, 4, 1, 5, 9]);
    lv1.sort();
    assert_eq!(lv1.to_vec(), vec![1, 1, 3, 4, 5, 9]);

    lv1 = LinkedVector::from([104, 188, 5, 44, 199, 139, 31, 54, 30, 43, 151, 
                              70, 68, 131, 132, 116, 26, 177, 35, 141, 22, 150, 
                              29, 122, 145, 72, 106, 51, 125, 160, 1, 98, 119, 
                              107, 181, 123, 128, 23, 147, 191, 153, 162, 172, 
                              37, 161, 126, 66, 149, 38, 165, 189, 94, 33, 41, 
                              103, 71, 176, 18, 166, 196, 195, 42, 194, 156, 7, 
                              154, 140, 190, 34, 36, 79, 46, 64, 3, 76, 118, 
                              109, 92, 175, 60, 129, 120, 75, 105, 136, 2, 173, 
                              61, 56, 19, 82, 48, 62, 27, 12, 77, 93, 87, 21, 
                              99, 163, 45, 47, 138, 0, 108, 57, 65, 146, 17, 
                              86, 10, 53, 117, 134, 39, 96, 90, 127, 14, 185, 
                              157, 192, 169, 159, 74, 197, 183, 59, 130, 67, 
                              58, 95, 49, 148, 78, 184, 111, 155, 198, 167, 
                              102, 15, 114, 69, 52, 158, 143, 88, 137, 178, 
                              182, 100, 85, 144, 124, 81, 11, 170, 50, 4, 16, 
                              73, 89, 97, 25, 9, 84, 55, 180, 193, 135, 164, 
                              113, 110, 186, 80, 28, 13, 101, 115, 6, 179, 171, 
                              20, 174, 168, 112, 121, 91, 32, 24, 8, 40, 133, 
                              152, 142, 83, 187, 63]);
    lv1.sort();
    lv1.into_iter().zip(0..).for_each(|(a, b)| assert_eq!(a, b));

    lv1 = LinkedVector::from([5, 2]);
    lv1.sort();
    assert_eq!(lv1.to_vec(), vec![2, 5]);
}

#[test]
fn sort_by() {
    let mut lv1 = LinkedVector::from([2, 1, 6, 7, 4, 8, 5, 3]);
    lv1.sort_by(|a, b| b.cmp(a));
    assert_eq!(lv1.to_vec(), vec![8, 7, 6, 5, 4, 3, 2, 1]);

    lv1 = LinkedVector::from([3, 1, 4, 1, 5, 9]);
    lv1.sort_by(|a, b| b.cmp(a));
    assert_eq!(lv1.to_vec(), vec![9, 5, 4, 3, 1, 1]);

    lv1 = LinkedVector::from([104, 188, 5, 44, 199, 139, 31, 54, 30, 43, 151, 
                              70, 68, 131, 132, 116, 26, 177, 35, 141, 22, 150, 
                              29, 122, 145, 72, 106, 51, 125, 160, 1, 98, 119, 
                              107, 181, 123, 128, 23, 147, 191, 153, 162, 172, 
                              37, 161, 126, 66, 149, 38, 165, 189, 94, 33, 41, 
                              103, 71, 176, 18, 166, 196, 195, 42, 194, 156, 7, 
                              154, 140, 190, 34, 36, 79, 46, 64, 3, 76, 118, 
                              109, 92, 175, 60, 129, 120, 75, 105, 136, 2, 173, 
                              61, 56, 19, 82, 48, 62, 27, 12, 77, 93, 87, 21, 
                              99, 163, 45, 47, 138, 0, 108, 57, 65, 146, 17, 
                              86, 10, 53, 117, 134, 39, 96, 90, 127, 14, 185, 
                              157, 192, 169, 159, 74, 197, 183, 59, 130, 67, 
                              58, 95, 49, 148, 78, 184, 111, 155, 198, 167, 
                              102, 15, 114, 69, 52, 158, 143, 88, 137, 178, 
                              182, 100, 85, 144, 124, 81, 11, 170, 50, 4, 16, 
                              73, 89, 97, 25, 9, 84, 55, 180, 193, 135, 164, 
                              113, 110, 186, 80, 28, 13, 101, 115, 6, 179, 171, 
                              20, 174, 168, 112, 121, 91, 32, 24, 8, 40, 133, 
                              152, 142, 83, 187, 63]);
    lv1.sort_by(|a, b| b.cmp(a));
    lv1.into_iter().zip((0..200).rev()).for_each(|(a, b)| assert_eq!(a, b));
}

#[test]
fn sort_unstable() {
    let mut lv = LinkedVector::from([2, 1, 6, 7, 4, 8, 5, 3]);
    lv.sort_unstable();
    assert_eq!(lv.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8]);

    lv = LinkedVector::from([2, 1, 6, 7, 4, 8, 5, 3]);
    lv.sort_unstable_by(|a, b| b.cmp(a));
    assert_eq!(lv.to_vec(), vec![8, 7, 6, 5, 4, 3, 2, 1]);

    lv = LinkedVector::from([2, 1, 6, 7, 4, 8, 5, 3]);
    lv.sort_unstable_by_key(|a| Reverse(*a));
    assert_eq!(lv.to_vec(), vec![8, 7, 6, 5, 4, 3, 2, 1]);
}

#[test]
fn sort_by_key() {
    let mut lv1 = LinkedVector::from([2, 1, 6, 7, 4, 8, 5, 3]);
    lv1.sort_by_key(|a| Reverse(*a));
    assert_eq!(lv1.to_vec(), vec![8, 7, 6, 5, 4, 3, 2, 1]);    
}

#[test]
fn to_vec() {
    let mut lv1 = LinkedVector::new();
    lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    assert_eq!(lv1.to_vec(), vec![1, 2, 3]);
}

#[test]
fn with_capacity() {
    let mut lv1 = LinkedVector::with_capacity(10);
    assert_eq!(lv1.capacity(), 10);
    lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    assert_eq!(lv1.capacity(), 10);
    assert_eq!(lv1.len(), 3);
}