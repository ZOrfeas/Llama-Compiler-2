#[derive(Debug)]
pub struct Program {
    pub definitions: Vec<Definition>,
}

#[derive(Debug)]
pub enum Definition {
    Let(Letdef),
    Type(Typedef),
}
#[derive(Debug)]
pub struct Letdef {
    pub rec: bool,
    pub defs: Vec<Def>,
}

#[derive(Debug)]
pub enum Def {
    Const(ConstDef),
    Variable(VariableDef),
    Array(ArrayDef),
    Function(FunctionDef),
}
#[derive(Debug)]
pub struct ConstDef {
    pub id: String,
    pub type_: Option<Type>,
    pub expr: Expr,
}
#[derive(Debug)]
pub struct VariableDef {
    pub id: String,
    pub type_: Option<Type>,
}
#[derive(Debug)]
pub struct ArrayDef {
    pub id: String,
    pub type_: Option<Type>,
    pub dims: Vec<Expr>,
}
#[derive(Debug)]
pub struct FunctionDef {
    pub id: String,
    pub pars: Vec<Par>,
    pub type_: Option<Type>,
    pub expr: Expr,
}
#[derive(Debug)]
pub struct Typedef {
    pub tdefs: Vec<TDef>,
}
#[derive(Debug)]
pub struct TDef {
    pub id: String,
    pub constrs: Vec<Constr>,
}
#[derive(Debug)]
pub struct Constr {
    pub id: String,
    pub types: Vec<Type>,
}

#[derive(Debug)]
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
#[derive(Debug)]
pub struct Par {
    pub id: String,
    pub type_: Option<Type>,
}
#[derive(Debug)]
pub struct Expr {}
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
