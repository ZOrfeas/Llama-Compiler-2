use crate::parse::ast::{
    self,
    annotation::TypeAnnotation,
    data_map::{DataMap, NodeRef},
};
use colored::{ColoredString, Colorize};
use log::info;
use std::{collections::HashMap, rc::Rc};
use strum::EnumDiscriminants;

// ! Implementation notes:
// !   Solve inference groups on every definition seperately.
// !   Types left unknown after solving a group are generic.
// !     On lookup of a generic definition:
// !       - Create an instantiation. An instantiation is an
#[derive(Debug)]
pub enum Constraint {
    Allow(Vec<TypeKind>),
    Disallow(Vec<TypeKind>),
}
impl Constraint {
    pub fn allow_numeric() -> Self {
        Self::Allow(vec![TypeKind::Int, TypeKind::Float])
    }
    pub fn allow_comparables() -> Self {
        Self::Allow(vec![TypeKind::Int, TypeKind::Float, TypeKind::Char])
    }
    // fn allow_
    pub fn disallow_array_and_func() -> Self {
        Self::Disallow(vec![TypeKind::Array, TypeKind::Func])
    }
}

#[derive(Debug)]
pub enum ArrayDims {
    Known(i32),
    LowerBounded(i32),
}

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(name(TypeKind))]
pub enum Type {
    // *Note: Unknown types can carry their own constraints.
    // !Note: Might be necessary to wrap the Vec<Constraint> in a RefCell.
    Unknown(u32, Option<Constraint>),
    Unit,
    Int,
    Char,
    Bool,
    Float,
    Func { lhs: Rc<Type>, rhs: Rc<Type> },
    Ref(Rc<Type>),
    // *Note: Store a possible lower bound for dim_cnt. Possibly with an enum.
    Array { inner: Rc<Type>, dim_cnt: ArrayDims },
    Tuple(Vec<Rc<Type>>),
    Custom { id: String },
}

impl From<&TypeAnnotation> for Rc<Type> {
    fn from(annotation: &TypeAnnotation) -> Self {
        use TypeAnnotation::*;
        match annotation {
            Unit => Rc::new(Type::Unit),
            Int => Rc::new(Type::Int),
            Char => Rc::new(Type::Char),
            Bool => Rc::new(Type::Bool),
            Float => Rc::new(Type::Float),
            Func { lhs, rhs } => Rc::new(Type::Func {
                lhs: (&**lhs).into(),
                rhs: (&**rhs).into(),
            }),
            Ref(inner) => Rc::new(Type::Ref((&**inner).into())),
            Array { inner, dim_cnt } => Rc::new(Type::Array {
                inner: (&**inner).into(),
                dim_cnt: ArrayDims::Known(*dim_cnt),
            }),
            Tuple(types) => Rc::new(Type::Tuple(types.iter().map(|t| (t).into()).collect())),
            Custom { id } => Rc::new(Type::Custom { id: id.clone() }),
        }
    }
}
impl Type {
    pub fn unknown_id_to_name(mut u: u32) -> String {
        let mut acc = Vec::new();
        loop {
            let c = (u % 26) as u8 + b'a';
            acc.push(if acc.is_empty() { c } else { c - 1 });
            if u < 26 {
                break;
            }
            u = u / 26;
        }
        acc.into_iter().rev().map(|c| c as char).collect()
    }
    pub fn new_ref(inner: Rc<Type>) -> Rc<Type> {
        Rc::new(Type::Ref(inner))
    }
    pub fn new_known_array(inner: Rc<Type>, dim_cnt: i32) -> Rc<Type> {
        Rc::new(Type::Array {
            inner,
            dim_cnt: ArrayDims::Known(dim_cnt),
        })
    }
    pub fn new_func(lhs: Rc<Type>, rhs: Rc<Type>) -> Rc<Type> {
        Rc::new(Type::Func { lhs, rhs })
    }
    pub fn new_tuple(types: Vec<Rc<Type>>) -> Rc<Type> {
        Rc::new(Type::Tuple(types))
    }
}
#[derive(Debug)]
pub struct InferenceGroup(Vec<(Rc<Type>, Rc<Type>)>);
// TODO: Add field that holds some info about where every unification came from.
impl InferenceGroup {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    #[inline(always)]
    pub fn insert_unificiation(&mut self, lhs: Rc<Type>, rhs: Rc<Type>) {
        info!("Inserting unification pair: {:?} = {:?}", lhs, rhs);
        self.0.push((lhs, rhs));
    }
}

#[derive(Debug)]
pub struct TypeMap<'a> {
    /// Attaches a type to every node in the AST (that makes sense to have a type).
    node_type_map: DataMap<'a, Rc<Type>>,
    // /// Stores the type object for each custom or builtin type by name.
    // name_type_map: HashMap<String, Rc<Type>>,
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
        let mut new_type_map = Self {
            node_type_map: DataMap::new(p),
            // name_type_map: HashMap::new(),
            unifications: HashMap::new(),
            next_unknown_id: 0,

            int_type: Rc::new(Type::Int),
            char_type: Rc::new(Type::Char),
            bool_type: Rc::new(Type::Bool),
            float_type: Rc::new(Type::Float),
            unit_type: Rc::new(Type::Unit),
        };
        new_type_map
    }
    pub fn insert(&mut self, node: impl Into<NodeRef<'a>>, ty: Rc<Type>) {
        if self.node_type_map.insert(node.into(), ty).is_some() {
            panic!("Tried to insert type for node that already has one.");
        }
    }
    // pub fn insert_custom_type(&mut self, name: String, ty: Rc<Type>) {
    //     if self.name_type_map.insert(name, ty).is_some() {
    //         panic!("Tried to insert type for node that already has one.");
    //     }
    // }
    // pub fn lookup_type(&self, name: &str) -> Option<Rc<Type>> {
    //     self.name_type_map.get(name).cloned()
    // }
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
