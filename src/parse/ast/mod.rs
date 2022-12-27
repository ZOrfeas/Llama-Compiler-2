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
pub struct Letdef {}
#[derive(Debug)]
pub struct Def {}

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
