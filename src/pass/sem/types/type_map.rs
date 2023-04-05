use std::{collections::HashMap, rc::Rc};

use crate::parse::ast::{
    self,
    data_map::{DataMap, NodeRef},
};

use super::{Constraint, Type};

#[derive(Debug)]
pub struct TypeMap<'a> {
    /// Attaches a type to every node in the AST (that makes sense to have a type).
    node_type_map: DataMap<'a, Rc<Type>>,
    /// Stores the resolved type unifications after inference.
    unifications: HashMap<u32, Rc<Type>>,

    /// The id of the next unknown type to be created.
    next_unknown_id: u32,

    int_type: Rc<Type>,
    char_type: Rc<Type>,
    bool_type: Rc<Type>,
    float_type: Rc<Type>,
    unit_type: Rc<Type>,
}

impl<'a> TypeMap<'a> {
    pub fn new(p: &'a ast::Program) -> Self {
        Self {
            node_type_map: DataMap::new(p),
            unifications: HashMap::new(),
            next_unknown_id: 0,

            int_type: Rc::new(Type::Int),
            char_type: Rc::new(Type::Char),
            bool_type: Rc::new(Type::Bool),
            float_type: Rc::new(Type::Float),
            unit_type: Rc::new(Type::Unit),
        }
    }
    pub fn insert(&mut self, node: impl Into<NodeRef<'a>>, ty: Rc<Type>) {
        if self.node_type_map.insert(node.into(), ty).is_some() {
            panic!("Tried to insert type for node that already has one.");
        }
    }
    #[inline(always)]
    pub fn get_type(&self, node: impl Into<NodeRef<'a>>) -> Option<Rc<Type>> {
        self.node_type_map.get(node).cloned()
    }
    #[inline(always)]
    pub fn get_node_type(&self, node: &NodeRef<'a>) -> Option<Rc<Type>> {
        self.node_type_map.get_node(node).cloned()
    }
    pub fn new_unknown_with_constraint(&mut self, constraints: Constraint) -> Rc<Type> {
        let id = self.next_unknown_id;
        self.next_unknown_id += 1;
        Rc::new(Type::Unknown(id, Some(constraints)))
    }
    pub fn new_unknown(&mut self) -> Rc<Type> {
        let id = self.next_unknown_id;
        self.next_unknown_id += 1;
        Rc::new(Type::Unknown(id, None))
    }
    pub fn new_unknown_ref(&mut self) -> Rc<Type> {
        Rc::new(Type::Ref(self.new_unknown()))
    }
    #[inline(always)]
    pub fn get_int(&self) -> Rc<Type> {
        self.int_type.clone()
    }
    #[inline(always)]
    pub fn get_char(&self) -> Rc<Type> {
        self.char_type.clone()
    }
    #[inline(always)]
    pub fn get_bool(&self) -> Rc<Type> {
        self.bool_type.clone()
    }
    #[inline(always)]
    pub fn get_float(&self) -> Rc<Type> {
        self.float_type.clone()
    }
    #[inline(always)]
    pub fn get_unit(&self) -> Rc<Type> {
        self.unit_type.clone()
    }
}
