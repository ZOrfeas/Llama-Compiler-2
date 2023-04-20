use std::collections::HashMap;

use log::trace;

use crate::parse::ast::{data_map::NodeRef, Program};

use super::types::inference::InferenceGroup;
use super::types::type_map::TypeMap;

// type TypeMap<'a> = ast::data_map::DataMap<'a, Type>;
type Scope<'a> = HashMap<&'a str, NodeRef<'a>>;
#[derive(Debug)]
pub struct SemTable<'a> {
    scopes: Vec<Scope<'a>>,

    // *NOTE: Type substitutions in TypeMap will be applied in bulk after inference.
    pub types: TypeMap<'a>,
}

impl<'a> SemTable<'a> {
    pub fn new(ast: &'a Program) -> Self {
        Self {
            scopes: vec![Scope::new()],
            types: TypeMap::new(ast),
        }
    }
    pub fn push_scope(&mut self) {
        trace!("Pushing scope.");
        self.scopes.push(Scope::new());
    }
    pub fn pop_scope(&mut self) {
        trace!("Popping scope.");
        self.scopes.pop().expect("pop scope called on root scope");
    }
    // fn current_scope(&self) -> &Scope<'a> {
    //     self.scopes.last().expect("there should always be a scope")
    // }
    fn current_scope_mut(&mut self) -> &mut Scope<'a> {
        self.scopes
            .last_mut()
            .expect("there should always be a scope")
    }
    // pub fn get_type_by_id_lookup(&self, id: &str) -> Option<Rc<Type>> {
    //     self.lookup(id)
    //         .and_then(|node| self.types.get_node_type(&node))
    // }
    // *Note: this may need a way to handle shadowing without deleting previous entry.
    /// Returns none if there was no previous binding.
    /// Returns the previous binding if there was one.
    pub fn insert_scope_binding(
        &mut self,
        name: &'a str,
        node: impl Into<NodeRef<'a>>,
    ) -> Option<NodeRef<'a>> {
        self.current_scope_mut().insert(name, node.into())
    }
    // pub fn lookup_strict(&self, name: &str) -> Option<NodeRef<'a>> {
    //     self.current_scope().get(name).cloned()
    // }
    pub fn lookup(&self, name: &str) -> Option<NodeRef<'a>> {
        trace!("Looking up name: {}", name);
        for scope in self.scopes.iter().rev() {
            if let Some(node) = scope.get(name) {
                return Some(node.clone());
            }
        }
        None
    }
    #[inline(always)]
    pub fn new_inference_group(&self) -> InferenceGroup<'a> {
        InferenceGroup::new()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::ast::{self};

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
        assert_eq!(table.insert_scope_binding("foo", &foo_node), None);
        assert!(matches!(table.lookup("foo"), Some(_)));
        // assert!(matches!(table.lookup_strict("foo"), Some(_)));
        table.push_scope();
        let bar_node = new_const_def("bar");
        assert_eq!(table.insert_scope_binding("bar", &bar_node), None);
        assert!(matches!(table.lookup("bar"), Some(_)));
        // assert!(matches!(table.lookup_strict("bar"), Some(_)));
        assert!(matches!(table.lookup("foo"), Some(_)));
        // assert!(matches!(table.lookup_strict("foo"), None));
        table.pop_scope();
        assert!(matches!(table.lookup("bar"), None));
    }
}
