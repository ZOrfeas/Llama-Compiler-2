use std::{collections::HashMap, fmt::Debug, hash::Hash};

use enum_dispatch::enum_dispatch;

use super::{
    annotation::TypeAnnotation,
    def::{Constr, Def, Definition, Par, TDef},
    expr::{Clause, Expr, Pattern},
    Program,
};
#[derive(Debug)]
pub struct DataMap<'a, T> {
    map: HashMap<NodeRef<'a>, T>,
}
#[enum_dispatch(NodeRefInner)]
#[derive(Debug, Clone)]
pub enum NodeRef<'a> {
    Program(&'a Program),
    Definition(&'a Definition),
    Def(&'a Def),
    TDef(&'a TDef),
    Constr(&'a Constr),
    Type(&'a TypeAnnotation),
    Par(&'a Par),
    Expr(&'a Expr),
    Clause(&'a Clause),
    Pattern(&'a Pattern),
}
#[enum_dispatch]
trait NodeRefInner {
    // TODO: Think of a way to implement this function here only once.
    fn into_ptr(&self) -> *const ();
}
macro_rules! impl_node_ref_inner {
    ($($t:ty),*) => {
        $(
            impl<'a> NodeRefInner for &'a $t {
                fn into_ptr(&self) -> *const () {
                    *self as *const $t as *const ()
                }
            }
        )*
    }
}
impl_node_ref_inner!(
    Program,
    Definition,
    Def,
    TDef,
    Constr,
    TypeAnnotation,
    Par,
    Expr,
    Clause,
    Pattern
);

impl<'a, T> DataMap<'a, T> {
    pub fn new(p: &'a Program) -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    /// Insert a value into the map.
    /// The key is the node that the value is associated with.
    /// Returns None if the key was not present, otherwise returns the old value.
    pub fn insert<K: Into<NodeRef<'a>>>(&mut self, key: K, value: T) -> Option<T> {
        self.map.insert(key.into(), value)
    }
    pub fn get<K: Into<NodeRef<'a>>>(&self, key: K) -> Option<&T> {
        self.map.get(&key.into())
    }
    pub fn get_node(&self, key: &NodeRef<'a>) -> Option<&T> {
        self.map.get(key)
    }
    // pub fn iter(&self) -> std::collections::hash_map::Iter<NodeRef<'a>, T> {
    //     self.map.iter()
    // }
}
impl<'a> PartialEq for NodeRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        if std::mem::discriminant(self) == std::mem::discriminant(other) {
            std::ptr::eq(self.into_ptr(), other.into_ptr())
        } else {
            false
        }
    }
}
impl<'a> Eq for NodeRef<'a> {}
impl<'a> Hash for NodeRef<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.into_ptr().hash(state)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::ast::def::{Def, DefKind, Letdef};

    #[test]
    fn can_create() {
        let program = make_dummy_program();
        let _: DataMap<i32> = DataMap::new(&program);
    }
    #[test]
    fn can_insert() {
        let program = make_dummy_program();
        let mut data_map: DataMap<i32> = DataMap::new(&program);
        assert_eq!(data_map.insert(&program.definitions[0], 42), None);
    }
    #[test]
    fn can_get() {
        let program = make_dummy_program();
        let mut data_map: DataMap<i32> = DataMap::new(&program);
        data_map.insert(&program.definitions[0], 42);
        assert_eq!(data_map.get(&program.definitions[0]), Some(&42));
    }
    #[test]
    fn can_insert_twice() {
        let program = make_dummy_program();
        let mut data_map: DataMap<i32> = DataMap::new(&program);
        data_map.insert(&program.definitions[0], 42);
        data_map.insert(&program.definitions[0], 43);
        assert_eq!(data_map.get(&program.definitions[0]), Some(&43));
    }

    fn make_dummy_program() -> Program {
        Program {
            definitions: vec![Definition::Let(Letdef {
                rec: false,
                defs: vec![Def {
                    id: "some_name".to_string(),
                    type_: None,
                    kind: DefKind::Variable,
                    span: Default::default(),
                }],
                span: Default::default(),
            })],
            // span: Span::default(),
        }
    }
}
