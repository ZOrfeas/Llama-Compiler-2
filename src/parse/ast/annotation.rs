use crate::lex::token::TokenKind;

use strum::Display;
use strum::EnumDiscriminants;

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumDiscriminants)]
#[strum_discriminants(derive(Display))]
#[strum_discriminants(name(TypeKind))]
pub enum Type {
    Unknown(u32),
    Unit,
    Int,
    Char,
    Bool,
    Float,
    Func { lhs: Box<Type>, rhs: Box<Type> },
    Ref(Box<Type>),
    Array { inner: Box<Type>, dim_cnt: i32 },
    Tuple(Vec<Type>),
    Custom { id: String },
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
    /// If the vector contains only one element, return that element.
    /// Otherwise, return a tuple.
    pub fn maybe_tuple(types: Vec<Type>) -> Self {
        if types.len() == 1 {
            types
                .into_iter()
                .next()
                .expect("maybe_tuple called with empty vector")
        } else {
            Self::Tuple(types)
        }
    }
    pub fn is_fully_known(&self) -> bool {
        match self {
            Self::Unknown(_) => false,
            Self::Unit | Self::Int | Self::Char | Self::Bool | Self::Float => true,
            Self::Func { lhs, rhs } => lhs.is_fully_known() && rhs.is_fully_known(),
            Self::Ref(inner) => inner.is_fully_known(),
            Self::Array { inner, .. } => inner.is_fully_known(),
            Self::Tuple(types) => types.iter().all(|t| t.is_fully_known()),
            Self::Custom { .. } => true,
        }
    }
}

impl From<&TokenKind> for Type {
    fn from(token_kind: &TokenKind) -> Self {
        match token_kind {
            TokenKind::Unit => Self::Unit,
            TokenKind::Int => Self::Int,
            TokenKind::Char => Self::Char,
            TokenKind::Bool => Self::Bool,
            TokenKind::Float => Self::Float,
            _ => panic!("Cannot convert token to type"),
        }
    }
}
