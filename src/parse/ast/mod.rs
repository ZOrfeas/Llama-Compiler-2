pub mod annotation;
pub mod data_map;
pub mod def;
pub mod expr;
pub mod print;

use crate::lex::token::Position;

use self::def::Definition;

#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: Vec<Definition>,
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
impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.start, self.end)
    }
}
