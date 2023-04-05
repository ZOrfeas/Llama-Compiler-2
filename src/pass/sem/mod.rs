mod def;
mod expr;
mod inference;
mod sem_table;
mod types;

use crate::parse::ast::{def::Definition, Program, Span};
use thiserror::Error;

use self::{def::SemDef, sem_table::SemTable};

pub fn sem<'a>(ast: &'a Program) -> SemResult<SemTable<'a>> {
    let mut sem_table = SemTable::new(ast);
    for def in &ast.definitions {
        match def {
            Definition::Let(letdef) => {
                if letdef.rec {
                    for def in &letdef.defs {
                        // *DONE: Insert an unknown type for each def as well I think
                        sem_table.insert_scope_binding(&def.id, def);
                        let def_type = sem_table.types.new_unknown(); // needed cause poor ol' borrowchecker's whinin'
                        sem_table.types.insert(def, def_type);
                    }
                }
                for def in &letdef.defs {
                    sem_table.sem_def(def)?;
                }
                if !letdef.rec {
                    for def in &letdef.defs {
                        sem_table.insert_scope_binding(&def.id, def);
                    }
                }
            }
            Definition::Type(typedef) => todo!(),
        }
    }
    Ok(sem_table)
}
type SemResult<T> = Result<T, SemanticError>;

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("Identifier {} not found (at {})", id, span)]
    LookupError { id: String, span: Span },
    // #[error("Type mismatch: expected {}, got {} ({})", expected, given, msg)]
    // TypeMismatch {
    //     expected: Type,
    //     given: Type,
    //     msg: &'static str,
    // },
    // #[error("Invalid type given: {} ({})", given, msg)]
    // InvalidType { given: Type, msg: &'static str },
}
