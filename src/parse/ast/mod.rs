use std::borrow::Cow;

use ptree::TreeItem;

#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: Vec<Definition>,
}
impl Program {
    pub fn print(&self, w: impl std::io::Write) -> std::io::Result<()> {
        // let config = ptree::PrintConfig::from_env();
        // ptree::write_tree_with(&Node::Program(&self), w, &config)
        ptree::write_tree(&Node::Program(&self), w)
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
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unit => write!(f, "unit"),
            Type::Int => write!(f, "int"),
            Type::Char => write!(f, "char"),
            Type::Bool => write!(f, "bool"),
            Type::Float => write!(f, "float"),
            Type::Func(t1, t2) => write!(f, "{} -> ({})", t1, t2),
            Type::Ref(t) => write!(f, "({} ref)", t),
            Type::Array(t, n) => write!(
                f,
                "{}[{}]",
                t,
                (0..*n).map(|_| "*").collect::<Vec<_>>().join(", ")
            ),
            Type::Tuple(ts) => {
                write!(
                    f,
                    "({})",
                    ts.iter()
                        .map(|t| format!("{}", t))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Type::Custom(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node<'a> {
    Program(&'a Program),
    Definition(&'a Definition),
    Def(&'a Def),
    TDef(&'a TDef),
    Constr(&'a Constr),
    Type(&'a Type),
    Par(&'a Par),
    Expr(&'a Expr),
}

impl<'a> TreeItem for Node<'a> {
    type Child = Node<'a>;
    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &ptree::Style,
    ) -> std::io::Result<()> {
        write!(
            f,
            "{}",
            style.paint(match self {
                Node::Program(p) => format!("Program with {} definitions", p.definitions.len()),
                Node::Definition(d) => match d {
                    Definition::Let(l) => format!(
                        "{}Letdef with {} defs",
                        if l.rec { "Rec " } else { "" },
                        l.defs.len()
                    ),
                    Definition::Type(t) => format!("Typedef with {} tdefs", t.tdefs.len()),
                },
                Node::Def(d) => {
                    let (str, type_) = match d {
                        Def::Const(c) => (format!("Const {}", c.id), &c.type_),
                        Def::Variable(v) => (format!("Variable {}", v.id), &v.type_),
                        Def::Array(a) => (format!("Array {}", a.id), &a.type_),
                        Def::Function(f) => (format!("Function {}", f.id), &f.type_),
                    };
                    if let Some(t) = type_ {
                        format!("{} of type {}", str, t)
                    } else {
                        str
                    }
                }
                Node::TDef(t) => format!("TDef {}", t.id),
                Node::Constr(c) => format!("Constr {}", c.id),
                Node::Type(t) => format!("{}", t),
                Node::Par(p) => format!("Par {}", p.id),
                Node::Expr(e) => todo!(),
            })
        )
    }
    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(match self {
            Node::Program(p) => p
                .definitions
                .iter()
                .map(|d| Node::Definition(d))
                .collect::<Vec<_>>(),
            Node::Definition(d) => match d {
                Definition::Let(l) => l.defs.iter().map(|d| Node::Def(d)).collect::<Vec<_>>(),
                Definition::Type(t) => t.tdefs.iter().map(|t| Node::TDef(t)).collect::<Vec<_>>(),
            },
            Node::Def(d) => match d {
                Def::Const(c) => vec![Node::Expr(&c.expr)],
                Def::Variable(v) => Vec::new(),
                Def::Array(a) => a.dims.iter().map(|e| Node::Expr(e)).collect(),
                Def::Function(fun) => {
                    let mut vec = Vec::new();
                    vec.extend(fun.pars.iter().map(|p| Node::Par(p)));
                    vec.push(Node::Expr(&fun.expr));
                    vec
                }
            },
            Node::TDef(t) => t
                .constrs
                .iter()
                .map(|c| Node::Constr(c))
                .collect::<Vec<_>>(),
            Node::Constr(c) => c.types.iter().map(|t| Node::Type(t)).collect::<Vec<_>>(),
            Node::Type(t) => Vec::new(),
            Node::Par(p) => {
                let mut vec = Vec::new();
                if let Some(t) = &p.type_ {
                    vec.push(Node::Type(t));
                }
                vec
            }
            Node::Expr(e) => todo!(),
        })
    }
}
