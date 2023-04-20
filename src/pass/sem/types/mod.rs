pub mod inference;
pub mod type_map;

use crate::parse::ast::annotation::TypeAnnotation;
use std::{cell::RefCell, rc::Rc};
use strum::EnumDiscriminants;

use self::inference::{ArrayDims, Constraints};

// ! Implementation notes:
// !   Solve inference groups on every definition seperately.
// !   Types left unknown after solving a group are generic.
// !     On lookup of a generic definition:
// !       - Create an instantiation. An instantiation is an

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(name(TypeKind))]
#[strum_discriminants(derive(Hash))]
pub enum Type {
    // *Note: Unknown types can carry their own constraints.
    Unknown(u32, RefCell<Constraints>),
    Unit,
    Int,
    Char,
    Bool,
    Float,
    Func {
        lhs: Rc<Type>,
        rhs: Rc<Type>,
    },
    Ref(Rc<Type>),
    // *Note: Store a possible lower bound for dim_cnt. Possibly with an enum.
    Array {
        inner: Rc<Type>,
        dim_cnt: RefCell<Rc<RefCell<ArrayDims>>>,
    },
    Tuple(Vec<Rc<Type>>),
    Custom {
        id: String,
    },
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
    #[inline(always)]
    pub fn new_ref(inner: Rc<Type>) -> Rc<Type> {
        Rc::new(Type::Ref(inner))
    }
    #[inline(always)]
    pub fn new_array(inner: Rc<Type>, dim_cnt: ArrayDims) -> Rc<Type> {
        Rc::new(Type::Array {
            inner,
            dim_cnt: RefCell::new(Rc::new(RefCell::new(dim_cnt))),
        })
    }
    #[inline(always)]
    pub fn new_known_array(inner: Rc<Type>, dim_cnt: u32) -> Rc<Type> {
        Self::new_array(inner, ArrayDims::Known(dim_cnt))
    }
    #[inline(always)]
    pub fn new_bounded_array(inner: Rc<Type>, bound: u32) -> Rc<Type> {
        Self::new_array(inner, ArrayDims::LowerBounded(bound))
    }
    #[inline(always)]
    pub fn new_func(lhs: Rc<Type>, rhs: Rc<Type>) -> Rc<Type> {
        Rc::new(Type::Func { lhs, rhs })
    }
    #[inline(always)]
    pub fn new_multi_arg_func(args: Vec<Rc<Type>>, ret: Rc<Type>) -> Rc<Type> {
        args.into_iter()
            .rfold(ret, |acc, arg| Type::new_func(arg, acc))
    }
    #[inline(always)]
    pub fn new_tuple(types: Vec<Rc<Type>>) -> Rc<Type> {
        Rc::new(Type::Tuple(types))
    }
    pub fn is_fully_known(&self) -> bool {
        use Type::*;
        match self {
            Unknown(_, _) => false,
            Func { lhs, rhs } => lhs.is_fully_known() && rhs.is_fully_known(),
            Ref(inner) => inner.is_fully_known(),
            Array { inner, dim_cnt } => {
                inner.is_fully_known() && matches!(*dim_cnt.borrow().borrow(), ArrayDims::Known(_))
            }
            Tuple(types) => types.iter().all(|t| t.is_fully_known()),
            _ => true,
        }
    }
    // #[inline(always)]
    // pub fn get_return_type(ty: &Rc<Type>) -> Rc<Type> {
    //     match &**ty {
    //         Type::Func { rhs, .. } => Self::get_return_type(rhs),
    //         _ => ty.clone(),
    //     }
    // }
}
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use Type::*;
        if std::mem::discriminant(self) != std::mem::discriminant(other) {
            return false;
        }
        match (self, other) {
            (Unknown(id1, _), Unknown(id2, _)) => id1 == id2,
            (
                Func {
                    lhs: lhs1,
                    rhs: rhs1,
                },
                Func {
                    lhs: lhs2,
                    rhs: rhs2,
                },
            ) => lhs1 == lhs2 && rhs1 == rhs2,
            (Ref(inner1), Ref(inner2)) => inner1 == inner2,
            (
                Array {
                    inner: i1,
                    dim_cnt: d1,
                },
                Array {
                    inner: i2,
                    dim_cnt: d2,
                },
            ) => i1 == i2 && d1 == d2,
            (Tuple(types1), Tuple(types2)) => types2 == types1,
            (Custom { id: id1 }, Custom { id: id2 }) => id1 == id2,
            _ => true,
        }
    }
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
            Array { inner, dim_cnt } => Type::new_known_array((&**inner).into(), *dim_cnt),
            Tuple(types) => Rc::new(Type::Tuple(types.iter().map(|t| (t).into()).collect())),
            Custom { id } => Rc::new(Type::Custom { id: id.clone() }),
        }
    }
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            Type::Unknown(id, constraints) => {
                format!("'{}{}", Type::unknown_id_to_name(*id), constraints.borrow())
            }
            Type::Unit => "unit".to_string(),
            Type::Int => "int".to_string(),
            Type::Char => "char".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Float => "float".to_string(),
            Type::Func { lhs, rhs } => format!("{} -> ({})", lhs, rhs),
            Type::Ref(inner) => format!("({} ref)", inner),
            Type::Array { inner, dim_cnt } => format!("{}[{}]", inner, dim_cnt.borrow().borrow()),
            Type::Tuple(types) => {
                format!(
                    "({})",
                    types
                        .iter()
                        .map(|t| format!("{}", t))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Type::Custom { id } => format!("{}", id),
        };
        write!(f, "{}", type_str)
    }
}
