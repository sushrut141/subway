SkipList
========

A fast, performant implementation of skip list in Rust.  
A skip list is probabilistic data structure that provides `O(log N)` search and insertion complexity.  
For more information about how to skiplist work refer [here](https://en.wikipedia.org/wiki/Skip_list).

![Build](https://github.com/sushrut141/skiplist/workflows/Rust/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](https://opensource.org/licenses/MIT)

## Usage

The SkipList supports the following operations.

### `insert`

Insert an element into the list while maintaining sorted order.  
The insert method accepts a key and a value.   
The values in the list will be stored sorted by key.

```rust
let list = SkipList::new();
list.insert(1, 1);
list.insert(2, 2);
```

### `get`

Returns an optional value if the supplied key is found in the list.  
Time complexity of this operation is around `O(logN)`.

```rust
let maybe_value = list.get(&key);
if maybe_value.is_some() {
    let value = maybe_value.unwrap();
} 
```

### `delete`

Deletes an item from the linked list if present using the supplied key.

```rust
list.delete(&key_to_delete);
```

### `collect`

Collects the list of items in the skip list sorted by key into a list.

```rust
list.insert(2, 2);
list.insert(2, 2);
list.insert(5, 5);
let values = list.collect(); // (1, 1), (2, 2), (5, 5)
```