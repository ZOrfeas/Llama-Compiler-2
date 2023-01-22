use std::{collections::HashMap, hash::Hash};

use super::{
    annotation::Type,
    def::{Constr, Def, Definition, Par, TDef},
    expr::{Clause, Expr, Pattern},
    Program,
};
#[derive(Debug)]
pub struct DataMap<'a, T> {
    map: HashMap<NodeRef<'a>, T>,
}
#[derive(Debug, Clone)]
pub enum NodeRef<'a> {
    Program(&'a Program),
    Definition(&'a Definition),
    Def(&'a Def),
    TDef(&'a TDef),
    Constr(&'a Constr),
    Type(&'a Type),
    Par(&'a Par),
    Expr(&'a Expr),
    Clause(&'a Clause),
    Pattern(&'a Pattern),
}

impl<'a, T> DataMap<'a, T> {
    pub fn new(p: &'a Program) -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn insert<K: Into<NodeRef<'a>>>(&mut self, key: K, value: T) -> Option<T> {
        self.map.insert(key.into(), value)
    }
    pub fn get<K: Into<NodeRef<'a>>>(&self, key: K) -> Option<&T> {
        self.map.get(&key.into())
    }
    // pub fn iter(&self) -> std::collections::hash_map::Iter<NodeRef<'a>, T> {
    //     self.map.iter()
    // }
}
impl<'a> From<&'a Program> for NodeRef<'a> {
    fn from(item: &'a Program) -> Self {
        NodeRef::Program(item)
    }
}
impl<'a> From<&'a Definition> for NodeRef<'a> {
    fn from(item: &'a Definition) -> Self {
        NodeRef::Definition(item)
    }
}
impl<'a> From<&'a Def> for NodeRef<'a> {
    fn from(item: &'a Def) -> Self {
        NodeRef::Def(item)
    }
}
impl<'a> From<&'a TDef> for NodeRef<'a> {
    fn from(item: &'a TDef) -> Self {
        NodeRef::TDef(item)
    }
}
impl<'a> From<&'a Constr> for NodeRef<'a> {
    fn from(item: &'a Constr) -> Self {
        NodeRef::Constr(item)
    }
}
impl<'a> From<&'a Type> for NodeRef<'a> {
    fn from(item: &'a Type) -> Self {
        NodeRef::Type(item)
    }
}
impl<'a> From<&'a Par> for NodeRef<'a> {
    fn from(item: &'a Par) -> Self {
        NodeRef::Par(item)
    }
}
impl<'a> From<&'a Expr> for NodeRef<'a> {
    fn from(item: &'a Expr) -> Self {
        NodeRef::Expr(item)
    }
}
impl<'a> From<&'a Clause> for NodeRef<'a> {
    fn from(item: &'a Clause) -> Self {
        NodeRef::Clause(item)
    }
}
impl<'a> From<&'a Pattern> for NodeRef<'a> {
    fn from(item: &'a Pattern) -> Self {
        NodeRef::Pattern(item)
    }
}
impl<'a> PartialEq for NodeRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeRef::Program(this), NodeRef::Program(other)) => std::ptr::eq(*this, *other),
            (NodeRef::Definition(this), NodeRef::Definition(other)) => std::ptr::eq(*this, *other),
            (NodeRef::Def(this), NodeRef::Def(other)) => std::ptr::eq(*this, *other),
            (NodeRef::TDef(this), NodeRef::TDef(other)) => std::ptr::eq(*this, *other),
            (NodeRef::Constr(this), NodeRef::Constr(other)) => std::ptr::eq(*this, *other),
            (NodeRef::Type(this), NodeRef::Type(other)) => std::ptr::eq(*this, *other),
            (NodeRef::Par(this), NodeRef::Par(other)) => std::ptr::eq(*this, *other),
            (NodeRef::Expr(this), NodeRef::Expr(other)) => std::ptr::eq(*this, *other),
            (NodeRef::Clause(this), NodeRef::Clause(other)) => std::ptr::eq(*this, *other),
            (NodeRef::Pattern(this), NodeRef::Pattern(other)) => std::ptr::eq(*this, *other),
            _ => false,
        }
    }
}
impl<'a> Eq for NodeRef<'a> {}
impl<'a> Hash for NodeRef<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            NodeRef::Program(item) => ((*item) as *const Program).hash(state),
            NodeRef::Definition(item) => ((*item) as *const Definition).hash(state),
            NodeRef::Def(item) => ((*item) as *const Def).hash(state),
            NodeRef::TDef(item) => ((*item) as *const TDef).hash(state),
            NodeRef::Constr(item) => ((*item) as *const Constr).hash(state),
            NodeRef::Type(item) => ((*item) as *const Type).hash(state),
            NodeRef::Par(item) => ((*item) as *const Par).hash(state),
            NodeRef::Expr(item) => ((*item) as *const Expr).hash(state),
            NodeRef::Clause(item) => ((*item) as *const Clause).hash(state),
            NodeRef::Pattern(item) => ((*item) as *const Pattern).hash(state),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::ast::{
        def::{Letdef, VariableDef},
        Span,
    };

    #[test]
    fn can_create() {
        let program = make_dummy_program();
        let mut data_map: DataMap<i32> = DataMap::new(&program);
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
                defs: vec![Def::Variable(VariableDef {
                    id: "some_name".to_string(),
                    type_: None,
                })],
            })],
            span: Span::default(),
        }
    }
}
