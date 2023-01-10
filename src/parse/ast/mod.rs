pub mod annotation;
pub mod data_map;
pub mod def;
pub mod expr;
pub mod print;

use self::def::Definition;

#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: Vec<Definition>,
}
