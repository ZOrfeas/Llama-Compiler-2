use std::io::Write;

pub struct WriterIterator<I, W, Item>
where
    I: Iterator<Item = Item>,
    W: Write,
    Item: std::fmt::Display,
{
    iter: I,
    writer: W,
    newline_end: bool,
}

impl<I, W, Iter> Iterator for WriterIterator<I, W, Iter>
where
    I: Iterator<Item = Iter>,
    W: Write,
    Iter: std::fmt::Display,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(inner) => {
                let s = format!("{}", inner);
                let _ = write!(self.writer, "{}", s);
                if !s.ends_with("\n") {
                    self.newline_end = true;
                }
                Some(inner)
            }
            None => {
                if self.newline_end {
                    let _ = write!(self.writer, "\n");
                }
                None
            }
        }
    }
}

pub trait WriterIter<Item>: Iterator<Item = Item> + Sized
where
    Item: std::fmt::Display,
{
    fn writer_iter<W: Write>(self, writer: W) -> WriterIterator<Self, W, Item> {
        WriterIterator {
            iter: self,
            writer,
            newline_end: false,
        }
    }
}

impl<Item: std::fmt::Display, I: Iterator<Item = Item>> WriterIter<I::Item> for I {}
