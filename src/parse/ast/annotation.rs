use crate::lex::token::TokenKind;

#[derive(Debug, Clone)]
pub enum Type {
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
    /// If the vector contains only one element, return that element.
    /// Otherwise, return a tuple.
    pub fn maybe_tuple(types: Vec<Type>) -> Self {
        if types.len() == 1 {
            types.into_iter().next().expect("Tuple with 1 element")
        } else {
            Self::Tuple(types)
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
