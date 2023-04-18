use std::{cell::RefCell, collections::HashSet, rc::Rc};

use colored::Colorize;
use log::{debug, info};

use crate::{
    parse::ast::Span,
    pass::sem::{SemResult, SemanticError},
};

use super::{type_map::TypeMap, Type, TypeKind};

pub trait Inferer<'a> {
    fn solve_group(&mut self, group: InferenceGroup<'a>) -> SemResult<()>;
    // fn unify(&mut self, unification: Unification<'a>) -> SemResult<()>;
}

impl<'a> Inferer<'a> for TypeMap<'a> {
    fn solve_group(&mut self, group: InferenceGroup<'a>) -> SemResult<()> {
        info!("{}", "↓↓↓  Solving inference group   ↓↓↓".underline());
        for unification in group.0 {
            self.unify(unification)?;
        }
        info!("{}", "↑↑↑Done solving inference group↑↑↑".underline());
        Ok(())
    }
}

impl<'a> InfererHelpers<'a> for TypeMap<'a> {
    fn unify(&mut self, unification: Unification<'a>) -> SemResult<()> {
        // info!(
        //     "Applying unification: {}",
        //     format!("{} = {}", unification.lhs, unification.rhs).green()
        // );
        let lhs = self.deep_resolve_type(unification.lhs.clone());
        let rhs = self.deep_resolve_type(unification.rhs.clone());
        info!(
            "Which becomes:        {} {} {}",
            lhs.to_string().blue(),
            "=".red(),
            rhs.to_string().blue(),
        );
        if lhs == rhs {
            info!("Unifying equal types is a no-op");
            return Ok(());
        }
        use Type::*;
        match (&*lhs, &*rhs) {
            (Unknown(_, _), _) | (_, Unknown(_, _)) => {
                let (unknown, resolved) = if let Unknown(_, _) = &*lhs {
                    (lhs, rhs)
                } else {
                    (rhs, lhs)
                };
                self.try_add_resolution(unknown, resolved)
                    .map_err(|e| self.unification_into_error(unification, &e))
            }
            (
                Func {
                    lhs: lhs1,
                    rhs: rhs1,
                },
                Func {
                    lhs: lhs2,
                    rhs: rhs2,
                },
            ) => {
                self.unify(Unification {
                    lhs: lhs1.clone(),
                    rhs: lhs2.clone(),
                    span: unification.span,
                    msg: unification.msg,
                })?;
                self.unify(Unification {
                    lhs: rhs1.clone(),
                    rhs: rhs2.clone(),
                    span: unification.span,
                    msg: unification.msg,
                })
            }
            (Ref(lhs_inner), Ref(rhs_inner)) => self.unify(Unification {
                lhs: lhs_inner.clone(),
                rhs: rhs_inner.clone(),
                span: unification.span,
                msg: unification.msg,
            }),
            (
                Array {
                    inner: lhs_inner,
                    dim_cnt: lhs_dims,
                },
                Array {
                    inner: rhs_inner,
                    dim_cnt: rhs_dims,
                },
            ) => {
                if !lhs_dims
                    .borrow()
                    .borrow()
                    .are_compatible(&*rhs_dims.borrow().borrow())
                {
                    return Err(self.unification_into_error(
                        unification,
                        &format!(
                            "Can't match dims {} with {}",
                            lhs_dims.borrow().borrow(),
                            rhs_dims.borrow().borrow()
                        ),
                    ));
                }
                self.unify(Unification {
                    lhs: lhs_inner.clone(),
                    rhs: rhs_inner.clone(),
                    span: unification.span,
                    msg: unification.msg,
                })?;

                todo!("choose the dim_cnt that is more specific")
            }
            (Tuple(lhs_types), Tuple(rhs_types)) => {
                if lhs_types.len() != rhs_types.len() {
                    return Err(self.unification_into_error(unification, "Tuple sizes don't match"));
                }
                lhs_types
                    .iter()
                    .zip(rhs_types.iter())
                    .try_for_each(|(lhs, rhs)| {
                        self.unify(Unification {
                            lhs: lhs.clone(),
                            rhs: rhs.clone(),
                            span: unification.span,
                            msg: unification.msg,
                        })
                    })
            }
            _ => return Err(self.unification_into_error(unification, "Couldn't unify")),
        }
    }
    fn deep_resolve_type(&mut self, ty: Rc<Type>) -> Rc<Type> {
        let ty = self.resolve_type(ty);
        use Type::*;
        match &*ty {
            Func { lhs, rhs } => {
                let lhs = self.deep_resolve_type(lhs.clone());
                let rhs = self.deep_resolve_type(rhs.clone());
                Type::new_func(lhs, rhs)
            }
            Ref(inner) => Type::new_ref(self.deep_resolve_type(inner.clone())),
            // TODO: Think if this is a problem, if we need to refcell `inner` for Array and Ref
            Array { inner, dim_cnt } => Rc::new(Type::Array {
                inner: self.deep_resolve_type(inner.clone()),
                dim_cnt: RefCell::new(dim_cnt.borrow().clone()),
            }),
            Tuple(types) => Type::new_tuple(
                types
                    .iter()
                    .map(|ty| self.deep_resolve_type(ty.clone()))
                    .collect(),
            ),
            _ => ty,
        }
    }
    fn resolve_type(&mut self, ty: Rc<Type>) -> Rc<Type> {
        let mut type_ids = Vec::new();
        let mut retval = ty.clone();
        while let Type::Unknown(id, _) = &*retval {
            type_ids.push(*id);
            if let Some(resolved) = self.unifications.get(id) {
                retval = resolved.clone();
            } else {
                break;
            }
        }
        if type_ids.len() > 1 {
            for id in &type_ids[1..type_ids.len() - 1] {
                self.unifications.insert(*id, retval.clone());
            }
        }
        retval
    }
    fn try_add_resolution(&mut self, unknown: Rc<Type>, resolved: Rc<Type>) -> Result<(), String> {
        let (unknown_id, unknown_constraint) = match &*unknown {
            Type::Unknown(id, constraint_refcell) => (*id, constraint_refcell.borrow()),
            _ => unreachable!("try_add_resolution called on non-unknown type"),
        };
        if !Self::fulfills_constraints(&unknown, &resolved) {
            return Err(format!("Constraints violated"));
        }
        // TODO: Re-enable this check once implemented.
        // if self.occurs(unknown_id, &resolved) {
        //     return Err(format!(
        //         "Occurs check failed, recursive unknown type implied"
        //     ));
        // }
        match &*resolved {
            Type::Unknown(_, constraint_refcell) => {
                let mut resolved_constraints = constraint_refcell.borrow_mut();
                resolved_constraints.consolidate(&unknown_constraint)
            }
            _ => (),
        };
        self.unifications.insert(unknown_id, resolved);
        Ok(())
    }
    fn fulfills_constraints(unknown: &Rc<Type>, resolved: &Rc<Type>) -> bool {
        use Type::*;
        match &**unknown {
            Unknown(_, constraints) => constraints.borrow().are_satisfied_by(resolved),
            _ => unreachable!("fulfills_constraints called on non-unknown type"),
        }
    }
    fn occurs(&self, id: u32, ty: &Rc<Type>) -> bool {
        todo!()
    }
    fn unification_into_error(&mut self, unification: Unification<'a>, msg: &str) -> SemanticError {
        let lhs_resolved = self.deep_resolve_type(unification.lhs.clone());
        let rhs_resolved = self.deep_resolve_type(unification.rhs.clone());
        SemanticError::InferenceError {
            msg: msg.to_string(),
            lhs_resolved,
            rhs_resolved,
            lhs: unification.lhs,
            rhs: unification.rhs,
            span: unification.span.clone(),
            unification_reason: unification.msg.to_owned(),
        }
    }
}
trait InfererHelpers<'a> {
    fn unify(&mut self, unification: Unification<'a>) -> SemResult<()>;
    /// resolves the type and then resolves all the types inside of it.
    fn deep_resolve_type(&mut self, ty: Rc<Type>) -> Rc<Type>;
    /// follows the chain of unifications until it finds a type that is not an unknown type.
    fn resolve_type(&mut self, ty: Rc<Type>) -> Rc<Type>;
    /// checks if type `ty` contains an unknown type with id `id`, to prevent recursive unknown types.
    fn occurs(&self, id: u32, ty: &Rc<Type>) -> bool;
    ///
    fn try_add_resolution(&mut self, unknown: Rc<Type>, resolved: Rc<Type>) -> Result<(), String>;
    /// checks if `unknown` can resolve to `resolved`.
    fn fulfills_constraints(unknown: &Rc<Type>, resolved: &Rc<Type>) -> bool;
    /// converts a unification into a semantic error.
    fn unification_into_error(&mut self, unification: Unification<'a>, msg: &str) -> SemanticError;
}
#[derive(Debug)]
struct Unification<'a> {
    lhs: Rc<Type>,
    rhs: Rc<Type>,
    span: &'a Span,
    msg: &'static str,
}
#[derive(Debug)]
pub struct InferenceGroup<'a>(Vec<Unification<'a>>);
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
impl ArrayDims {
    pub fn are_compatible(&self, other: &Self) -> bool {
        use ArrayDims::*;
        match (self, other) {
            (a, b) if a == b => true,
            (Known(a), LowerBounded(b)) | (LowerBounded(b), Known(a)) => *b <= *a,
            (LowerBounded(_), LowerBounded(_)) => true,
            _ => false,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Constraints {
    allowed: HashSet<TypeKind>,
    disallowed: HashSet<TypeKind>,
}
impl Constraints {
    pub fn new() -> Self {
        Self {
            allowed: HashSet::new(),
            disallowed: HashSet::new(),
        }
    }
    pub fn allow_numeric() -> Self {
        Self {
            allowed: vec![TypeKind::Int, TypeKind::Float].into_iter().collect(),
            disallowed: HashSet::new(),
        }
    }
    pub fn allow_comparables() -> Self {
        Self {
            allowed: vec![TypeKind::Int, TypeKind::Float, TypeKind::Char]
                .into_iter()
                .collect(),
            disallowed: HashSet::new(),
        }
    }
    pub fn disallow_array_and_func() -> Self {
        Self {
            allowed: HashSet::new(),
            disallowed: vec![TypeKind::Array, TypeKind::Func].into_iter().collect(),
        }
    }
    pub fn are_satisfied_by(&self, ty: &Rc<Type>) -> bool {
        let type_kind: TypeKind = (&**ty).into();
        if !self.allowed.is_empty() && !self.allowed.contains(&type_kind) {
            return false;
        }
        if self.disallowed.contains(&type_kind) {
            return false;
        }
        true
    }
    pub fn consolidate(&mut self, other: &Self) {
        self.allowed.extend(other.allowed.iter().cloned());
        self.disallowed.extend(other.disallowed.iter().cloned());
    }
}
impl PartialEq for ArrayDims {
    fn eq(&self, other: &Self) -> bool {
        use ArrayDims::*;
        match (self, other) {
            (Known(n), Known(m)) => n == m,
            (LowerBounded(n), LowerBounded(m)) => *n == *m,
            _ => false,
        }
    }
}
impl std::fmt::Display for ArrayDims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayDims::Known(n) => write!(f, "{}", n),
            ArrayDims::LowerBounded(n) => write!(f, ">={}", *n),
        }
    }
}
impl std::fmt::Display for Constraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.allowed.is_empty() && self.disallowed.is_empty() {
            return write!(f, "");
        }

        if !self.allowed.is_empty() {
            write!(f, "{}", format!(" allow{:?}", self.allowed).to_lowercase())?;
            if !self.disallowed.is_empty() {
                write!(f, ", ")?;
            }
        }
        if !self.disallowed.is_empty() {
            write!(
                f,
                "{}",
                format!(" disallow{:?}", self.disallowed).to_lowercase()
            )
        } else {
            Ok(())
        }
    }
}
