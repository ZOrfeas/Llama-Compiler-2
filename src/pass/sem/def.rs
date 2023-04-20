use std::rc::Rc;

use log::warn;

use crate::parse::ast::{
    def::{Def, DefKind, Par},
    expr::Expr,
};

use super::{
    expr::SemExpr,
    sem_table::SemTable,
    types::inference::{InferenceGroup, Inferer},
    types::Type,
    SemResult,
};

pub trait SemDef<'a> {
    fn sem_def(&mut self, def: &'a Def) -> SemResult<()>;
}
impl<'a> SemDef<'a> for SemTable<'a> {
    fn sem_def(&mut self, def: &'a Def) -> SemResult<()> {
        let mut inf_group = self.new_inference_group();
        let annotation_type: Option<Rc<Type>> = def.type_.as_ref().map(|t| t.into());
        let node_type: Rc<Type> = match &def.kind {
            DefKind::Array { dims } => {
                self.sem_array_def(&mut inf_group, dims)?;
                match annotation_type {
                    Some(t) => t,
                    None => self.types.new_unknown(),
                }
            }
            DefKind::Const { expr } => {
                let expr_type = self.sem_expr(&mut inf_group, expr)?;
                match annotation_type {
                    Some(t) => {
                        inf_group.insert_unification(
                            t.clone(),
                            expr_type,
                            "expression type and annotation must match",
                            &def.span,
                        );
                        t
                    }
                    None => expr_type,
                }
            }
            DefKind::Function { pars, expr } => {
                let func_type = self.sem_func_def(&mut inf_group, pars, expr)?;
                match annotation_type {
                    Some(t) => inf_group.insert_unification(
                        t,
                        self.types
                            .get_type(expr)
                            .expect("function expression should have just had a time"),
                        "function expression type and annotation must match",
                        &def.span,
                    ),
                    None => (),
                };
                func_type
                // Some(self.sem_func_def(&mut inf_group, pars, expr)?)
            }
            DefKind::Variable => match annotation_type {
                Some(t) => t,
                None => self.types.new_unknown(),
            },
        };
        // *Note: lookup first. If it's already there, then instead of inserting, insert a unification.
        if let Some(ty) = self.types.get_type(def) {
            // TODO: Test that the 'ty' type is unknown (I think that's the only case where this is valid)
            inf_group.insert_unification(
                ty.clone(),
                node_type,
                "recursive definition's type",
                &def.span,
            );
        } else {
            self.types.insert(def, node_type);
        }
        // TODO: Think if you mark generics here, or at the end of the sem_letdef.
        self.types.solve_group(inf_group)?;
        Ok(())
    }
}
impl<'a> SemDefHelpers<'a> for SemTable<'a> {
    fn sem_array_def(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        dims: &'a Vec<Expr>,
    ) -> SemResult<()> {
        for dim in dims {
            let expr_type = self.sem_expr(inf_group, dim)?;
            inf_group.insert_unification(
                self.types.get_int(),
                expr_type,
                "array definition dimensions must be integers",
                &dim.span,
            );
        }
        Ok(())
    }

    fn sem_func_def(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        pars: &'a Vec<Par>,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        self.push_scope();
        let mut par_types = Vec::with_capacity(pars.len());
        for par in pars {
            // Insert all params in the current(new) scope and create a new unknown type for each.
            let par_type = par
                .type_
                .as_ref()
                .map(|t| t.into())
                .unwrap_or_else(|| self.types.new_unknown());
            self.insert_scope_binding(&par.id, par);
            self.types.insert(par, par_type.clone());
            par_types.push(par_type);
        }
        let expr_type = self.sem_expr(inf_group, expr)?;
        self.pop_scope();
        let func_type = Type::new_multi_arg_func(par_types, expr_type);
        Ok(func_type)
    }
}
trait SemDefHelpers<'a> {
    fn sem_array_def(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        dims: &'a Vec<Expr>,
    ) -> SemResult<()>;
    fn sem_func_def(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        pars: &'a Vec<Par>,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
}
