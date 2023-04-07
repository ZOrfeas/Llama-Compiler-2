use std::rc::Rc;

use log::info;

use crate::parse::ast::Span;

use super::{Type, TypeKind};

#[derive(Debug)]
struct Unification<'a> {
    lhs: Rc<Type>,
    rhs: Rc<Type>,
    span: &'a Span,
    msg: &'static str,
}
#[derive(Debug)]
pub struct InferenceGroup<'a>(Vec<Unification<'a>>);
// TODO: Add field that holds some info about where every unification came from.
impl<'a> InferenceGroup<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn insert_unification(
        &mut self,
        lhs: Rc<Type>,
        rhs: Rc<Type>,
        msg: &'static str,
        span: &'a Span,
    ) {
        info!("Inserting unification pair: {} = {} ({})", lhs, rhs, msg);
        self.0.push(Unification {
            lhs,
            rhs,
            msg,
            span,
        });
    }
}

#[derive(Debug, Clone)]
pub enum ArrayDims {
    Known(u32),
    LowerBounded(u32),
}
impl std::fmt::Display for ArrayDims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayDims::Known(n) => write!(f, "{}", n),
            ArrayDims::LowerBounded(n) => write!(f, ">={}", n),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Constraint {
    Allow(Vec<TypeKind>),
    Disallow(Vec<TypeKind>),
}
impl Constraint {
    pub fn allow_numeric() -> Self {
        Self::Allow(vec![TypeKind::Int, TypeKind::Float])
    }
    pub fn allow_comparables() -> Self {
        Self::Allow(vec![TypeKind::Int, TypeKind::Float, TypeKind::Char])
    }
    // fn allow_
    pub fn disallow_array_and_func() -> Self {
        Self::Disallow(vec![TypeKind::Array, TypeKind::Func])
    }
}
impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constraint::Allow(types) | Constraint::Disallow(types) => {
                let types = types
                    .iter()
                    .map(|t| format!("{:?}", t).to_lowercase())
                    .collect::<Vec<_>>()
                    .join(", ");
                match self {
                    Constraint::Allow(_) => write!(f, "allow({})", types),
                    Constraint::Disallow(_) => write!(f, "disallow({})", types),
                }
            }
        }
    }
}
