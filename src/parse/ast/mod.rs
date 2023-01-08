pub mod annotation;
pub mod def;
pub mod expr;
pub mod print;

use self::def::Definition;

#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: Vec<Definition>,
}
