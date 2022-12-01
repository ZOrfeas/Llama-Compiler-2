mod token;

use crate::long_peekable::{LongPeek, LongPeekable};
use crate::scan;

pub struct Lexer<S: Iterator<Item = scan::LineType>> {
    scanner: LongPeekable<S>,
}

impl<S: Iterator<Item = scan::LineType>> Lexer<S> {
    pub fn new(scanner: S) -> Self {
        Lexer {
            scanner: scanner.long_peekable(),
        }
    }
}
