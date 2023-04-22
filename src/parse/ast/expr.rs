use strum::Display;

use crate::lex::token::{Token, TokenKind};

use super::{annotation::TypeAnnotation, def::Letdef, Span};

#[derive(Debug, Clone)]
pub enum ExprKind {
    UnitLiteral,
    IntLiteral(i32),
    FloatLiteral(f64),
    CharLiteral(u8),
    StringLiteral(String),
    BoolLiteral(bool),
    Tuple(Vec<Expr>),
    Unop(Unop),
    Binop(Binop),
    Call(Call),
    ConstrCall(Call),
    ArrayAccess(ArrayAccess),
    Dim(Dim),
    New(TypeAnnotation),
    LetIn(LetIn),
    If(If),
    While(While),
    For(For),
    Match(Match),
}
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub struct Unop {
    pub op: UnopKind,
    pub operand: Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct Binop {
    pub lhs: Box<Expr>,
    pub op: BinopKind,
    pub rhs: Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct Call {
    pub id: String,
    pub args: Vec<Expr>,
}
#[derive(Debug, Clone)]
pub struct ArrayAccess {
    pub id: String,
    pub indexes: Vec<Expr>,
}
#[derive(Debug, Clone)]
pub struct Dim {
    pub id: String,
    pub dim: i32,
}
#[derive(Debug, Clone)]
pub struct LetIn {
    pub letdef: Letdef,
    pub expr: Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct If {
    pub cond: Box<Expr>,
    pub then_body: Box<Expr>,
    pub else_body: Option<Box<Expr>>,
}
#[derive(Debug, Clone)]
pub struct While {
    pub cond: Box<Expr>,
    pub body: Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct For {
    pub id: String,
    pub from: Box<Expr>,
    pub ascending: bool,
    pub to: Box<Expr>,
    pub body: Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct Match {
    pub to_match: Box<Expr>,
    pub clauses: Vec<Clause>,
}
#[derive(Debug, Clone, Display)]
pub enum UnopKind {
    Plus,
    Minus,
    Deref,
    Not,
    // PlusFlt,
    // MinusFlt,
    Delete,
}
#[derive(Debug, Clone, Display)]
pub enum BinopKind {
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
    // AddFlt,
    // SubFlt,
    // MulFlt,
    // DivFlt,
}
#[derive(Debug, Clone)]
pub struct Clause {
    pub pattern: Pattern,
    pub expr: Expr,
}
#[derive(Debug, Clone)]
pub struct Pattern {
    pub kind: PatternKind,
    pub span: Span,
}
#[derive(Debug, Clone)]
pub enum PatternKind {
    IntLiteral(i32),
    FloatLiteral(f64),
    CharLiteral(u8),
    StringLiteral(String),
    BoolLiteral(bool),
    IdLower(String),
    Tuple(Vec<Pattern>),
    IdUpper { id: String, args: Vec<Pattern> },
}
impl Pattern {
    pub fn maybe_tuple(patterns: Vec<Pattern>) -> Self {
        if patterns.len() == 1 {
            patterns.into_iter().next().expect("Tuple with 1 element")
        } else {
            Self {
                span: Span::default(),
                kind: PatternKind::Tuple(patterns),
            }
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
            Self {
                kind: ExprKind::Tuple(exprs),
                span: Default::default(), // caller should set span
            }
        }
    }
    pub fn left_assoc_from_vec(exprs: Vec<Expr>, token_kind: &TokenKind) -> Self {
        let binop = BinopKind::from(token_kind);
        exprs
            .into_iter()
            .reduce(|acc, e| Expr {
                span: Span::new(acc.span.start.clone(), e.span.end.clone()),
                kind: ExprKind::Binop(Binop {
                    lhs: Box::new(acc),
                    op: binop.clone(),
                    rhs: Box::new(e),
                }),
            })
            .expect("expression vector should not be empty")
    }
    pub fn from_literal(token: Token) -> Self {
        match &token.kind {
            TokenKind::IntLiteral => {
                let (span, val) = token.into_span_and_value::<i32>();
                Expr {
                    kind: ExprKind::IntLiteral(val),
                    span,
                }
            }
            TokenKind::FloatLiteral => {
                let (span, val) = token.into_span_and_value::<f64>();
                Expr {
                    kind: ExprKind::FloatLiteral(val),
                    span,
                }
            }
            TokenKind::CharLiteral => {
                let (span, val) = token.into_span_and_value::<u8>();
                Expr {
                    kind: ExprKind::CharLiteral(val),
                    span,
                }
            }
            TokenKind::StringLiteral => {
                let (span, val) = token.into_span_and_value::<String>();
                Expr {
                    kind: ExprKind::StringLiteral(val),
                    span,
                }
            }
            TokenKind::True | TokenKind::False => {
                let span = Span::new(token.from, token.to);
                Expr {
                    kind: ExprKind::BoolLiteral(match &token.kind {
                        TokenKind::True => true,
                        TokenKind::False => false,
                        _ => panic!("Cannot convert token to bool"),
                    }),
                    span,
                }
            }
            _ => panic!("Cannot convert token to literal"),
        }
    }
}
impl From<&TokenKind> for UnopKind {
    fn from(token_kind: &TokenKind) -> Self {
        match token_kind {
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Exclam => Self::Deref,
            TokenKind::Not => Self::Not,
            TokenKind::Delete => Self::Delete,
            _ => panic!("Cannot convert token to unop"),
        }
    }
}
impl From<&TokenKind> for BinopKind {
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
            _ => panic!("Cannot convert token to binop"),
        }
    }
}
