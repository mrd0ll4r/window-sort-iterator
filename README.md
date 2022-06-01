# window-sort-iterator

[![Crates.io](https://img.shields.io/crates/d/window-sort-iterator.svg)](https://crates.io/crates/window-sort-iterator)
[![Crates.io](https://img.shields.io/crates/v/window-sort-iterator.svg)](https://crates.io/crates/window-sort-iterator)
[![Released API docs](https://docs.rs/window-sort-iterator/badge.svg)](https://docs.rs/window-sort-iterator)

An iterator adapter that sorts items within a sliding window.

## Implementation

This keeps a [BinaryHeap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html) of the items within a
sliding window on top of an iterator.
The algorithm works like this:
- As long as the window is not full, requests elements from the underlying iterator and inserts them into the heap.
- If the window is full, pops the next sorted element from the heap.
- If the underlying iterator does not produce any more items, drains the remaining elements in-order from the heap.

By default, this uses a max-heap, which results in the highest item being yielded first.
You can use a min-heap by wrapping items with `std::cmp::Reverse`.

## Usage

```rust
use window_sort_iterator::WindowSortIterExt;

let a = &[4, 2, 3, 1];
let mut it = a.iter().cloned().window_sort(2);
assert_eq!(Some(4), it.next());
assert_eq!(Some(3), it.next());
assert_eq!(Some(2), it.next());
assert_eq!(Some(1), it.next());
assert_eq!(None, it.next());
```

## License

MIT, see [LICENSE](LICENSE).