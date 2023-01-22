pub mod annotation;
pub mod data_map;
pub mod def;
pub mod expr;
pub mod print;

use std::rc::Rc;

use crate::lex::token::Position;

use self::def::Definition;

// TODO: Think of a way to store the start and end position of every ast-node.

#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: Vec<Definition>,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}
impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}
impl Default for Span {
    fn default() -> Self {
        Self {
            start: Default::default(),
            end: Default::default(),
        }
    }
}

impl Program {
    pub fn new(definitions: Vec<Definition>, from: Position, to: Position) -> Self {
        Self {
            definitions,
            span: Span::new(from, to),
        }
    }
}
