use std::rc::Rc;

use crate::parse::ast::{
    def::{Def, DefKind, Par},
    expr::Expr,
};

use super::{
    expr::SemExpr,
    sem_table::SemTable,
    types::{InferenceGroup, Type},
    SemResult,
};

pub trait SemDef<'a> {
    fn sem_def(&mut self, def: &'a Def) -> SemResult<()>;
}
impl<'a> SemDef<'a> for SemTable<'a> {
    fn sem_def(&mut self, def: &'a Def) -> SemResult<()> {
        let mut inf_group = InferenceGroup::new();
        let annotation_type: Option<Rc<Type>> = def.type_.as_ref().map(|t| t.into());
        let def_type: Option<Rc<Type>> = match &def.kind {
            DefKind::Array { dims } => {
                self.sem_array_def(&mut inf_group, dims)?;
                None
            }
            DefKind::Const { expr } => Some(self.sem_expr(&mut inf_group, expr)?),
            DefKind::Function { pars, expr } => {
                Some(self.sem_func_def(&mut inf_group, pars, expr)?)
            }
            DefKind::Variable => None,
        };
        let node_type: Rc<Type> = match (annotation_type, def_type) {
            (Some(annotation_type), Some(def_type)) => {
                inf_group.insert_unificiation(Rc::clone(&annotation_type), def_type);
                annotation_type
            }
            (Some(t), None) | (None, Some(t)) => t,
            _ => self.types.new_unknown(),
        };
        // *Note: lookup first. If it's already there, then instead of inserting, insert a unification.
        if let Some(ty) = self.types.get_type(def) {
            inf_group.insert_unificiation(Rc::clone(&ty), node_type)
        } else {
            self.types.insert(def, node_type);
        }
        // TODO: Unify all types in the inference group
        Ok(())
    }
}
impl<'a> SemDefHelpers<'a> for SemTable<'a> {
    fn sem_array_def(
        &mut self,
        inf_group: &mut InferenceGroup,
        dims: &'a Vec<Expr>,
    ) -> SemResult<()> {
        for dim in dims {
            let expr_type = self.sem_expr(inf_group, dim)?;
            inf_group.insert_unificiation(self.types.get_int(), expr_type);
        }
        Ok(())
    }

    fn sem_func_def(
        &mut self,
        inf_group: &mut InferenceGroup,
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
            self.types.insert(par, Rc::clone(&par_type));
            par_types.push(par_type);
        }
        let expr_type = self.sem_expr(inf_group, expr)?;
        self.pop_scope();
        let func_type = par_types
            .into_iter()
            .rfold(expr_type, |acc, par_type| Type::new_func(par_type, acc));
        Ok(func_type)
    }
}
trait SemDefHelpers<'a> {
    fn sem_array_def(
        &mut self,
        inf_group: &mut InferenceGroup,
        dims: &'a Vec<Expr>,
    ) -> SemResult<()>;
    fn sem_func_def(
        &mut self,
        inf_group: &mut InferenceGroup,
        pars: &'a Vec<Par>,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
}
