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
