use std::collections::HashMap;

use crate::parse::ast::annotation::Type;

#[derive(Debug)]
pub struct TypeEquivalence {
    pub lhs: Type,
    pub rhs: Type,
    pub msg: &'static str,
}
#[derive(Debug)]
pub enum ConstraintKind {}
#[derive(Debug)]
pub struct Constraint {
    pub msg: &'static str,
}
#[derive(Debug)]
pub struct ConstraintGroup {
    pub constraints: HashMap<Type, Vec<Constraint>>,
}
