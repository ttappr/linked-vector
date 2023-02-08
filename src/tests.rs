#![allow(unused_variables)]

use std::cmp::Reverse;

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
    let h5 = lv.find_node(&5).unwrap();
    let mut cursor = lv.cursor(h5);

    assert_eq!(cursor.get(), Some(&5));
    
    let h6 = cursor.move_next().unwrap();

    assert_eq!(lv[h6], 6);

    cursor.forward(2).expect("Should move forward 2.");
    assert_eq!(cursor.get(), Some(&8));
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
fn find_node() {
    let mut lv1 = LinkedVector::new();
    let _h1 = lv1.push_back(1);
    let  h2 = lv1.push_back(2);
    let _h3 = lv1.push_back(3);
    assert_eq!(lv1.find_node(&2), Some(h2));
    assert_eq!(lv1.find_node(&4), None);
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
fn remove_node() {
    let mut lv1 = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let h = lv1.find_node(&7).unwrap();
    assert_eq!(lv1.remove(h), Some(7));
    assert_eq!(lv1.to_vec(), vec![1, 2, 3, 4, 5, 6, 8, 9]);
}

#[test]
fn sort_unstable() {
    let mut lv1 = LinkedVector::from([2, 1, 6, 7, 4, 8, 5, 3]);
    lv1.sort_unstable();
    assert_eq!(lv1.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8]);

    lv1 = LinkedVector::from([3, 1, 4, 1, 5, 9]);
    lv1.sort_unstable();
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
    lv1.sort_unstable();
    lv1.into_iter().zip(0..).for_each(|(a, b)| assert_eq!(a, b));

    lv1 = LinkedVector::from([5, 2]);
    lv1.sort_unstable();
    assert_eq!(lv1.to_vec(), vec![2, 5]);
}

#[test]
fn sort_unstable_by() {
    let mut lv1 = LinkedVector::from([2, 1, 6, 7, 4, 8, 5, 3]);
    lv1.sort_unstable_by(|a, b| b.cmp(a));
    assert_eq!(lv1.to_vec(), vec![8, 7, 6, 5, 4, 3, 2, 1]);

    lv1 = LinkedVector::from([3, 1, 4, 1, 5, 9]);
    lv1.sort_unstable_by(|a, b| b.cmp(a));
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
    lv1.sort_unstable_by(|a, b| b.cmp(a));
    lv1.into_iter().zip((0..200).rev()).for_each(|(a, b)| assert_eq!(a, b));
}

#[test]
fn sort_unstable_by_key() {
    let mut lv1 = LinkedVector::from([2, 1, 6, 7, 4, 8, 5, 3]);
    lv1.sort_unstable_by_key(|a| Reverse(*a));
    assert_eq!(lv1.to_vec(), vec![8, 7, 6, 5, 4, 3, 2, 1]);    
}

#[test]
fn swap() {
    let mut lv = LinkedVector::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut handles = lv.handles().collect::<Vec<_>>();

    // Swap non terminal nodes.

    let (h0, h8) = handles.split_at_mut(8);

    lv.swap(&mut h0[1], &mut h8[0]);

    assert_eq!(lv.get(handles[1]), Some(&8));
    assert_eq!(lv.get(handles[8]), Some(&1));

    assert_eq!(lv.to_vec(), vec![0, 8, 2, 3, 4, 5, 6, 7, 1, 9]);

    assert_eq!(lv.iter().copied().rev().collect::<Vec<_>>(), 
               vec![9, 1, 7, 6, 5, 4, 3, 2, 8, 0]);

    assert_eq!(lv.next_node(handles[1]), Some(handles[2]));

    // Swap head and tail nodes.

    let (h0, h9) = handles.split_at_mut(9);

    lv.swap(&mut h0[0], &mut h9[0]);

    assert_eq!(lv.get(handles[0]), Some(&9));
    assert_eq!(lv.get(handles[9]), Some(&0));

    assert_eq!(lv.to_vec(), vec![9, 8, 2, 3, 4, 5, 6, 7, 1, 0]);

    assert_eq!(lv.iter().copied().rev().collect::<Vec<_>>(), 
               vec![0, 1, 7, 6, 5, 4, 3, 2, 8, 9]);

    assert_eq!(lv.back(), Some(&0));
    assert_eq!(lv.front(), Some(&9));

    assert_eq!(lv.next_node(handles[0]), Some(handles[1]));

    // Swap middle node with head node.

    lv           = LinkedVector::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    handles      = lv.handles().collect::<Vec<_>>();
    let (h0, h5) = handles.split_at_mut(5);

    lv.swap(&mut h0[0], &mut h5[0]);

    assert_eq!(lv.get(handles[0]), Some(&5));
    assert_eq!(lv.prev_node(handles[0]), None);
    assert_eq!(lv.prev_node(handles[5]), Some(handles[4]));
    assert_eq!(lv.to_vec(), vec![5, 1, 2, 3, 4, 0, 6, 7, 8, 9]);
    assert_eq!(lv.back(), Some(&9));

    // Swap middle node with tail node.

    lv           = LinkedVector::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    handles      = lv.handles().collect::<Vec<_>>();
    let (h0, h9) = handles.split_at_mut(9);

    lv.swap(&mut h0[5], &mut h9[0]);

    assert_eq!(lv.get(handles[9]), Some(&5));
    assert_eq!(lv.get(handles[5]), Some(&9));
    assert_eq!(lv.next_node(handles[5]), Some(handles[6]));
    assert_eq!(lv.next_node(handles[9]), None);

    assert_eq!(lv.to_vec(), vec![0, 1, 2, 3, 4, 9, 6, 7, 8, 5]);
    assert_eq!(lv.front(), Some(&0));
    assert_eq!(lv.back(), Some(&5));
    assert_eq!(lv.iter().rev().copied().collect::<Vec<_>>(), 
               vec![5, 8, 7, 6, 9, 4, 3, 2, 1, 0]);

    // Swap adjacent nodes.

    lv           = LinkedVector::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    handles      = lv.handles().collect::<Vec<_>>();
    let (h0, h5) = handles.split_at_mut(5);

    lv.swap(&mut h0[4], &mut h5[0]);

    assert_eq!(lv.get(handles[4]), Some(&5));
    assert_eq!(lv.get(handles[5]), Some(&4));

    assert_eq!(lv.to_vec(), vec![0, 1, 2, 3, 5, 4, 6, 7, 8, 9]);
    assert_eq!(lv.front(), Some(&0));
    assert_eq!(lv.back(), Some(&9));
    assert_eq!(lv.iter().rev().copied().collect::<Vec<_>>(), 
               vec![9, 8, 7, 6, 4, 5, 3, 2, 1, 0]);
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