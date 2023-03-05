use crate::parse::ast::def::{Def, DefKind};

use super::{sem_table::SemTable, SemResult};

pub fn sem_def<'a>(def: &'a Def, sem_table: &mut SemTable<'a>) -> SemResult<()> {
    todo!()
    // match &def.kind {
    //     DefKind::Array { dims } => sem_array_def(),
    //     DefKind::Const { expr } => sem_const_def(),
    //     DefKind::Function { pars, expr } => sem_func_def(),
    //     DefKind::Variable => sem_var_def(),
    // }
}
