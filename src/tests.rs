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