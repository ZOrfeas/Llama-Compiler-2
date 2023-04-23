use std::{cell::RefCell, collections::HashMap, rc::Rc};

use log::trace;

use crate::{
    parse::ast::{
        self,
        data_map::{DataMap, NodeRef, NodeRefInner},
    },
    pass::sem::types::inference::InfererHelpers,
};

use super::{inference::Constraints, Type};

#[derive(Debug)]
pub struct TypeMap<'a> {
    /// Attaches a type to every node in the AST (that makes sense to have a type).
    node_type_map: DataMap<'a, Rc<Type>>,

    /// Stores instantiations for each generic type.
    /// TODO: The index of the instantiation can be stored at the call-site to help lookup during codegen.
    instantiations: DataMap<'a, Vec<Rc<Type>>>,
    /// Stores the resolved type unifications after inference.
    pub unifications: HashMap<u32, Rc<Type>>,

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

            instantiations: DataMap::new(p),
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
        trace!("Getting type for node: {}", node);
        self.node_type_map.get_node(node).cloned().map(|t| {
            trace!("Got type: {}", t);
            t
        })
    }
    pub fn mark_generic(&mut self, node: impl Into<NodeRef<'a>>) {
        self.instantiations.insert(node.into(), Vec::new());
    }
    fn instantiate(&mut self, ty: &Rc<Type>) -> Rc<Type> {
        // *Done: Fully traverse, instantiate each unknown type, and then create one using those mappings.
        let mut mappings = HashMap::new();
        self.traverse_and_instantiate(ty, &mut mappings);
        self.instantiate_with_mappings(ty, &mappings)
    }
    fn instantiate_with_mappings(
        &self,
        ty: &Rc<Type>,
        mappings: &HashMap<u32, Rc<Type>>,
    ) -> Rc<Type> {
        use Type::*;
        match &**ty {
            Unknown(id, _) => mappings
                .get(&id)
                .expect("instantiation should have been created")
                .clone(),
            Func { lhs, rhs } => Type::new_func(
                self.instantiate_with_mappings(lhs, mappings),
                self.instantiate_with_mappings(rhs, mappings),
            ),
            Ref(inner) => Type::new_ref(self.instantiate_with_mappings(inner, mappings)),
            Array { inner, dim_cnt } => Type::new_array(
                self.instantiate_with_mappings(inner, mappings),
                dim_cnt.borrow().borrow().clone(),
            ),
            Tuple(types) => Type::new_tuple(
                types
                    .iter()
                    .map(|t| self.instantiate_with_mappings(t, mappings))
                    .collect(),
            ),
            // Unit | Int | Char | Bool | Float | Custom { .. } => ty.clone(),
            _ => ty.clone(),
        }
    }
    fn traverse_and_instantiate(&mut self, ty: &Rc<Type>, mappings: &mut HashMap<u32, Rc<Type>>) {
        use Type::*;
        match &**ty {
            Unknown(id, constraints) if !mappings.contains_key(id) => {
                let new_ty = self.new_unknown_with_constraint(constraints.borrow().clone());
                mappings.insert(*id, new_ty.clone());
            }
            Func { lhs, rhs } => {
                self.traverse_and_instantiate(lhs, mappings);
                self.traverse_and_instantiate(rhs, mappings);
            }
            Ref(inner) => self.traverse_and_instantiate(inner, mappings),
            Array { inner, .. } => {
                self.traverse_and_instantiate(inner, mappings);
            }
            Tuple(types) => {
                for ty in types {
                    self.traverse_and_instantiate(ty, mappings);
                }
            }
            _ => {}
        }
    }
    pub fn get_node_type_or_instantiation(&mut self, node: &NodeRef<'a>) -> Rc<Type> {
        let node_type = self
            .get_node_type(node)
            .expect("looked up node should have a type");
        // TODO: Find a way to avoid two get_node_mut lookups. Currently not allowed cause get_node_mut returns a &mut ref which self.instantiate also wants.
        if self.instantiations.get_node_mut(node).is_none() {
            return node_type;
        }
        let node_type = self.deep_resolve_type(node_type);
        let instance = self.instantiate(&node_type);
        trace!("Instantiating generic {} to {}", node_type, instance);
        self.instantiations
            .get_node_mut(node)
            .unwrap()
            .push(instance.clone());
        instance
    }

    pub fn print_node_types(&mut self, mut w: impl std::io::Write) -> std::io::Result<()> {
        writeln!(
            w,
            "{}",
            format!("{:^50}│{:^60}│{:^50}", "Node", "Type", "Location")
        )?;
        writeln!(w, "{}", format!("{:─^50}┼{:─^60}┼{:─^50}", "", "", ""))?;
        for (node, ty) in self.node_type_map.iter() {
            writeln!(
                w,
                "{:^50}│{:^60}│{:^50}",
                node.to_string(),
                self.deep_resolve_type(ty.clone()).to_string(),
                node.get_span().start.to_string()
            )?;
        }
        Ok(())
    }

    #[inline(always)]
    fn get_and_advance_unknown_id(&mut self) -> u32 {
        let id = self.next_unknown_id;
        self.next_unknown_id += 1;
        id
    }
    pub fn new_unknown_with_constraint(&mut self, constraints: Constraints) -> Rc<Type> {
        let id = self.get_and_advance_unknown_id();
        Rc::new(Type::Unknown(id, RefCell::new(constraints)))
    }
    pub fn new_unknown(&mut self) -> Rc<Type> {
        let id = self.get_and_advance_unknown_id();
        Rc::new(Type::Unknown(id, RefCell::new(Constraints::new())))
    }
    #[inline(always)]
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
