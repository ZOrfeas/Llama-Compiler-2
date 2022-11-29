use std::{
    collections::{vec_deque::Drain, VecDeque},
    iter::{FusedIterator, Map},
};

pub struct LongPeekable<I: Iterator> {
    iter: I,
    queue: VecDeque<Option<I::Item>>,
}

impl<I: Iterator> LongPeekable<I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            queue: VecDeque::new(),
        }
    }
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.fill_queue(1);
        self.queue.front().and_then(|x| x.as_ref())
    }
    pub fn peek_nth(&mut self, n: usize) -> Option<&I::Item> {
        self.fill_queue(n + 1);
        self.queue.get(n).and_then(|x| x.as_ref())
    }
    pub fn peek_range(&mut self, start: usize, end: usize) -> &[Option<I::Item>] {
        assert!(start <= end);
        self.fill_queue(end);
        self.queue.make_contiguous(); // ensures that self.queue.as_slices().0 is the range we want
        &self.queue.as_slices().0[start..end]
    }
    pub fn peek_amount(&mut self, amount: usize) -> &[Option<I::Item>] {
        self.peek_range(0, amount)
    }
    pub fn drain_all_peeked(
        &mut self,
    ) -> Map<
        Drain<Option<<I as Iterator>::Item>>,
        impl FnMut(Option<<I as Iterator>::Item>) -> <I as Iterator>::Item,
    > {
        self.queue.drain(..).map(|x| x.unwrap())
    }

    fn fill_queue(&mut self, required_elements: usize) {
        let stored_elements = self.queue.len();
        if stored_elements <= required_elements {
            for _ in stored_elements..required_elements {
                self.push_next_to_queue();
            }
        }
    }
    fn push_next_to_queue(&mut self) {
        match self.queue.back() {
            Some(None) => return, // iterator is exhausted
            _ => self.queue.push_back(self.iter.next()),
        }
    }
}
impl<I: Iterator> Iterator for LongPeekable<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            Some(item) => item,
            None => self.iter.next(),
        }
    }
}

impl<I: ExactSizeIterator> ExactSizeIterator for LongPeekable<I> {}
impl<I: FusedIterator> FusedIterator for LongPeekable<I> {}

pub trait LongPeek: Iterator + Sized {
    fn long_peekable(self) -> LongPeekable<Self>;
}
impl<I: Iterator> LongPeek for I {
    fn long_peekable(self) -> LongPeekable<Self> {
        LongPeekable::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::LongPeek;

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
        let mut iter = (0..10).long_peekable();
        assert_eq!(iter.peek_amount(3), &[Some(0), Some(1), Some(2)]);
        assert_eq!(iter.queue.len(), 3);
        let drained: Vec<i32> = iter.drain_all_peeked().collect();
        assert_eq!(drained, &[0, 1, 2]);
        assert_eq!(iter.queue.len(), 0);
        assert_eq!(iter.peek_amount(3), &[Some(3), Some(4), Some(5)]);
        assert_eq!(iter.queue.len(), 3);
        let drained: Vec<i32> = iter.drain_all_peeked().collect();
        assert_eq!(drained, &[3, 4, 5]);
        assert_eq!(iter.queue.len(), 0);
        let drained: Vec<i32> = iter.drain_all_peeked().collect();
        assert_eq!(drained, &[]);
    }
}
