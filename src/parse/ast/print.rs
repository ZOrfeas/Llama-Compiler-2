use ptree::TreeItem;

use super::*;

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
    Clause(&'a Clause),
    Pattern(&'a Pattern),
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
                Node::Program(p) => format!(
                    "Program with {} statement{}",
                    p.definitions.len(),
                    if p.definitions.len() == 1 { "" } else { "s" }
                ),
                Node::Definition(d) => match d {
                    Definition::Let(l) => format!(
                        "{}let statement with {} definition{}",
                        if l.rec { "Recursive " } else { "" },
                        l.defs.len(),
                        if l.defs.len() == 1 { "" } else { "s" }
                    ),
                    Definition::Type(t) => format!(
                        "type statement with {} type definition{}",
                        t.tdefs.len(),
                        if t.tdefs.len() == 1 { "" } else { "s" }
                    ),
                },
                Node::Def(d) => {
                    let (str, type_) = match d {
                        Def::Const(c) => (format!("Constant {}", c.id), &c.type_),
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
                Node::TDef(t) => format!("Type {}", t.id),
                Node::Constr(c) => format!("Constructor {}", c.id),
                Node::Type(t) => format!("{}", t),
                Node::Par(p) => format!("Parameter {}", p.id),
                Node::Expr(e) => match e {
                    Expr::UnitLiteral => "Unit Literal".to_string(),
                    Expr::IntLiteral(i) => format!("Int Literal '{}'", i),
                    Expr::FloatLiteral(f) => format!("Float Literal '{}'", f),
                    Expr::CharLiteral(c) => format!("Char Literal '{}'", c),
                    Expr::StringLiteral(s) => format!("String Literal '{}'", s),
                    Expr::BoolLiteral(b) => format!("Bool Literal '{}'", b),
                    Expr::Tuple(ts) => format!(
                        "Tuple with {} element{}",
                        ts.len(),
                        if ts.len() == 1 { "" } else { "s" }
                    ),
                    Expr::Unop(op, _) => format!("Unary Operation '{:?}'", op),
                    Expr::Binop(op, _, _) => format!("Binary Operation '{:?}'", op),
                    Expr::Call(id, args) => format!(
                        "Call to {} with {} argument{}",
                        id,
                        args.len(),
                        if args.len() == 1 { "" } else { "s" }
                    ),
                    Expr::ConstrCall(id, args) => format!(
                        "Constructor call to {} with {} argument{}",
                        id,
                        args.len(),
                        if args.len() == 1 { "" } else { "s" }
                    ),
                    Expr::ArrayAccess(id, args) => format!(
                        "Array access to {} with {} argument{}",
                        id,
                        args.len(),
                        if args.len() == 1 { "" } else { "s" }
                    ),
                    Expr::Dim(id, arg) => format!("dim call for id {} and dimension {}", id, arg),
                    Expr::New(t) => format!("New on type {}", t),
                    Expr::LetIn(_, _) => format!("Let In expression"),
                    Expr::If(_, _, _) => format!("If expression"),
                    Expr::While(_, _) => format!("While expression"),
                    Expr::For(id, _, ascending, _, _) => format!(
                        "For expression with id {} and {}",
                        id,
                        if *ascending {
                            "ascending order"
                        } else {
                            "descending order"
                        }
                    ),
                    Expr::Match(_, clauses) => format!(
                        "Match expression with {} clause{}",
                        clauses.len(),
                        if clauses.len() == 1 { "" } else { "s" }
                    ),
                },
                Node::Clause(_) => format!("Clause"),
                Node::Pattern(p) => match p {
                    Pattern::IntLiteral(i) => format!("Int Literal '{}'", i),
                    Pattern::FloatLiteral(f) => format!("Float Literal '{}'", f),
                    Pattern::CharLiteral(c) => format!("Char Literal '{}'", c),
                    Pattern::StringLiteral(s) => format!("String Literal '{}'", s),
                    Pattern::BoolLiteral(b) => format!("Bool Literal '{}'", b),
                    Pattern::IdLower(id) => format!("Name binding on '{}'", id),
                    Pattern::IdUpper(id, patterns) => format!(
                        "Pattern matching on constructor '{}' with {} argument{}",
                        id,
                        patterns.len(),
                        if patterns.len() == 1 { "" } else { "s" }
                    ),
                    Pattern::Tuple(ps) => format!(
                        "Tuple with {} element{}",
                        ps.len(),
                        if ps.len() == 1 { "" } else { "s" }
                    ),
                },
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
                Def::Variable(_) => Vec::new(),
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
            Node::Type(_) => Vec::new(),
            Node::Par(p) => {
                let mut vec = Vec::new();
                if let Some(t) = &p.type_ {
                    vec.push(Node::Type(t));
                }
                vec
            }
            Node::Expr(e) => match e {
                Expr::UnitLiteral
                | Expr::IntLiteral(_)
                | Expr::FloatLiteral(_)
                | Expr::CharLiteral(_)
                | Expr::StringLiteral(_)
                | Expr::BoolLiteral(_) => Vec::new(),
                Expr::Tuple(ts) => ts.iter().map(|e| Node::Expr(e)).collect(),
                Expr::Unop(_, e) => vec![Node::Expr(e)],
                Expr::Binop(_, e1, e2) => vec![Node::Expr(e1), Node::Expr(e2)],
                Expr::Call(_, args) => args.iter().map(|e| Node::Expr(e)).collect(),
                Expr::ConstrCall(_, args) => args.iter().map(|e| Node::Expr(e)).collect(),
                Expr::ArrayAccess(_, args) => args.iter().map(|e| Node::Expr(e)).collect(),
                Expr::Dim(_, _) => Vec::new(),
                Expr::New(_) => Vec::new(),
                Expr::LetIn(letdef, e) => {
                    let mut vec = Vec::new();
                    vec.extend(letdef.defs.iter().map(|d| Node::Def(d)));
                    vec.push(Node::Expr(e));
                    vec
                }
                Expr::If(e1, e2, e3) => {
                    let mut v = vec![Node::Expr(e1), Node::Expr(e2)];
                    if let Some(e) = e3 {
                        v.push(Node::Expr(e));
                    }
                    v
                }
                Expr::While(e1, e2) => vec![Node::Expr(e1), Node::Expr(e2)],
                Expr::For(_, e1, _, e2, e3) => {
                    vec![Node::Expr(e1), Node::Expr(e2), Node::Expr(e3)]
                }
                Expr::Match(e, clauses) => {
                    let mut vec = Vec::new();
                    vec.push(Node::Expr(e));
                    vec.extend(clauses.iter().map(|c| Node::Clause(c)));
                    vec
                }
            },
            Node::Clause(c) => vec![Node::Pattern(&c.pattern), Node::Expr(&c.expr)],
            Node::Pattern(p) => match p {
                Pattern::IntLiteral(_)
                | Pattern::FloatLiteral(_)
                | Pattern::CharLiteral(_)
                | Pattern::StringLiteral(_)
                | Pattern::BoolLiteral(_) => Vec::new(),
                Pattern::IdLower(_) => Vec::new(),
                Pattern::IdUpper(_, patterns) => {
                    patterns.iter().map(|p| Node::Pattern(p)).collect()
                }
                Pattern::Tuple(ps) => ps.iter().map(|p| Node::Pattern(p)).collect(),
            },
        })
    }
}
