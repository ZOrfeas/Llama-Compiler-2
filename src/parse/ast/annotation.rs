use crate::lex::token::TokenKind;

#[derive(Debug, Clone)]
pub enum TypeAnnotation {
    Unit,
    Int,
    Char,
    Bool,
    Float,
    Func {
        lhs: Box<TypeAnnotation>,
        rhs: Box<TypeAnnotation>,
    },
    Ref(Box<TypeAnnotation>),
    Array {
        inner: Box<TypeAnnotation>,
        dim_cnt: i32,
    },
    Tuple(Vec<TypeAnnotation>),
    Custom {
        id: String,
    },
}
impl TypeAnnotation {
    /// If the vector contains only one element, return that element.
    /// Otherwise, return a tuple.
    pub fn maybe_tuple(types: Vec<TypeAnnotation>) -> Self {
        if types.len() == 1 {
            types
                .into_iter()
                .next()
                .expect("maybe_tuple called with empty vector")
        } else {
            Self::Tuple(types)
        }
    }
    // pub fn unknown_id_to_name(mut u: u32) -> String {
    //     let mut acc = Vec::new();
    //     loop {
    //         let c = (u % 26) as u8 + b'a';
    //         acc.push(if acc.is_empty() { c } else { c - 1 });
    //         if u < 26 {
    //             break;
    //         }
    //         u = u / 26;
    //     }
    //     acc.into_iter().rev().map(|c| c as char).collect()
    // }
}

impl From<&TokenKind> for TypeAnnotation {
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
