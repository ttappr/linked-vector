var searchIndex = JSON.parse('{\
"linked_vector":{"doc":"LinkedVector","t":[3,8,3,3,3,3,3,3,3,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,10,11,11,10,11,11,10,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Cursor","CursorBase","CursorMut","HNode","Handles","IntoIter","Iter","IterMut","LinkedVector","append","back","back_mut","back_node","backward","backward","backward","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","capacity","clear","clone","clone","clone_into","clone_into","contains","cursor","cursor_at","cursor_at_mut","cursor_mut","default","default","eq","eq","extend","extend","find_node","fmt","fmt","forward","forward","forward","from","from","from","from","from","from","from","from","from","from_iter","front","front_mut","front_node","get","get","get","get","get_mut","get_mut","get_mut","get_mut","handles","index","index_mut","insert_after","insert_before","into","into","into","into","into","into","into","into","into_iter","into_iter","into_iter","into_iter","into_iter","into_iter","into_iter","is_empty","iter","iter_mut","last","last","last","len","move_next","move_next","move_next","move_prev","move_prev","move_prev","move_to","move_to","move_to","move_to_end","move_to_end","move_to_end","move_to_start","move_to_start","move_to_start","new","new","new","new","next","next","next","next","next_back","next_back","next_back","next_back","next_node","pop_back","pop_front","prev_node","push_back","push_front","remove","remove_node","size_hint","size_hint","size_hint","size_hint","sort_unstable","to_owned","to_owned","to_vec","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","vzip","vzip","vzip","vzip","vzip","vzip","vzip","vzip","with_capacity"],"q":["linked_vector","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["A cursor which can only read the elements of the list.","A cursor is a position within a linked vector. It can be …","A cursor which can read and write the elements of the list.","A handle to a node within a <code>LinkedVector</code>. Internally, it …","An iterator over the elements of a <code>LinkedVector</code>. Yields …","The consuming iterator class of <code>LinkedVector</code>. Yields owned …","The basic iterator class of <code>LinkedVector</code>. Yields …","The basic iterator class of <code>LinkedVector</code>. Yields mutable …","A doubly-linked list that uses handles to refer to …","Moves all elements from <code>other</code> into <code>self</code>, leaving <code>other</code> …","Gives a reference to the back element, or <code>None</code> if the list …","Gives a mutable reference to the element back element, or …","Returns a handle to the last node in the list, or <code>None</code> if …","Moves the cursor backward by the specified number of …","","","","","","","","","","","","","","","","","","","Returns the total number of elements the vector can hold …","Removes all elements from the list.","","","","","Returns <code>true</code> if the list contains an element with the …","Creates a cursor that can be used to traverse the list.","Creates a cursor that can be used to traverse the list …","Creates a cursor that holds a mutable reference to the …","Creates a cursor that holds a mutable reference to the …","","","","","","","Returns the handle to the first node with the given value. …","","","Moves the cursor forward by the specified number of …","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","Returns the argument unchanged.","","Gives a reference to the element at the front of the …","Gives a mutable reference to the element at the front of …","Returns a handle to the first node in the list, or <code>None</code> if …","Returns a reference to the element at the cursor’s …","","","Provides a reference to the element indicated by the given …","Returns a mutable reference to the element at the cursor’…","","","Provides a mutable reference to the element indicated by …","Returns an iterator over the handles of the vector. The …","","","Inserts a new element after the one indicated by the …","Inserts a new element before the one indicated by the …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","Returns <code>true</code> if the list contains no elements.","Returns an iterator over the elements of the list.","Returns an iterator over the elements of the list. Renders …","","","","Returns the length of the list.","Moves the cursor to the next element. Returns the handle …","","","Moves the cursor to the previous element. Returns the …","","","Moves the cursor to the specified handle. Returns true if …","","","Moves the cursor to the end of the list. Returns the …","","","Moves the cursor to the start of the list. Returns the …","","","","","","Creates a new, empty <code>LinkedVector</code>.","","","","","","","","","Returns a handle to the next node in the list, or <code>None</code> if …","Pops the last element of the vector. Returns <code>None</code> if the …","Pops the first element of the vector. Returns <code>None</code> if the …","Returns a handle to the previous node in the list, or <code>None</code> …","Pushes a new element to the back of the list. Returns a …","Pushes a new element to the front of the list. Returns a …","Removes the first element with the indicated value. …","Removes the element indicated by the handle, <code>node</code>. Returns …","","","","","Sorts the elemements in place by their value. This …","","","Returns a vector containing the elements of the list. This …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Creates a new, empty <code>LinkedVector</code> with the specified …"],"i":[0,0,0,0,0,0,0,0,0,1,1,1,1,18,6,7,6,7,12,13,14,15,3,1,6,7,12,13,14,15,3,1,1,1,3,1,3,1,1,1,1,1,1,3,1,3,1,1,1,1,3,1,18,6,7,6,7,12,13,14,15,3,1,1,1,1,1,1,18,6,7,1,18,6,7,1,1,1,1,1,1,6,7,12,13,14,15,3,1,12,13,14,15,1,1,1,1,1,1,12,13,14,1,18,6,7,18,6,7,18,6,7,18,6,7,18,6,7,12,13,14,1,12,13,14,15,12,13,14,15,1,1,1,1,1,1,1,1,12,13,14,15,1,3,1,1,6,7,12,13,14,15,3,1,6,7,12,13,14,15,3,1,6,7,12,13,14,15,3,1,6,7,12,13,14,15,3,1,1],"f":[0,0,0,0,0,0,0,0,0,[[1,1]],[1,2],[1,2],[1,[[2,[3]]]],[4,[[5,[3,3]]]],[[6,4],[[5,[3,3]]]],[[7,4],[[5,[3,3]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1,4],[1],[3,3],[1,1],[[]],[[]],[1,8],[1,6],[[1,3],6],[[1,3],7],[1,7],[[],3],[[],1],[[3,3],8],[[1,1],8],[1],[1],[1,[[2,[3]]]],[[3,9],10],[[[1,[11]],9],10],[4,[[5,[3,3]]]],[[6,4],[[5,[3,3]]]],[[7,4],[[5,[3,3]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],1],[[]],[[],1],[1,2],[1,2],[1,[[2,[3]]]],[[],2],[6,2],[7,2],[[1,3],2],[[],2],[6,2],[7,2],[[1,3],2],[1,12],[[1,3]],[[1,3]],[[1,3],3],[[1,3],3],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1],[1],[1],[1,8],[1,13],[1,14],[12,2],[13,2],[14,2],[1,4],[[],[[2,[3]]]],[6,[[2,[3]]]],[7,[[2,[3]]]],[[],[[2,[3]]]],[6,[[2,[3]]]],[7,[[2,[3]]]],[3,8],[[6,3],8],[[7,3],8],[[],[[2,[3]]]],[6,[[2,[3]]]],[7,[[2,[3]]]],[[],[[2,[3]]]],[6,[[2,[3]]]],[7,[[2,[3]]]],[1,12],[1,13],[1,14],[[],1],[12,2],[13,2],[14,2],[15,2],[12,2],[13,2],[14,2],[15,2],[[1,3],[[2,[3]]]],[1,2],[1,2],[[1,3],[[2,[3]]]],[1,3],[1,3],[1,2],[[1,3],2],[12],[13],[14],[15],[1],[[]],[[]],[1,16],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[4,1]],"p":[[3,"LinkedVector"],[4,"Option"],[3,"HNode"],[15,"usize"],[4,"Result"],[3,"Cursor"],[3,"CursorMut"],[15,"bool"],[3,"Formatter"],[6,"Result"],[8,"Debug"],[3,"Handles"],[3,"Iter"],[3,"IterMut"],[3,"IntoIter"],[3,"Vec"],[3,"TypeId"],[8,"CursorBase"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
