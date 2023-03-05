use super::{annotation::Type, expr::Expr, Span};

#[derive(Debug, Clone)]
pub enum Definition {
    Let(Letdef),
    Type(Typedef),
}
#[derive(Debug, Clone)]
pub struct Letdef {
    pub rec: bool,
    pub defs: Vec<Def>,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub enum DefKind {
    Const { expr: Expr },
    Variable,
    Array { dims: Vec<Expr> },
    Function { pars: Vec<Par>, expr: Expr },
}
#[derive(Debug, Clone)]
pub struct Def {
    pub id: String,
    pub type_: Option<Type>,
    pub kind: DefKind,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub struct Par {
    pub id: String,
    pub type_: Option<Type>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Typedef {
    pub tdefs: Vec<TDef>,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub struct TDef {
    pub id: String,
    pub constrs: Vec<Constr>,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub struct Constr {
    pub id: String,
    pub types: Vec<Type>,
    pub span: Span,
}
