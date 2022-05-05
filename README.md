# pqueue

[![license](https://img.shields.io/crates/l/pqueue.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/d/pqueue.svg)](https://crates.io/crates/pqueue)
[![version](https://img.shields.io/crates/v/pqueue.svg)](https://crates.io/crates/pqueue/)
[![documentation](https://docs.rs/pqueue/badge.svg)](https://docs.rs/pqueue/)

A fast little [priority queue](https://en.wikipedia.org/wiki/Priority_queue) for Rust.

Allows for items that have the `PartialOrd` trait.

## Example

Here we create a queue of simple integers.

```rust
let items = [9, 5, 1, 3, 4, 2, 6, 8, 9, 2, 1];
let mut q = pqueue::Queue::new();

for item in items {
    q.push(item);
}

while let Some(item) = q.pop() {
    println!("{}", item);
}

// OUTPUT:
// 1
// 1
// 2
// 2
// 3
// 4
// 5
// 6
// 8
// 9
// 9
```
