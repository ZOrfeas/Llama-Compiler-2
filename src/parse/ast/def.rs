use super::{annotation::Type, expr::Expr};

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
pub struct Par {
    pub id: String,
    pub type_: Option<Type>,
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
