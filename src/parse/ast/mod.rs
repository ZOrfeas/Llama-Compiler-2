pub mod print;

use std::borrow::Cow;

use crate::lex::token::TokenKind;

#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: Vec<Definition>,
}
impl Program {
    pub fn print(&self, w: impl std::io::Write) -> std::io::Result<()> {
        // let config = ptree::PrintConfig::from_env();
        // ptree::write_tree_with(&Node::Program(&self), w, &config)
        ptree::write_tree(&print::Node::Program(&self), w)
    }
}

#[derive(Debug, Clone)]
pub enum Definition {
    Let(Letdef),
    Type(Typedef),
}
#[derive(Debug, Clone)]
pub struct Letdef {
    pub rec: bool,
    pub defs: Vec<Def>,
}

#[derive(Debug, Clone)]
pub enum Def {
    Const(ConstDef),
    Variable(VariableDef),
    Array(ArrayDef),
    Function(FunctionDef),
}
#[derive(Debug, Clone)]
pub struct ConstDef {
    pub id: String,
    pub type_: Option<Type>,
    pub expr: Expr,
}
#[derive(Debug, Clone)]
pub struct VariableDef {
    pub id: String,
    pub type_: Option<Type>,
}
#[derive(Debug, Clone)]
pub struct ArrayDef {
    pub id: String,
    pub type_: Option<Type>,
    pub dims: Vec<Expr>,
}
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub id: String,
    pub pars: Vec<Par>,
    pub type_: Option<Type>,
    pub expr: Expr,
}
#[derive(Debug, Clone)]
pub struct Typedef {
    pub tdefs: Vec<TDef>,
}
#[derive(Debug, Clone)]
pub struct TDef {
    pub id: String,
    pub constrs: Vec<Constr>,
}
#[derive(Debug, Clone)]
pub struct Constr {
    pub id: String,
    pub types: Vec<Type>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Unit,
    Int,
    Char,
    Bool,
    Float,
    Func(Box<Type>, Box<Type>),
    Ref(Box<Type>),
    Array(Box<Type>, i32),
    Tuple(Vec<Type>),
    Custom(String),
}
#[derive(Debug, Clone)]
pub struct Par {
    pub id: String,
    pub type_: Option<Type>,
}
#[derive(Debug, Clone)]
pub enum Expr {
    UnitLiteral,
    IntLiteral(i32),
    FloatLiteral(f64),
    CharLiteral(u8),
    StringLiteral(String),
    BoolLiteral(bool),
    Tuple(Vec<Expr>),
    Unop(Unop, Box<Expr>),
    Binop(Binop, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    ConstrCall(String, Vec<Expr>),
    ArrayAccess(String, Vec<Expr>),
    Dim(String, i32),
    New(Type),
    LetIn(Letdef, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    While(Box<Expr>, Box<Expr>),
    For(String, Box<Expr>, bool, Box<Expr>, Box<Expr>),
    Match(Box<Expr>, Vec<Clause>),
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
    IdUpper(String, Vec<Pattern>),
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
            .reduce(|acc, e| Expr::Binop(binop.clone(), Box::new(acc), Box::new(e)))
            .expect("expression vector should not be empty")
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
