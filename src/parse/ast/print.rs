use std::borrow::Cow;

use ptree::TreeItem;

use super::{annotation::*, data_map::NodeRef, def::*, expr::*, *};

impl Program {
    pub fn print(&self, w: impl std::io::Write) -> std::io::Result<()> {
        // let config = ptree::PrintConfig::from_env();
        // ptree::write_tree_with(&NodeRef::Program(&self), w, &config)
        ptree::write_tree(&NodeRef::Program(&self), w)
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

impl<'a> TreeItem for NodeRef<'a> {
    type Child = NodeRef<'a>;
    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &ptree::Style,
    ) -> std::io::Result<()> {
        write!(
            f,
            "{}",
            style.paint(match self {
                NodeRef::Program(p) => format!(
                    "Program with {} statement{}",
                    p.definitions.len(),
                    if p.definitions.len() == 1 { "" } else { "s" }
                ),
                NodeRef::Definition(d) => match d {
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
                NodeRef::Def(d) => {
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
                NodeRef::TDef(t) => format!("Type {}", t.id),
                NodeRef::Constr(c) => format!("Constructor {}", c.id),
                NodeRef::Type(t) => format!("{}", t),
                NodeRef::Par(p) => format!(
                    "Parameter {}{}",
                    p.id,
                    if let Some(t) = &p.type_ {
                        format!(" of type {}", t)
                    } else {
                        "".to_string()
                    }
                ),
                NodeRef::Expr(e) => match e {
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
                NodeRef::Clause(_) => format!("Clause"),
                NodeRef::Pattern(p) => match p {
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
        let mut children: Vec<NodeRef> = self.children().unwrap_or(Vec::new());
        children.retain(|child| !matches!(child, NodeRef::Type(_)));
        Cow::from(children)
    }
}

impl<'a> NodeRef<'a> {
    fn children(&self) -> Option<Vec<Self>> {
        match self {
            NodeRef::Program(p) => Some(p.definitions.iter().map(NodeRef::Definition).collect()),
            NodeRef::Definition(d) => match d {
                Definition::Let(l) => Some(l.defs.iter().map(NodeRef::Def).collect()),
                Definition::Type(t) => Some(t.tdefs.iter().map(NodeRef::TDef).collect()),
            },
            NodeRef::Def(d) => match d {
                Def::Const(c) => {
                    let mut children = Vec::new();
                    if let Some(t) = &c.type_ {
                        children.push(NodeRef::Type(t));
                    }
                    children.push(NodeRef::Expr(&c.expr));
                    Some(children)
                }
                Def::Variable(v) => v.type_.as_ref().map(|t| vec![NodeRef::Type(t)]),
                Def::Array(a) => {
                    let mut children = Vec::new();
                    if let Some(t) = &a.type_ {
                        children.push(NodeRef::Type(t));
                    }
                    children.extend(a.dims.iter().map(NodeRef::Expr));
                    Some(children)
                }
                Def::Function(fun) => {
                    let mut children = Vec::new();
                    if let Some(t) = &fun.type_ {
                        children.push(NodeRef::Type(t));
                    }
                    children.extend(fun.pars.iter().map(|p| NodeRef::Par(p)));
                    children.push(NodeRef::Expr(&fun.expr));
                    Some(children)
                }
            },
            NodeRef::TDef(t) => Some(t.constrs.iter().map(NodeRef::Constr).collect()),
            NodeRef::Constr(c) => Some(c.types.iter().map(NodeRef::Type).collect()),
            NodeRef::Type(t) => match t {
                Type::Func { lhs, rhs } => Some(vec![NodeRef::Type(lhs), NodeRef::Type(rhs)]),
                Type::Ref(t) => Some(vec![NodeRef::Type(t)]),
                Type::Array { inner, .. } => Some(vec![NodeRef::Type(inner)]),
                Type::Tuple(ts) => Some(ts.iter().map(NodeRef::Type).collect()),
                _ => None,
            },
            NodeRef::Par(p) => p.type_.as_ref().map(|t| vec![NodeRef::Type(t)]),
            NodeRef::Expr(e) => match e {
                Expr::UnitLiteral
                | Expr::IntLiteral(_)
                | Expr::FloatLiteral(_)
                | Expr::CharLiteral(_)
                | Expr::StringLiteral(_)
                | Expr::BoolLiteral(_) => None,
                Expr::Tuple(ts) => Some(ts.iter().map(NodeRef::Expr).collect()),
                Expr::Unop { op: _, operand } => Some(vec![NodeRef::Expr(operand)]),
                Expr::Binop { lhs, op: _, rhs } => {
                    Some(vec![NodeRef::Expr(lhs), NodeRef::Expr(rhs)])
                }
                Expr::Call { id: _, args } | Expr::ConstrCall { id: _, args } => {
                    Some(args.iter().map(NodeRef::Expr).collect())
                }
                Expr::ArrayAccess { id: _, indexes } => {
                    Some(indexes.iter().map(NodeRef::Expr).collect())
                }
                Expr::Dim { id: _, dim: _ } => None,
                Expr::New(t) => Some(vec![NodeRef::Type(t)]),
                Expr::LetIn { letdef, expr } => {
                    let mut children = Vec::new();
                    children.extend(letdef.defs.iter().map(NodeRef::Def));
                    children.push(NodeRef::Expr(expr));
                    Some(children)
                }
                Expr::If {
                    cond,
                    then_body,
                    else_body,
                } => {
                    let mut children = vec![NodeRef::Expr(cond), NodeRef::Expr(then_body)];
                    if let Some(e) = else_body {
                        children.push(NodeRef::Expr(e));
                    }
                    Some(children)
                }
                Expr::While { cond, body } => Some(vec![NodeRef::Expr(cond), NodeRef::Expr(body)]),
                Expr::For {
                    id: _,
                    from,
                    ascending: _,
                    to,
                    body,
                } => Some(vec![
                    NodeRef::Expr(from),
                    NodeRef::Expr(to),
                    NodeRef::Expr(body),
                ]),
                Expr::Match { to_match, clauses } => {
                    let mut children = vec![NodeRef::Expr(to_match)];
                    children.extend(clauses.iter().map(NodeRef::Clause));
                    Some(children)
                }
            },
            NodeRef::Clause(c) => Some(vec![NodeRef::Pattern(&c.pattern), NodeRef::Expr(&c.expr)]),
            NodeRef::Pattern(p) => match p {
                Pattern::IntLiteral(_)
                | Pattern::FloatLiteral(_)
                | Pattern::CharLiteral(_)
                | Pattern::StringLiteral(_)
                | Pattern::BoolLiteral(_)
                | Pattern::IdLower(_) => None,
                Pattern::IdUpper { id: _, args } => {
                    Some(args.iter().map(NodeRef::Pattern).collect())
                }
                Pattern::Tuple(ps) => Some(ps.iter().map(NodeRef::Pattern).collect()),
            },
        }
    }
}
