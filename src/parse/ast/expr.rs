use crate::lex::token::TokenKind;

use super::{annotation::Type, def::Letdef};

#[derive(Debug, Clone)]
pub enum Expr {
    UnitLiteral,
    IntLiteral(i32),
    FloatLiteral(f64),
    CharLiteral(u8),
    StringLiteral(String),
    BoolLiteral(bool),
    Tuple(Vec<Expr>),
    Unop {
        op: Unop,
        operand: Box<Expr>,
    },
    Binop {
        lhs: Box<Expr>,
        op: Binop,
        rhs: Box<Expr>,
    },
    Call {
        id: String,
        args: Vec<Expr>,
    },
    ConstrCall {
        id: String,
        args: Vec<Expr>,
    },
    ArrayAccess {
        id: String,
        indexes: Vec<Expr>,
    },
    Dim {
        id: String,
        dim: i32,
    },
    New(Type),
    LetIn {
        letdef: Letdef,
        expr: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then_body: Box<Expr>,
        else_body: Option<Box<Expr>>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Expr>,
    },
    For {
        id: String,
        from: Box<Expr>,
        ascending: bool,
        to: Box<Expr>,
        body: Box<Expr>,
    },
    Match {
        to_match: Box<Expr>,
        clauses: Vec<Clause>,
    },
}
#[derive(Debug, Clone)]
pub enum Unop {
    Plus,
    Minus,
    Deref,
    Not,
    PlusFlt,
    MinusFlt,
    Delete,
}
#[derive(Debug, Clone)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    StrEq,
    StrNotEq,
    NatEq,
    NatNotEq,
    Lt,
    Gt,
    LEq,
    GEq,
    And,
    Or,
    Semicolon,
    Assign,
    AddFlt,
    SubFlt,
    MulFlt,
    DivFlt,
}
#[derive(Debug, Clone)]
pub struct Clause {
    pub pattern: Pattern,
    pub expr: Expr,
}
#[derive(Debug, Clone)]
pub enum Pattern {
    IntLiteral(i32),
    FloatLiteral(f64),
    CharLiteral(u8),
    StringLiteral(String),
    BoolLiteral(bool),
    IdLower(String),
    Tuple(Vec<Pattern>),
    IdUpper { id: String, args: Vec<Pattern> },
}
impl Pattern {
    pub fn maybe_tuple(patterns: Vec<Pattern>) -> Self {
        if patterns.len() == 1 {
            patterns.into_iter().next().expect("Tuple with 1 element")
        } else {
            Self::Tuple(patterns)
        }
    }
}
impl Expr {
    pub fn maybe_tuple(exprs: Vec<Expr>) -> Self {
        if exprs.len() == 1 {
            exprs
                .into_iter()
                .next()
                .expect("Vector should not be empty")
        } else {
            Self::Tuple(exprs)
        }
    }
    pub fn left_assoc_from_vec(exprs: Vec<Expr>, token_kind: &TokenKind) -> Self {
        let binop = Binop::from(token_kind);
        exprs
            .into_iter()
            .reduce(|acc, e| Expr::Binop {
                lhs: Box::new(acc),
                op: binop.clone(),
                rhs: Box::new(e),
            })
            .expect("expression vector should not be empty")
    }
}
impl From<&TokenKind> for Unop {
    fn from(token_kind: &TokenKind) -> Self {
        match token_kind {
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Exclam => Self::Deref,
            TokenKind::Not => Self::Not,
            TokenKind::PlusDot => Self::PlusFlt,
            TokenKind::MinusDot => Self::MinusFlt,
            TokenKind::Delete => Self::Delete,
            _ => panic!("Cannot convert token to unop"),
        }
    }
}
impl From<&TokenKind> for Binop {
    fn from(token_kind: &TokenKind) -> Self {
        match token_kind {
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Sub,
            TokenKind::Star => Self::Mul,
            TokenKind::Slash => Self::Div,
            TokenKind::Mod => Self::Mod,
            TokenKind::DblStar => Self::Pow,
            TokenKind::Eq => Self::StrEq,
            TokenKind::LtGt => Self::StrNotEq,
            TokenKind::DblEq => Self::NatEq,
            TokenKind::ExclamEq => Self::NatNotEq,
            TokenKind::Lt => Self::Lt,
            TokenKind::Gt => Self::Gt,
            TokenKind::LEq => Self::LEq,
            TokenKind::GEq => Self::GEq,
            TokenKind::DblAmpersand => Self::And,
            TokenKind::DblBar => Self::Or,
            TokenKind::Semicolon => Self::Semicolon,
            TokenKind::ColonEq => Self::Assign,
            TokenKind::PlusDot => Self::AddFlt,
            TokenKind::MinusDot => Self::SubFlt,
            TokenKind::StarDot => Self::MulFlt,
            TokenKind::SlashDot => Self::DivFlt,
            _ => panic!("Cannot convert token to binop"),
        }
    }
}
