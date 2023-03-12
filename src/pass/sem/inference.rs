// use crate::parse::ast::{annotation::Type, annotation::TypeKind, Span};

// #[derive(Debug)]
// pub struct Inferer<'a> {}
// impl<'a> Inferer<'a> {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
// #[derive(Debug)]
// struct TypeEquivalence<'a> {
//     pub lhs: &'a Type,
//     pub rhs: &'a Type,
// }
// #[derive(Debug)]
// enum ConstraintKind<'a> {
//     Allows(&'a Type, Vec<TypeKind>),
//     Disallows(&'a Type, Vec<TypeKind>),
// }
// #[derive(Debug)]
// struct Constraint<'a> {
//     pub msg: &'static str,
//     pub span: &'a Span,
//     pub kind: ConstraintKind<'a>,
// }
