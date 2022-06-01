//! An iterator adapter that sorts items within a sliding window.
//!
//! This applies a sliding window over the iterator, sorts items within that window, and yields them
//! in order within that window.
//! The implementation uses a [std::collections::BinaryHeap], so the default direction
//! is from highest to lowest item.
//! In order to change the sorting direction, use [std::cmp::Reverse].
//!
//! This is useful in situations where you, e.g., have an of items which are mostly sorted which is
//! possibly large, so you don't want to keep it in memory to be sorted.
//! If you can define some reasonable bounds on the "scrambledness" of the iterator, you can then
//! un-scramble it with this.
//!
//! Internally, the algorithm tries to keep the heap filled as long as the underlying iterator
//! produces items.
//! When the heap is filled or the underlying iterator does not yield anymore items, the next item
//! is popped off the heap.
//!
//! # Examples
//!
//! Basic usage: Adapt an iterator to be sorted.
//! ```
//! use window_sort_iterator::WindowSortIterExt;
//!
//! let a = &[4, 2, 3, 1];
//! let mut it = a.iter().cloned().window_sort(2);
//! assert_eq!(Some(4), it.next());
//! assert_eq!(Some(3), it.next());
//! assert_eq!(Some(2), it.next());
//! assert_eq!(Some(1), it.next());
//! assert_eq!(None, it.next());
//! ```
//!
//! Reverse, to use a min-heap:
//! ```
//! use std::cmp::Reverse;
//! use window_sort_iterator::window_sort;
//!
//! let a = &[1, 4, 2, 3];
//! let mut it = window_sort(a.iter().cloned().map(|i| Reverse(i)), 2).map(|i| i.0);
//! assert_eq!(Some(1), it.next());
//! assert_eq!(Some(2), it.next());
//! assert_eq!(Some(3), it.next());
//! assert_eq!(Some(4), it.next());
//! assert_eq!(None, it.next())
//! ```

use std::collections::BinaryHeap;

/// An iterator adapter that sorts items within a sliding window.
/// See the crate-level documentation for more info.
pub struct WindowSort<I>
where
    I: Iterator,
    <I as Iterator>::Item: Ord,
{
    orig: I,
    window_size: usize,
    heap: BinaryHeap<I::Item>,
}

impl<I> Iterator for WindowSort<I>
where
    I: Iterator,
    <I as Iterator>::Item: Ord,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Do we need to fill up the heap?
        while self.heap.len() < self.window_size {
            // Are there still items to be read from the underlying iterator?
            if let Some(item) = self.orig.next() {
                // If yes: push onto the heap.
                self.heap.push(item);
            } else {
                // If not: break from filling the heap, pop highest item.
                break;
            }
        }

        // Pop highest item off the heap.
        // If the heap is empty this will return None.
        self.heap.pop()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let heap_items = self.heap.len();
        match self.orig.size_hint() {
            (lower, Some(upper)) => (
                lower.saturating_add(heap_items),
                Some(upper.saturating_add(heap_items)),
            ),
            (lower, None) => (lower.saturating_add(heap_items), None),
        }
    }
}

/// Sorts the underlying iterator within a sliding window.
/// See the crate-level documentation for more info.
pub fn window_sort<I: Iterator>(xs: I, window_size: usize) -> WindowSort<I>
where
    <I as Iterator>::Item: Ord,
{
    WindowSort {
        orig: xs,
        window_size,
        heap: BinaryHeap::new(),
    }
}

/// Trait that extends iterators with functionality to sort items within a sliding window.
/// See the crate-level documentation for more info.
pub trait WindowSortIterExt: Sized {
    fn window_sort(self, window_size: usize) -> WindowSort<Self>
    where
        Self: Iterator,
        <Self as Iterator>::Item: Ord;
}

impl<I: Iterator> WindowSortIterExt for I
where
    <I as Iterator>::Item: Ord,
{
    /// Sorts the underlying iterator within a sliding window.
    /// See the crate-level documentation for more info.
    fn window_sort(self, window_size: usize) -> WindowSort<Self> {
        window_sort(self, window_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sort_i32_fn() {
        let a = &[3_i32, 4, 2, 1];
        let mut it = window_sort(a.iter().cloned(), 2);
        assert_eq!(Some(4), it.next());
        assert_eq!(Some(3), it.next());
        assert_eq!(Some(2), it.next());
        assert_eq!(Some(1), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn should_sort_i32_method() {
        let a = &[3_i32, 4, 2, 1];
        let mut it = a.iter().cloned().window_sort(2);
        assert_eq!(Some(4), it.next());
        assert_eq!(Some(3), it.next());
        assert_eq!(Some(2), it.next());
        assert_eq!(Some(1), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn should_sort_window_only() {
        let a = &[4_i32, 2, 1, 3];
        let mut it = window_sort(a.iter().cloned(), 2);
        assert_eq!(Some(4), it.next());
        assert_eq!(Some(2), it.next());
        assert_eq!(Some(3), it.next());
        assert_eq!(Some(1), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn small_underlying_iterator() {
        let a = &[2_i32, 3, 4, 1];
        let mut it = window_sort(a.iter().cloned(), 10);
        assert_eq!(Some(4), it.next());
        assert_eq!(Some(3), it.next());
        assert_eq!(Some(2), it.next());
        assert_eq!(Some(1), it.next());
        assert_eq!(None, it.next());
    }
}
