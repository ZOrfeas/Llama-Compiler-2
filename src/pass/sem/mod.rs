mod def;
mod expr;
mod sem_table;
mod types;

use std::rc::Rc;

use crate::parse::ast::{
    def::{DefKind, Definition, Letdef, Typedef},
    Program, Span,
};
use thiserror::Error;

use self::{
    def::SemDef,
    sem_table::SemTable,
    types::{inference::InfererHelpers, Type},
};

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
        for tdef in &typedef.tdefs {
            self.insert_scope_binding(&tdef.id, tdef);
            // if let Some(previous_node) = self.insert_scope_binding(&tdef.id, tdef) {
            //     return Err(SemanticError::GeneralError {
            //         msg: format!(
            //             "Type {} already defined at {}",
            //             tdef.id,
            //             previous_node.get_span().start
            //         ),
            //         span: tdef.span.clone(),
            //     });
            // }
            self.types.insert(tdef, Type::new_custom(tdef.id.clone()));
        }
        for tdef in &typedef.tdefs {
            for constr in &tdef.constrs {
                // *Done(?): Fix this to work with constructors that take no arguments
                self.insert_scope_binding(&constr.id, constr);
                let tdef_type = self
                    .types
                    .get_type(tdef)
                    .expect("type should have just been inserted");
                // if let Some(previous_node) = self.insert_scope_binding(&constr.id, constr) {
                //     return Err(SemanticError::GeneralError {
                //         msg: format!(
                //             "Constructor {} already defined at {}",
                //             constr.id,
                //             previous_node.get_span().start
                //         ),
                //         span: constr.span.clone(),
                //     });
                // }
                if constr.types.len() > 0 {
                    self.types.insert(
                        constr,
                        Type::new_multi_arg_func(
                            constr.types.iter().map(|t| t.into()).collect(),
                            tdef_type,
                        ),
                    )
                } else {
                    self.types.insert(constr, tdef_type)
                }
            }
        }
        Ok(())
    }
    fn sem_letdef(&mut self, letdef: &'a Letdef) -> SemResult<()> {
        if letdef.rec {
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
        // if it's recursive and any of the definitions is not fully known
        // (can be relaxed later on perhaps)
        if letdef.rec {
            if let Some(def) = letdef.defs.iter().find(|def| {
                self.types
                    .get_type(*def)
                    .map(|t| !self.types.deep_resolve_type(t).is_fully_known())
                    .unwrap_or(false)
            }) {
                return Err(SemanticError::GeneralError {
                    msg: format!(
                        "Recursive generic def is not (yet) supported (of type {})",
                        self.types
                            .deep_resolve_type(self.types.get_type(def).unwrap())
                    ),
                    // msg: "Recursive generic definitions are not (yet) allowed".to_string(),
                    span: def.span.clone(),
                });
            }
        }
        for def in &letdef.defs {
            let def_type = self
                .types
                .get_type(def)
                .expect("should have a type after sem");
            let def_type = self.types.deep_resolve_type(def_type);
            if !def_type.is_fully_known()
                && !matches!(def.kind, DefKind::Variable | DefKind::Array { .. })
            {
                self.types.mark_generic(def);
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
            format!("originally {} = {}", lhs, rhs)
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
    },
    #[error("{} (at {})", msg, span)]
    GeneralError { msg: String, span: Span },
    // #[error("Type mismatch: expected {}, got {} ({})", expected, given, msg)]
    // TypeMismatch {
    //     expected: Type,
    //     given: Type,
    //     msg: &'static str,
    // },
    // #[error("Invalid type given: {} ({})", given, msg)]
    // InvalidType { given: Type, msg: &'static str },
}
