mod def;
mod expr;
mod inference;
pub mod sem_table;

use crate::parse::ast::{annotation::Type, def::Definition, Program};
use thiserror::Error;

use self::{def::sem_def, sem_table::SemTable};

pub fn sem<'a>(ast: &'a Program) -> SemResult<SemTable<'a>> {
    let mut sem_table = SemTable::new(ast);

    for def in &ast.definitions {
        match def {
            Definition::Let(letdef) => {
                if letdef.rec {
                    letdef.defs.iter().for_each(|def| {
                        sem_table.insert_binding(&def.id, def);
                    });
                }
                for def in &letdef.defs {
                    sem_def(def, &mut sem_table)?;
                }
                if !letdef.rec {
                    letdef.defs.iter().for_each(|def| {
                        sem_table.insert_binding(&def.id, def);
                    });
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
    #[error("Identifier {} not found", id)]
    LookupError { id: String },
    #[error("Type mismatch: expected {}, got {} ({})", expected, given, msg)]
    TypeMismatch {
        expected: Type,
        given: Type,
        msg: &'static str,
    },
    #[error("Invalid type given: {} ({})", given, msg)]
    InvalidType { given: Type, msg: &'static str },
}
