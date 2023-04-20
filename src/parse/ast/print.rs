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

impl std::fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Type::Unknown(id) => write!(f, "'{}", Type::unknown_id_to_name(*id)),
            TypeAnnotation::Unit => write!(f, "unit"),
            TypeAnnotation::Int => write!(f, "int"),
            TypeAnnotation::Char => write!(f, "char"),
            TypeAnnotation::Bool => write!(f, "bool"),
            TypeAnnotation::Float => write!(f, "float"),
            TypeAnnotation::Func { lhs, rhs } => write!(f, "{} -> ({})", lhs, rhs),
            TypeAnnotation::Ref(t) => write!(f, "({} ref)", t),
            TypeAnnotation::Array { inner, dim_cnt } => write!(
                f,
                "{}[{}]",
                inner,
                (0..*dim_cnt).map(|_| "*").collect::<Vec<_>>().join(", ")
            ),
            TypeAnnotation::Tuple(ts) => {
                write!(
                    f,
                    "({})",
                    ts.iter()
                        .map(|t| format!("{}", t))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            TypeAnnotation::Custom { id } => write!(f, "{}", id),
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
                    let def_type = match &d.kind {
                        DefKind::Const { .. } => "Constant",
                        DefKind::Variable => "Variable",
                        DefKind::Array { .. } => "Array",
                        DefKind::Function { .. } => "Function",
                    };
                    format!(
                        "{} {}{}",
                        def_type,
                        d.id,
                        d.type_
                            .as_ref()
                            .map_or("".to_string(), |t| format!(" annotated '{}'", t))
                    )
                }
                NodeRef::TDef(t) => format!("Type {}", t.id),
                NodeRef::Constr(c) => format!("Constructor {}", c.id),
                NodeRef::Type(t) => format!("{}", t),
                NodeRef::Par(p) => format!(
                    "Parameter {}{}",
                    p.id,
                    if let Some(t) = &p.type_ {
                        format!(" annotated '{}'", t)
                    } else {
                        "".to_string()
                    }
                ),
                NodeRef::Expr(e) => match &e.kind {
                    ExprKind::UnitLiteral => "Unit Literal".to_string(),
                    ExprKind::IntLiteral(i) => format!("Int Literal '{}'", i),
                    ExprKind::FloatLiteral(f) => format!("Float Literal '{}'", f),
                    ExprKind::CharLiteral(c) => format!("Char Literal '{}'", c),
                    ExprKind::StringLiteral(s) =>
                        format!("String Literal '{}'", s.replace("\n", "\\n")),
                    ExprKind::BoolLiteral(b) => format!("Bool Literal '{}'", b),
                    ExprKind::Tuple(ts) => format!(
                        "Tuple with {} element{}",
                        ts.len(),
                        if ts.len() == 1 { "" } else { "s" }
                    ),
                    ExprKind::Unop(Unop { op, operand: _ }) => format!("Unary Operation '{}'", op),
                    ExprKind::Binop(Binop { lhs: _, op, rhs: _ }) =>
                        format!("Binary Operation '{}'", op),
                    ExprKind::Call(Call { id, args }) => format!(
                        "Call to {} with {} argument{}",
                        id,
                        args.len(),
                        if args.len() == 1 { "" } else { "s" }
                    ),
                    ExprKind::ConstrCall(Call { id, args }) => format!(
                        "Constructor call to {} with {} argument{}",
                        id,
                        args.len(),
                        if args.len() == 1 { "" } else { "s" }
                    ),
                    ExprKind::ArrayAccess(ArrayAccess { id, indexes }) => format!(
                        "Array access to {} with {} argument{}",
                        id,
                        indexes.len(),
                        if indexes.len() == 1 { "" } else { "s" }
                    ),
                    ExprKind::Dim(Dim { id, dim }) =>
                        format!("dim call for id {} and dimension {}", id, dim),
                    ExprKind::New(t) => format!("New on type {}", t),
                    ExprKind::LetIn { .. } => format!("Let In expression"),
                    ExprKind::If { .. } => format!("If expression"),
                    ExprKind::While(While { cond: _, body: _ }) => format!("While expression"),
                    ExprKind::For(For {
                        id,
                        from: _,
                        ascending,
                        to: _,
                        body: _,
                    }) => format!(
                        "For expression with id {} and {}",
                        id,
                        if *ascending {
                            "ascending order"
                        } else {
                            "descending order"
                        }
                    ),
                    ExprKind::Match(Match {
                        to_match: _,
                        clauses,
                    }) => format!(
                        "Match expression with {} clause{}",
                        clauses.len(),
                        if clauses.len() == 1 { "" } else { "s" }
                    ),
                },
                NodeRef::For(_) => panic!("For should not be a TreeItem"),
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
            NodeRef::Def(d) => match &d.kind {
                DefKind::Const { expr } => {
                    let mut children = Vec::new();
                    if let Some(t) = &d.type_ {
                        children.push(NodeRef::Type(t));
                    }
                    children.push(NodeRef::Expr(expr));
                    Some(children)
                }
                DefKind::Variable => d.type_.as_ref().map(|t| vec![NodeRef::Type(t)]),
                DefKind::Array { dims } => {
                    let mut children = Vec::new();
                    if let Some(t) = &d.type_ {
                        children.push(NodeRef::Type(t));
                    }
                    children.extend(dims.iter().map(NodeRef::Expr));
                    Some(children)
                }
                DefKind::Function { expr, pars } => {
                    let mut children = Vec::new();
                    if let Some(t) = &d.type_ {
                        children.push(NodeRef::Type(t));
                    }
                    children.extend(pars.iter().map(|p| NodeRef::Par(p)));
                    children.push(NodeRef::Expr(expr));
                    Some(children)
                }
            },
            NodeRef::TDef(t) => Some(t.constrs.iter().map(NodeRef::Constr).collect()),
            NodeRef::Constr(c) => Some(c.types.iter().map(NodeRef::Type).collect()),
            NodeRef::Type(t) => match t {
                TypeAnnotation::Func { lhs, rhs } => {
                    Some(vec![NodeRef::Type(lhs), NodeRef::Type(rhs)])
                }
                TypeAnnotation::Ref(t) => Some(vec![NodeRef::Type(t)]),
                TypeAnnotation::Array { inner, .. } => Some(vec![NodeRef::Type(inner)]),
                TypeAnnotation::Tuple(ts) => Some(ts.iter().map(NodeRef::Type).collect()),
                _ => None,
            },
            NodeRef::Par(p) => p.type_.as_ref().map(|t| vec![NodeRef::Type(t)]),
            NodeRef::Expr(e) => match &e.kind {
                ExprKind::UnitLiteral
                | ExprKind::IntLiteral(_)
                | ExprKind::FloatLiteral(_)
                | ExprKind::CharLiteral(_)
                | ExprKind::StringLiteral(_)
                | ExprKind::BoolLiteral(_) => None,
                ExprKind::Tuple(ts) => Some(ts.iter().map(NodeRef::Expr).collect()),
                ExprKind::Unop(Unop { op: _, operand }) => Some(vec![NodeRef::Expr(operand)]),
                ExprKind::Binop(Binop { lhs, op: _, rhs }) => {
                    Some(vec![NodeRef::Expr(lhs), NodeRef::Expr(rhs)])
                }
                ExprKind::Call(Call { id: _, args })
                | ExprKind::ConstrCall(Call { id: _, args }) => {
                    Some(args.iter().map(NodeRef::Expr).collect())
                }
                ExprKind::ArrayAccess(ArrayAccess { id: _, indexes }) => {
                    Some(indexes.iter().map(NodeRef::Expr).collect())
                }
                ExprKind::Dim(Dim { id: _, dim: _ }) => None,
                ExprKind::New(t) => Some(vec![NodeRef::Type(t)]),
                ExprKind::LetIn(LetIn { letdef, expr }) => {
                    let mut children = Vec::new();
                    children.extend(letdef.defs.iter().map(NodeRef::Def));
                    children.push(NodeRef::Expr(expr));
                    Some(children)
                }
                ExprKind::If(If {
                    cond,
                    then_body,
                    else_body,
                }) => {
                    let mut children = vec![NodeRef::Expr(cond), NodeRef::Expr(then_body)];
                    if let Some(e) = else_body {
                        children.push(NodeRef::Expr(e));
                    }
                    Some(children)
                }
                ExprKind::While(While { cond, body }) => {
                    Some(vec![NodeRef::Expr(cond), NodeRef::Expr(body)])
                }
                ExprKind::For(For {
                    id: _,
                    from,
                    ascending: _,
                    to,
                    body,
                }) => Some(vec![
                    NodeRef::Expr(from),
                    NodeRef::Expr(to),
                    NodeRef::Expr(body),
                ]),
                ExprKind::Match(Match { to_match, clauses }) => {
                    let mut children = vec![NodeRef::Expr(to_match)];
                    children.extend(clauses.iter().map(NodeRef::Clause));
                    Some(children)
                }
            },
            NodeRef::For(_) => None,
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
impl<'a> std::fmt::Display for NodeRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_self(&mut DisplayWriter(f), &Default::default())
            .map_err(|_| std::fmt::Error)
    }
}

struct DisplayWriter<'a, 'b>(&'a mut std::fmt::Formatter<'b>);

impl<'a, 'b> std::io::Write for DisplayWriter<'a, 'b> {
    fn write(&mut self, bytes: &[u8]) -> std::result::Result<usize, std::io::Error> {
        self.0
            .write_str(&String::from_utf8_lossy(bytes))
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        todo!()
    }
}
