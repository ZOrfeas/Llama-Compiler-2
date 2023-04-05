use std::rc::Rc;

use log::info;

use crate::parse::ast::Span;

use super::Type;

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
