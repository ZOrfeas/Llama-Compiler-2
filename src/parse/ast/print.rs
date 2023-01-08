use std::borrow::Cow;

use ptree::TreeItem;

use super::{annotation::*, def::*, expr::*, *};

impl Program {
    pub fn print(&self, w: impl std::io::Write) -> std::io::Result<()> {
        // let config = ptree::PrintConfig::from_env();
        // ptree::write_tree_with(&Node::Program(&self), w, &config)
        ptree::write_tree(&print::Node::Program(&self), w)
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unknown(id) => write!(f, "@{}", id),
            Type::Unit => write!(f, "unit"),
            Type::Int => write!(f, "int"),
            Type::Char => write!(f, "char"),
            Type::Bool => write!(f, "bool"),
            Type::Float => write!(f, "float"),
            Type::Func { lhs, rhs } => write!(f, "{} -> ({})", lhs, rhs),
            Type::Ref(t) => write!(f, "({} ref)", t),
            Type::Array { inner, dim_cnt } => write!(
                f,
                "{}[{}]",
                inner,
                (0..*dim_cnt).map(|_| "*").collect::<Vec<_>>().join(", ")
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
            Type::Custom { id } => write!(f, "{}", id),
        }
    }
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
                Node::Par(p) => format!(
                    "Parameter {}{}",
                    p.id,
                    if let Some(t) = &p.type_ {
                        format!(" of type {}", t)
                    } else {
                        "".to_string()
                    }
                ),
                Node::Expr(e) => match e {
                    Expr::UnitLiteral => "Unit Literal".to_string(),
                    Expr::IntLiteral(i) => format!("Int Literal '{}'", i),
                    Expr::FloatLiteral(f) => format!("Float Literal '{}'", f),
                    Expr::CharLiteral(c) => format!("Char Literal '{}'", c),
                    Expr::StringLiteral(s) =>
                        format!("String Literal '{}'", s.replace("\n", "\\n")),
                    Expr::BoolLiteral(b) => format!("Bool Literal '{}'", b),
                    Expr::Tuple(ts) => format!(
                        "Tuple with {} element{}",
                        ts.len(),
                        if ts.len() == 1 { "" } else { "s" }
                    ),
                    Expr::Unop { op, operand: _ } => format!("Unary Operation '{:?}'", op),
                    Expr::Binop { lhs: _, op, rhs: _ } => format!("Binary Operation '{:?}'", op),
                    Expr::Call { id, args } => format!(
                        "Call to {} with {} argument{}",
                        id,
                        args.len(),
                        if args.len() == 1 { "" } else { "s" }
                    ),
                    Expr::ConstrCall { id, args } => format!(
                        "Constructor call to {} with {} argument{}",
                        id,
                        args.len(),
                        if args.len() == 1 { "" } else { "s" }
                    ),
                    Expr::ArrayAccess { id, indexes } => format!(
                        "Array access to {} with {} argument{}",
                        id,
                        indexes.len(),
                        if indexes.len() == 1 { "" } else { "s" }
                    ),
                    Expr::Dim { id, dim } =>
                        format!("dim call for id {} and dimension {}", id, dim),
                    Expr::New(t) => format!("New on type {}", t),
                    Expr::LetIn { .. } => format!("Let In expression"),
                    Expr::If { .. } => format!("If expression"),
                    Expr::While { cond: _, body: _ } => format!("While expression"),
                    Expr::For {
                        id,
                        from: _,
                        ascending,
                        to: _,
                        body: _,
                    } => format!(
                        "For expression with id {} and {}",
                        id,
                        if *ascending {
                            "ascending order"
                        } else {
                            "descending order"
                        }
                    ),
                    Expr::Match {
                        to_match: _,
                        clauses,
                    } => format!(
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
                    Pattern::IdUpper { id, args } => format!(
                        "Pattern matching on constructor '{}' with {} argument{}",
                        id,
                        args.len(),
                        if args.len() == 1 { "" } else { "s" }
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
        let mut children: Vec<Node> = self.children().unwrap_or(Vec::new());
        children.retain(|child| !matches!(child, Node::Type(_)));
        Cow::from(children)
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
impl<'a> Node<'a> {
    fn children(&self) -> Option<Vec<Self>> {
        match self {
            Node::Program(p) => Some(p.definitions.iter().map(Node::Definition).collect()),
            Node::Definition(d) => match d {
                Definition::Let(l) => Some(l.defs.iter().map(Node::Def).collect()),
                Definition::Type(t) => Some(t.tdefs.iter().map(Node::TDef).collect()),
            },
            Node::Def(d) => match d {
                Def::Const(c) => {
                    let mut children = Vec::new();
                    if let Some(t) = &c.type_ {
                        children.push(Node::Type(t));
                    }
                    children.push(Node::Expr(&c.expr));
                    Some(children)
                }
                Def::Variable(v) => v.type_.as_ref().map(|t| vec![Node::Type(t)]),
                Def::Array(a) => {
                    let mut children = Vec::new();
                    if let Some(t) = &a.type_ {
                        children.push(Node::Type(t));
                    }
                    children.extend(a.dims.iter().map(Node::Expr));
                    Some(children)
                }
                Def::Function(fun) => {
                    let mut children = Vec::new();
                    if let Some(t) = &fun.type_ {
                        children.push(Node::Type(t));
                    }
                    children.extend(fun.pars.iter().map(|p| Node::Par(p)));
                    children.push(Node::Expr(&fun.expr));
                    Some(children)
                }
            },
            Node::TDef(t) => Some(t.constrs.iter().map(Node::Constr).collect()),
            Node::Constr(c) => Some(c.types.iter().map(Node::Type).collect()),
            Node::Type(t) => match t {
                Type::Func { lhs, rhs } => Some(vec![Node::Type(lhs), Node::Type(rhs)]),
                Type::Ref(t) => Some(vec![Node::Type(t)]),
                Type::Array { inner, .. } => Some(vec![Node::Type(inner)]),
                Type::Tuple(ts) => Some(ts.iter().map(Node::Type).collect()),
                _ => None,
            },
            Node::Par(p) => p.type_.as_ref().map(|t| vec![Node::Type(t)]),
            Node::Expr(e) => match e {
                Expr::UnitLiteral
                | Expr::IntLiteral(_)
                | Expr::FloatLiteral(_)
                | Expr::CharLiteral(_)
                | Expr::StringLiteral(_)
                | Expr::BoolLiteral(_) => None,
                Expr::Tuple(ts) => Some(ts.iter().map(Node::Expr).collect()),
                Expr::Unop { op: _, operand } => Some(vec![Node::Expr(operand)]),
                Expr::Binop { lhs, op: _, rhs } => Some(vec![Node::Expr(lhs), Node::Expr(rhs)]),
                Expr::Call { id: _, args } | Expr::ConstrCall { id: _, args } => {
                    Some(args.iter().map(Node::Expr).collect())
                }
                Expr::ArrayAccess { id: _, indexes } => {
                    Some(indexes.iter().map(Node::Expr).collect())
                }
                Expr::Dim { id: _, dim: _ } => None,
                Expr::New(t) => Some(vec![Node::Type(t)]),
                Expr::LetIn { letdef, expr } => {
                    let mut children = Vec::new();
                    children.extend(letdef.defs.iter().map(Node::Def));
                    children.push(Node::Expr(expr));
                    Some(children)
                }
                Expr::If {
                    cond,
                    then_body,
                    else_body,
                } => {
                    let mut children = vec![Node::Expr(cond), Node::Expr(then_body)];
                    if let Some(e) = else_body {
                        children.push(Node::Expr(e));
                    }
                    Some(children)
                }
                Expr::While { cond, body } => Some(vec![Node::Expr(cond), Node::Expr(body)]),
                Expr::For {
                    id: _,
                    from,
                    ascending: _,
                    to,
                    body,
                } => Some(vec![Node::Expr(from), Node::Expr(to), Node::Expr(body)]),
                Expr::Match { to_match, clauses } => {
                    let mut children = vec![Node::Expr(to_match)];
                    children.extend(clauses.iter().map(Node::Clause));
                    Some(children)
                }
            },
            Node::Clause(c) => Some(vec![Node::Pattern(&c.pattern), Node::Expr(&c.expr)]),
            Node::Pattern(p) => match p {
                Pattern::IntLiteral(_)
                | Pattern::FloatLiteral(_)
                | Pattern::CharLiteral(_)
                | Pattern::StringLiteral(_)
                | Pattern::BoolLiteral(_)
                | Pattern::IdLower(_) => None,
                Pattern::IdUpper { id: _, args } => Some(args.iter().map(Node::Pattern).collect()),
                Pattern::Tuple(ps) => Some(ps.iter().map(Node::Pattern).collect()),
            },
        }
    }
}
