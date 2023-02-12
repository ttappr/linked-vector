#![allow(unused_variables)]

use crate::linked_vector::*;
use crate::cursor::*;

#[test]
fn cursor() {
    let lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7]);
    let mut cursor = lv.cursor(lv.front_node().unwrap());
    
    assert_eq!(cursor.get(), &1);
    
    cursor.move_next();
    
    assert_eq!(cursor.get(), &2);
    
    let hend = cursor.move_to_back().unwrap();
    let hbak = cursor.backward(2).unwrap();
    
    assert_eq!(cursor.get(), &5);
    assert_eq!(lv.get(hend), &7);
    assert_eq!(lv.get(hbak), &5);
    
    let mut cursor = lv.cursor(hbak);
    
    match cursor.backward(20) {
        Ok(handle) => panic!("Should move to beginning on overshoot."),
        Err(handle) => assert_eq!(lv.get(handle), &1),
    }

    match cursor.forward(20) {
        Ok(handle) => panic!("Should move to end on overshoot."),
        Err(handle) => assert_eq!(lv.get(handle), &7),
    }
}

#[test]
fn cursor_2() {
    let lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let h5 = lv.handle(4).unwrap();
    let mut cursor = lv.cursor(h5);

    assert_eq!(cursor.get(), &5);
    
    let h6 = cursor.move_next().unwrap();

    assert_eq!(lv[h6], 6);

    cursor.forward(2).expect("Should move forward 2.");
    assert_eq!(cursor.get(), &8);
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
    assert_eq!(cursor.get(), &6);

    cursor.move_to_back().unwrap();
    cursor.remove();
    assert_eq!(cursor.get(), &8);
    assert_eq!(lv.to_vec(), vec![1, 2, 3, 4, 6, 7, 8]);
}
