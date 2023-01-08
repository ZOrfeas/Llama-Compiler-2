use std::{io::Write, iter::FusedIterator};

pub struct WriterIterator<I, W, Item>
where
    I: Iterator<Item = Item>,
    W: Write,
    Item: std::fmt::Display,
{
    iter: I,
    writer: W,
}

impl<I, W, Item> Iterator for WriterIterator<I, W, Item>
where
    I: Iterator<Item = Item>,
    W: Write,
    Item: std::fmt::Display,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(inner) => {
                let s = format!("{}", inner);
                let _ = write!(self.writer, "{}", s);
                Some(inner)
            }
            None => None,
        }
    }
}
impl<I: FusedIterator, W, Item> FusedIterator for WriterIterator<I, W, Item>
where
    I: Iterator<Item = Item>,
    W: Write,
    Item: std::fmt::Display,
{
}
pub trait WriterIter<Item>: Iterator<Item = Item> + Sized
where
    Item: std::fmt::Display,
{
    fn writer_iter<W: Write>(self, writer: W) -> WriterIterator<Self, W, Item> {
        WriterIterator { iter: self, writer }
    }
}
impl<Item: std::fmt::Display, I: Iterator<Item = Item>> WriterIter<I::Item> for I {}
