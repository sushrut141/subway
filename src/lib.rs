//! Subway is a fast, performant implementation of the skip list data structure written in rust.
//! Skip List is an alternative to self balancing sorted data structures like AVL Trees and
//! Red Black Trees.
//!
//! It supports fast insertion and lookup times with logarithmic complexity.
//!
//! Skip List is a probabilistic data structure that uses multiple stacked Linked Lists
//! to achieve fast read and writes.
//! For more information about how skip lists work
//! refer [here](https://en.wikipedia.org/wiki/Skip_list).
pub mod skiplist;

#[cfg(test)]
mod tests {
    use crate::skiplist::SkipList;

    #[test]
    fn test_skiplist() {
        let list: SkipList<i32, i32> = SkipList::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_operations() {
        let mut list: SkipList<i32, i32> = SkipList::new();
        list.insert(3,3);
        list.insert(1, 1);
        list.insert(2, 2);
        assert_eq!(list.len(), 3);
        let key = 1;
        assert_eq!(list.get(&key).is_some(), true);
        assert_eq!(list.get(&key).unwrap(), 1);
        assert_eq!(list.collect(), vec![(1, 1), (2, 2), (3, 3)]);
        list.delete(&key);
        assert_eq!(list.get(&key).is_none(), true);
        assert_eq!(list.len(), 2);
        assert_eq!(list.collect(), vec![(2, 2), (3, 3)]);
    }
}