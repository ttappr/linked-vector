#![allow(unused_variables)]

use crate::linked_vector::*;
use crate::cursor::*;

#[test]
fn cursor() {
    let lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7]);
    let mut cursor = lv.cursor(lv.front_node().unwrap());
    
    let mut cursor = {

        #[cfg(feature = "optionless-accessors")]
        {
            assert_eq!(cursor.get(), &1);
            
            cursor.move_next();
            
            assert_eq!(cursor.get(), &2);
            
            let hend = cursor.move_to_back().unwrap();
            let hbak = cursor.backward(2).unwrap();
            
            assert_eq!(cursor.get(), &5);
            assert_eq!(lv.get(hend), &7);
            assert_eq!(lv.get(hbak), &5);
            lv.cursor(hbak)
        }
        #[cfg(not(feature = "optionless-accessors"))]
        {
            assert_eq!(cursor.get(), Some(&1));
            
            cursor.move_next();
            
            assert_eq!(cursor.get(), Some(&2));
            
            let hend = cursor.move_to_back().unwrap();
            let hbak = cursor.backward(2).unwrap();
            
            assert_eq!(cursor.get(), Some(&5));
            assert_eq!(lv.get(hend), Some(&7));
            assert_eq!(lv.get(hbak), Some(&5));
            lv.cursor(hbak)
        }
    };
    
    match cursor.backward(20) {
        Ok(handle) => panic!("Should move to beginning on overshoot."),
        Err(handle) => { 
            #[cfg(feature = "optionless-accessors")]
            { assert_eq!(lv.get(handle), &1) }
            #[cfg(not(feature = "optionless-accessors"))]
            { assert_eq!(lv.get(handle), Some(&1)) }
        },
    }

    match cursor.forward(20) {
        Ok(handle) => panic!("Should move to end on overshoot."),
        Err(handle) => {
            #[cfg(feature = "optionless-accessors")]
            { assert_eq!(lv.get(handle), &7) }
            #[cfg(not(feature = "optionless-accessors"))]
            { assert_eq!(lv.get(handle), Some(&7)) }
        },
    }
}

#[test]
fn cursor_2() {
    let lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let h5 = lv.handle(4).unwrap();
    let mut cursor = lv.cursor(h5);

    #[cfg(feature = "optionless-accessors")]
    {
        assert_eq!(cursor.get(), &5);
    }
    #[cfg(not(feature = "optionless-accessors"))]
    {
        assert_eq!(cursor.get(), Some(&5));
    }
    
    let h6 = cursor.move_next().unwrap();

    assert_eq!(lv[h6], 6);

    cursor.forward(2).expect("Should move forward 2.");

    #[cfg(feature = "optionless-accessors")]
    {
        assert_eq!(cursor.get(), &8);
    }
    #[cfg(not(feature = "optionless-accessors"))]
    {
        assert_eq!(cursor.get(), Some(&8));
    }
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
#[cfg(feature = "cursor-remove")]
fn cursor_remove() {
    let mut lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let h5 = lv.handle(4).unwrap();
    
    let mut cursor = lv.cursor_mut(h5);

    cursor.remove();

    #[cfg(feature = "optionless-accessors")]
    {
        assert_eq!(cursor.get(), &6);
    }
    #[cfg(not(feature = "optionless-accessors"))]
    {
        assert_eq!(cursor.get(), Some(&6));
    }

    cursor.move_to_back().unwrap();
    cursor.remove();

    #[cfg(feature = "optionless-accessors")]
    {
        assert_eq!(cursor.get(), &8);
    }
    #[cfg(not(feature = "optionless-accessors"))]
    {
        assert_eq!(cursor.get(), Some(&8));
    }
    
    assert_eq!(lv.to_vec(), vec![1, 2, 3, 4, 6, 7, 8]);
}

#[test]
fn cursor_deref() {
    let lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let h5 = lv.handle(4).unwrap();
    let mut cursor = lv.cursor(h5);

    assert_eq!(*cursor, 5);

    cursor.forward(2).expect("Should move forward 2.");

    assert_eq!(*cursor, 7);
}

#[test]
fn cursor_mut_deref() {
    let mut lv = LinkedVector::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let h5 = lv.handle(4).unwrap();
    let mut cursor = lv.cursor_mut(h5);

    assert_eq!(*cursor, 5);
    *cursor = 10;
    assert_eq!(*cursor, 10);
    
    cursor.forward(2).expect("Should move forward 2.");

    assert_eq!(*cursor, 7);

    assert_eq!(lv.to_vec(), vec![1, 2, 3, 4, 10, 6, 7, 8, 9]);
}