#![allow(dead_code, unused_variables)]
use std::{collections::VecDeque, iter::FusedIterator};

pub struct LongPeekableIterator<I: Iterator> {
    iter: I,
    queue: VecDeque<Option<I::Item>>,
}

impl<I: Iterator> LongPeekableIterator<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            queue: VecDeque::new(),
        }
    }
    /// Works conceptually similar to [`std::iter::Peekable::peek()`].
    ///
    /// Every call to `peek()` will return the same value until `next()` is called (directly or indirectly).
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.fill_queue(1);
        self.queue.front().and_then(|x| x.as_ref())
    }
    /// Peeks `n` values into the iterator (zero-indexed).
    ///
    /// `peek_nth(0)` is equivalent to `peek()`.
    pub fn peek_nth(&mut self, n: usize) -> Option<&I::Item> {
        self.fill_queue(n + 1);
        self.queue.get(n).and_then(|x| x.as_ref())
    }
    /// Returns the specified range of peeked values (zero-indexed) form `start` (inclusive) to `end` (exclusive).
    pub fn peek_range(&mut self, start: usize, end: usize) -> &[Option<I::Item>] {
        assert!(start <= end);
        self.fill_queue(end);
        self.queue.make_contiguous(); // ensures that self.queue.as_slices().0 is the range we want
        &self.queue.as_slices().0[start..end]
    }
    /// Calls [`LongPeekable::peek_range`] with `start` set to `0` and `end` set to `amount`.
    pub fn peek_amount(&mut self, amount: usize) -> &[Option<I::Item>] {
        self.peek_range(0, amount)
    }
    /// Returns a drain iterator of all the currently stored peeked values.
    pub fn drain_all_peeked(&mut self) -> impl Iterator<Item = I::Item> + '_ {
        self.queue
            .drain(..)
            .take_while(|x| x.is_some())
            .map(|x| x.expect("take_while should have filtered out None values"))
    }
    /// Returns the count of currently stored peeked values.
    ///
    /// ```
    /// use llamac::long_peekable::*;
    /// let mut iter = (0..10).long_peekable();
    /// assert_eq!(iter.peek(), Some(&0));
    /// assert_eq!(iter.next(), Some(0));
    /// ```
    pub fn count_peeked(&self) -> usize {
        self.queue.len()
    }

    fn fill_queue(&mut self, required_elements: usize) {
        (self.queue.len()..required_elements).for_each(|_| self.push_next_to_queue());
    }
    fn push_next_to_queue(&mut self) {
        match self.queue.back() {
            Some(None) => return, // iterator is exhausted
            _ => self.queue.push_back(self.iter.next()),
        }
    }
}
impl<I: Iterator> Iterator for LongPeekableIterator<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            Some(item) => item,
            None => self.iter.next(),
        }
    }
}

impl<I: ExactSizeIterator> ExactSizeIterator for LongPeekableIterator<I> {}
impl<I: FusedIterator> FusedIterator for LongPeekableIterator<I> {}

