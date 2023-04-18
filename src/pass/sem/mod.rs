mod def;
mod expr;
mod sem_table;
mod types;

use std::rc::Rc;

use crate::parse::ast::{
    def::{Definition, Letdef, Typedef},
    Program, Span,
};
use thiserror::Error;

use self::{def::SemDef, sem_table::SemTable, types::Type};

pub fn sem<'a>(ast: &'a Program) -> SemResult<SemTable<'a>> {
    let mut sem_table = SemTable::new(ast);
    for def in &ast.definitions {
        match def {
            Definition::Let(letdef) => sem_table.sem_letdef(letdef)?,
            Definition::Type(typedef) => sem_table.sem_typedef(typedef)?,
        }
    }
    Ok(sem_table)
}
trait SemDefHelpers<'a> {
    fn sem_letdef(&mut self, letdef: &'a Letdef) -> SemResult<()>;
    fn sem_typedef(&mut self, typedef: &'a Typedef) -> SemResult<()>;
}
impl<'a> SemDefHelpers<'a> for SemTable<'a> {
    fn sem_typedef(&mut self, typedef: &'a Typedef) -> SemResult<()> {
        todo!()
    }
    fn sem_letdef(&mut self, letdef: &'a Letdef) -> SemResult<()> {
        if letdef.rec {
            todo!("Make sure inference_groups don't ruin shit here");
            for def in &letdef.defs {
                // *DONE: Insert an unknown type for each def as well I think
                self.insert_scope_binding(&def.id, def);
                let def_type = self.types.new_unknown(); // needed cause poor ol' borrowchecker's whinin'
                self.types.insert(def, def_type);
            }
        }
        for def in &letdef.defs {
            self.sem_def(def)?;
        }
        if !letdef.rec {
            for def in &letdef.defs {
                self.insert_scope_binding(&def.id, def);
            }
        }
        Ok(())
    }
}

type SemResult<T> = Result<T, SemanticError>;

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("Identifier {} not found (at {})", id, span)]
    LookupError { id: String, span: Span },
    #[error(
        "{}: {} = {} ({} {} at {})",
        msg,
        lhs_resolved,
        rhs_resolved,
        if lhs != lhs_resolved || rhs != rhs_resolved {
            format!("orignally {} = {}", lhs, rhs)
        } else {
            "".to_string()
        },
        unification_reason,
        span,
    )]
    InferenceError {
        msg: String,
        lhs: Rc<Type>,
        rhs: Rc<Type>,
        lhs_resolved: Rc<Type>,
        rhs_resolved: Rc<Type>,
        span: Span,
        unification_reason: String,
    }, // #[error("Type mismatch: expected {}, got {} ({})", expected, given, msg)]
       // TypeMismatch {
       //     expected: Type,
       //     given: Type,
       //     msg: &'static str,
       // },
       // #[error("Invalid type given: {} ({})", given, msg)]
       // InvalidType { given: Type, msg: &'static str },
}
