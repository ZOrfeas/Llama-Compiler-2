use crate::parse::ast::{self, data_map::DataMap};
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Unknown(u32),
    Unit,
    Int,
    Char,
    Bool,
    Float,
    Func { lhs: Rc<Type>, rhs: Rc<Type> },
    Ref(Rc<Type>),
    Array { inner: Rc<Type>, dim_cnt: i32 },
    Tuple(Vec<Rc<Type>>),
    Custom { id: String },
}
struct TypeMap<'a> {
    node_type_map: DataMap<'a, Rc<Type>>,
}

impl<'a> TypeMap<'a> {
    pub fn new(p: &'a ast::Program) -> Self {
        Self {
            node_type_map: DataMap::new(p),
        }
    }
}
