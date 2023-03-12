use std::collections::HashMap;

use crate::parse::ast::{self, data_map::NodeRef, Program};

use super::types::Type;

type TypeMap<'a> = ast::data_map::DataMap<'a, Type>;
type Scope<'a> = HashMap<&'a str, NodeRef<'a>>;
#[derive(Debug)]
pub struct SemTable<'a> {
    scopes: Vec<Scope<'a>>,

    // *NOTE: Type substitutions in TypeMap will be applied in bulk after inference.
    types: TypeMap<'a>,
    next_unknown_id: u32,
}

impl<'a> SemTable<'a> {
    pub fn new(ast: &'a Program) -> Self {
        Self {
            scopes: vec![Scope::new()],
            types: TypeMap::new(ast),
            next_unknown_id: 0,
        }
    }
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }
    pub fn pop_scope(&mut self) {
        self.scopes.pop().expect("pop scope called on root scope");
    }
    fn current_scope(&self) -> &Scope<'a> {
        self.scopes.last().expect("there should always be a scope")
    }
    fn current_scope_mut(&mut self) -> &mut Scope<'a> {
        self.scopes
            .last_mut()
            .expect("there should always be a scope")
    }
    pub fn insert_type(&mut self, node: impl Into<NodeRef<'a>>, ty: Type) -> &Type {
        let node_ref: NodeRef<'a> = node.into();
        self.types.insert(node_ref.clone(), ty);
        self.types.get_node(&node_ref).unwrap()
    }
    pub fn get_type(&self, node: NodeRef<'a>) -> Option<&Type> {
        self.types.get_node(&node)
    }
    pub fn get_type_by_id_lookup(&self, id: &str) -> Option<&Type> {
        self.lookup(id).and_then(|node| self.get_type(node))
    }
    // *Note: this may need a way to handle shadowing without deleting previous entry.
    /// Returns none if there was no previous binding.
    /// Returns the previous binding if there was one.
    pub fn insert_binding(
        &mut self,
        name: &'a str,
        node: impl Into<NodeRef<'a>>,
    ) -> Option<NodeRef<'a>> {
        self.current_scope_mut().insert(name, node.into())
    }
    pub fn lookup_strict(&self, name: &str) -> Option<NodeRef<'a>> {
        self.current_scope().get(name).cloned()
    }
    pub fn lookup(&self, name: &str) -> Option<NodeRef<'a>> {
        for scope in self.scopes.iter().rev() {
            if let Some(node) = scope.get(name) {
                return Some(node.clone());
            }
        }
        None
    }

    // pub fn new_unknown(&mut self) -> Type {
    //     let unknown_id = self.next_unknown_id;
    //     self.next_unknown_id += 1;
    //     Type::Unknown(unknown_id)
    // }
    // pub fn new_many_unknowns(&mut self, count: usize) -> Vec<Type> {
    //     (0..count).map(|_| self.new_unknown()).collect()
    // }
    // pub fn new_ref(&mut self) -> Type {
    //     Type::Ref(Box::new(self.new_unknown()))
    // }
}
// TODO: Write some tests

#[cfg(test)]
mod test {
    use super::*;

    fn new_empty_program() -> ast::Program {
        ast::Program {
            definitions: Vec::new(),
        }
    }
    fn new_test_table<'a>(p: &'a Program) -> SemTable<'a> {
        SemTable::new(p)
    }
    fn new_const_def(name: &str) -> ast::def::Def {
        ast::def::Def {
            id: name.to_owned(),
            type_: None,
            kind: ast::def::DefKind::Const {
                expr: ast::expr::Expr {
                    kind: ast::expr::ExprKind::UnitLiteral,
                    span: Default::default(),
                },
            },
            span: Default::default(),
        }
    }

    #[test]
    fn scope_inserts_ordering_and_basic_lookup() {
        let p = new_empty_program();
        let mut table = new_test_table(&p);
        let foo_node = new_const_def("foo");
        assert_eq!(table.insert_binding("foo", &foo_node), None);
        assert!(matches!(table.lookup("foo"), Some(_)));
        assert!(matches!(table.lookup_strict("foo"), Some(_)));
        table.push_scope();
        let bar_node = new_const_def("bar");
        assert_eq!(table.insert_binding("bar", &bar_node), None);
        assert!(matches!(table.lookup("bar"), Some(_)));
        assert!(matches!(table.lookup_strict("bar"), Some(_)));
        assert!(matches!(table.lookup("foo"), Some(_)));
        assert!(matches!(table.lookup_strict("foo"), None));
        table.pop_scope();
        assert!(matches!(table.lookup("bar"), None));
    }
}