pub trait LongPeek: Iterator + Sized {
    fn long_peekable(self) -> LongPeekableIterator<Self>;
}
impl<I: Iterator> LongPeek for I {
    fn long_peekable(self) -> LongPeekableIterator<Self> {
        LongPeekableIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_built() {
        let _ = (0..10).long_peekable();
    }
    #[test]
    fn simple_peek() {
        let mut iter = (0..10).long_peekable();
        assert_eq!(iter.peek(), Some(&0));
        assert_eq!(iter.peek(), Some(&0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.peek(), Some(&1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.peek(), Some(&3));
    }
    #[test]
    fn peek_nth() {
        let mut iter = (0..10).long_peekable();
        assert_eq!(iter.peek_nth(0), Some(&0));
        assert_eq!(iter.peek_nth(0), Some(&0));
        assert_eq!(iter.peek_nth(1), Some(&1));
        assert_eq!(iter.peek_nth(1), Some(&1));
        assert_eq!(iter.peek_nth(2), Some(&2));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.peek_nth(0), Some(&1));
        assert_eq!(iter.peek_nth(1), Some(&2));
    }
    #[test]
    fn peek_nth_with_next_nth() {
        let mut iter = (0..10).long_peekable();
        assert_eq!(iter.peek_nth(0), Some(&0));
        assert_eq!(iter.peek_nth(4), Some(&4));
        assert_eq!(iter.queue.len(), 5);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.queue.len(), 4);
        assert_eq!(iter.nth(3), Some(4));
        assert_eq!(iter.queue.len(), 0);
    }
    #[test]
    fn peek_queue() {
        let mut iter = (0..10).long_peekable();
        assert_eq!(iter.peek(), Some(&0));
        assert_eq!(iter.queue.len(), 1);
        assert_eq!(iter.peek(), Some(&0));
        assert_eq!(iter.queue.len(), 1);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.queue.len(), 0);
        assert_eq!(iter.peek(), Some(&1));
        assert_eq!(iter.queue.len(), 1);
        assert_eq!(iter.peek_nth(3), Some(&4));
        assert_eq!(iter.queue.len(), 4);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.queue.len(), 3);
    }
    #[test]
    fn peek_range() {
        let mut iter = (0..10).long_peekable();
        assert_eq!(iter.peek_range(0, 0), &[]);
        assert_eq!(iter.peek_range(0, 0), &[]);
        assert_eq!(iter.peek_range(0, 1), &[Some(0)]);
        assert_eq!(iter.peek_range(3, 5), &[Some(3), Some(4)]);
        assert_eq!(iter.queue.len(), 5);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.queue.len(), 4);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.queue.len(), 3);
        assert_eq!(iter.peek_range(0, 1), &[Some(2)]);
    }
    #[test]
    fn peek_after_end() {
        let mut iter = (0..10).long_peekable();
        assert_eq!(iter.peek_nth(10), None);
        assert_eq!(iter.peek(), Some(&0));
        assert_eq!(iter.queue.len(), 11);
        assert_eq!(iter.peek_nth(20), None);
        assert_eq!(iter.queue.len(), 11);
        assert_eq!(iter.nth(20), None);
        assert_eq!(iter.queue.len(), 0);
        assert_eq!(iter.peek(), None);
        assert_eq!(iter.queue.len(), 1);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.queue.len(), 0);
    }
    #[test]
    fn peek_amount() {
        let mut iter = (0..10).long_peekable();
        assert_eq!(iter.peek_amount(0), &[]);
        assert_eq!(iter.peek_amount(0), &[]);
        assert_eq!(iter.peek_amount(1), &[Some(0)]);
        assert_eq!(iter.peek_amount(3), &[Some(0), Some(1), Some(2)]);
        assert_eq!(iter.queue.len(), 3);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.queue.len(), 2);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.queue.len(), 1);
        assert_eq!(iter.peek_amount(1), &[Some(2)]);
    }
    #[test]
    fn drain_all_peeked() {
        let mut iter = (0..6).long_peekable();
        assert_eq!(iter.peek_amount(3), &[Some(0), Some(1), Some(2)]);
        assert_eq!(iter.queue.len(), 3);
        assert_eq!(iter.drain_all_peeked().collect::<Vec<i32>>(), [0, 1, 2]);
        assert_eq!(iter.queue.len(), 0);
        assert_eq!(iter.peek_amount(3), &[Some(3), Some(4), Some(5)]);
        assert_eq!(iter.queue.len(), 3);
        assert_eq!(iter.drain_all_peeked().collect::<Vec<i32>>(), [3, 4, 5]);
        assert_eq!(iter.queue.len(), 0);
        let empty: &[i32] = &[];
        assert_eq!(iter.drain_all_peeked().collect::<Vec<i32>>(), empty);
        assert_eq!(iter.peek(), None);
        assert_eq!(iter.drain_all_peeked().collect::<Vec<i32>>(), empty);
    }
    #[test]
    fn count_peeked() {
        let mut iter = (0..5).long_peekable();
        assert_eq!(iter.count_peeked(), 0);
        assert_eq!(iter.peek(), Some(&0));
        assert_eq!(iter.count_peeked(), 1);
        assert_eq!(iter.peek_nth(3), Some(&3));
        assert_eq!(iter.count_peeked(), 4);
        assert_eq!(iter.peek_range(0, 2), &[Some(0), Some(1)]);
        assert_eq!(iter.count_peeked(), 4);
        assert_eq!(iter.drain_all_peeked().collect::<Vec<i32>>(), &[0, 1, 2, 3]);
        assert_eq!(iter.count_peeked(), 0);
    }
}
