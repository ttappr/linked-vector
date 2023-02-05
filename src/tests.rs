#![allow(unused_variables)]

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
    let mut cursor = lv.cursor(None);
    
    assert_eq!(cursor.get(), Some(&1));
    
    cursor.move_next();
    
    assert_eq!(cursor.get(), Some(&2));
    
    let hend = cursor.move_to_end().unwrap();
    let hbak = cursor.backward(2).unwrap();
    
    assert_eq!(cursor.get(), Some(&5));
    assert_eq!(lv.get(hend), Some(&7));
    assert_eq!(lv.get(hbak), Some(&5));
    
    let mut cursor = lv.cursor(Some(hbak));
    
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
}

#[test]
fn extend() {
    let mut lv1 = LinkedVector::new();
    let mut lv2 = LinkedVector::new();
    for val in [1, 2, 3] {
        lv1.push_back(val);
    }
    for val in [4, 5, 6] {
        lv2.push_back(val);
    }
    lv1.extend(lv2.iter());

    lv1.iter().zip(1..).for_each(|(a, b)| assert_eq!(a, &b));
    assert_eq!(lv2.is_empty(), false);
    assert_eq!(lv1.len(), 6);
}

#[test]
fn test_drop() {
    let mut lv1 = LinkedVector::new();
    lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    drop(lv1);
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
fn index() {
    let mut lv1 = LinkedVector::new();
    let h1 = lv1.push_back(1);
    let h2 = lv1.push_back(2);
    let h3 = lv1.push_back(3);
    assert_eq!(lv1[h1], 1);
    assert_eq!(lv1[h2], 2);
    assert_eq!(lv1[h3], 3);
}

#[test]
fn index_mut() {
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
    let h4 = lv1.insert_before(h3, 4);
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
        lv1.remove_node(h);
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