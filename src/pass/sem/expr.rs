use std::rc::Rc;

// use log::debug;

use super::types::inference::InferenceGroup;
use super::types::Type;
use super::{sem_table::SemTable, SemResult};
use super::{SemDefHelpers, SemanticError};
use crate::parse::ast::expr::{
    ArrayAccess, Binop, Call, Dim, Expr, ExprKind, For, If, LetIn, Match, Pattern, PatternKind,
    Unop, While,
};
use crate::pass::sem::types::inference::Constraints;

pub trait SemExpr<'a> {
    fn sem_expr(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
}
impl<'a> SemExpr<'a> for SemTable<'a> {
    fn sem_expr(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        use ExprKind::*;
        let expr_type = match &expr.kind {
            UnitLiteral => self.types.get_unit(),
            IntLiteral(_) => self.types.get_int(),
            FloatLiteral(_) => self.types.get_float(),
            CharLiteral(_) => self.types.get_char(),
            StringLiteral(_) => Type::new_known_array(self.types.get_char(), 1),
            BoolLiteral(_) => self.types.get_bool(),
            Tuple(expr_vec) => Type::new_tuple(
                expr_vec
                    .iter()
                    .map(|expr| self.sem_expr(inf_group, expr))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            Unop(unop) => self.sem_unop(inf_group, unop, expr)?,
            Binop(binop) => self.sem_binop(inf_group, binop, expr)?,
            Call(call) if call.args.len() == 0 => self.sem_constant_call(call, expr)?,
            Call(call) => self.sem_func_call(inf_group, call, expr)?,
            ConstrCall(call) => self.sem_constructor_call(inf_group, call, expr)?,
            ArrayAccess(array_access) => self.sem_array_access(inf_group, array_access, expr)?,
            Dim(dim) => self.sem_dim(inf_group, dim, expr)?,
            New(annotation) => Type::new_ref(annotation.into()),
            LetIn(let_in) => self.sem_letin(inf_group, let_in)?,
            If(if_expr) => self.sem_if(inf_group, if_expr, expr)?,
            While(while_expr) => self.sem_while(inf_group, while_expr, expr)?,
            For(for_expr) => self.sem_for(inf_group, for_expr, expr)?,
            Match(match_expr) => self.sem_match(inf_group, match_expr, expr)?,
        };
        self.types.insert(expr, expr_type.clone());
        Ok(expr_type)
    }
}

impl<'a> SemExprHelpers<'a> for SemTable<'a> {
    fn sem_unop(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        unop: &'a Unop,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        let op_type = self.sem_expr(inf_group, &unop.operand)?;
        use crate::parse::ast::expr::UnopKind::*;
        match unop.op {
            Plus | Minus => {
                let unknown_numeric = self
                    .types
                    .new_unknown_with_constraint(Constraints::allow_numeric());
                inf_group.insert_unification(
                    op_type.clone(),
                    unknown_numeric,
                    "unary '+/-' operand must be numeric",
                    &expr.span,
                );
                Ok(op_type)
            }
            Not => {
                inf_group.insert_unification(
                    op_type.clone(),
                    self.types.get_bool(),
                    "unary 'not' operand must be boolean",
                    &expr.span,
                );
                Ok(self.types.get_bool())
            }
            Deref => {
                let inner = self.types.new_unknown();
                let unknown_ref = Type::new_ref(inner.clone());
                inf_group.insert_unification(
                    op_type.clone(),
                    unknown_ref.clone(),
                    "cannot dereference non-reference",
                    &expr.span,
                );
                Ok(inner)
            }
            Delete => {
                inf_group.insert_unification(
                    op_type.clone(),
                    self.types.new_unknown_ref(),
                    "cannot delete non-reference",
                    &expr.span,
                );
                Ok(self.types.get_unit())
            }
        }
    }
    fn sem_binop(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        binop: &'a Binop,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        let lhs_type = self.sem_expr(inf_group, &binop.lhs)?;
        let rhs_type = self.sem_expr(inf_group, &binop.rhs)?;
        use crate::parse::ast::expr::BinopKind::*;
        match binop.op {
            Add | Sub | Mul | Div | Pow => {
                let unknown_numeric = self
                    .types
                    .new_unknown_with_constraint(Constraints::allow_numeric());
                inf_group.insert_unification(
                    lhs_type.clone(),
                    unknown_numeric.clone(),
                    "binary '+-*/ **' left operand must be numeric",
                    &expr.span,
                );
                inf_group.insert_unification(
                    rhs_type.clone(),
                    unknown_numeric.clone(),
                    "binary '+-*/ **' right operand must be numeric",
                    &expr.span,
                );
                // *Note: Consider whether this one should be inserted or not.
                // inf_group.insert_unification(lhs_type.clone(), rhs_type.clone());
                // *Note: Consider deciding what to return based on what is less unknown.
                Ok(rhs_type)
            }
            Mod => {
                inf_group.insert_unification(
                    lhs_type.clone(),
                    self.types.get_int(),
                    "mod left operand must be an integer",
                    &expr.span,
                );
                inf_group.insert_unification(
                    rhs_type.clone(),
                    self.types.get_int(),
                    "mod right operand must be an integer",
                    &expr.span,
                );
                Ok(self.types.get_int())
            }
            NatEq | NatNotEq | StrEq | StrNotEq => {
                let unknown_non_array_non_func = self
                    .types
                    .new_unknown_with_constraint(Constraints::disallow_array_and_func());
                inf_group.insert_unification(
                    lhs_type.clone(),
                    unknown_non_array_non_func.clone(),
                    "cannot equality compare arrays or functions",
                    &expr.span,
                );
                inf_group.insert_unification(
                    rhs_type.clone(),
                    unknown_non_array_non_func.clone(),
                    "cannot equality compare arrays or functions",
                    &expr.span,
                );
                Ok(self.types.get_bool())
            }
            Lt | Gt | LEq | GEq => {
                let unknown_comparable = self
                    .types
                    .new_unknown_with_constraint(Constraints::allow_comparables());
                inf_group.insert_unification(
                    lhs_type.clone(),
                    unknown_comparable.clone(),
                    "only int/float/char are ordered",
                    &expr.span,
                );
                inf_group.insert_unification(
                    rhs_type.clone(),
                    unknown_comparable.clone(),
                    "only int/float/char are ordered",
                    &expr.span,
                );
                Ok(self.types.get_bool())
            }
            And | Or => {
                inf_group.insert_unification(
                    lhs_type.clone(),
                    self.types.get_bool(),
                    "'&&, ||' operands must be booleans",
                    &expr.span,
                );
                inf_group.insert_unification(
                    rhs_type.clone(),
                    self.types.get_bool(),
                    "'&&, ||' operands must be booleans",
                    &expr.span,
                );
                Ok(self.types.get_bool())
            }
            Semicolon => Ok(rhs_type),
            Assign => {
                let unknown = self.types.new_unknown();
                let unknown_ref = Type::new_ref(unknown.clone());
                inf_group.insert_unification(
                    lhs_type.clone(),
                    unknown_ref,
                    "lhs of ':=' operator must be a reference",
                    &expr.span,
                );
                inf_group.insert_unification(
                    rhs_type.clone(),
                    unknown,
                    "rhs of ':=' operator must be same as pointed at by lhs",
                    &expr.span,
                );
                Ok(self.types.get_unit())
            }
        }
    }
    fn sem_constant_call(&mut self, call: &'a Call, expr: &'a Expr) -> SemResult<Rc<Type>> {
        let called_node = self
            .lookup(&call.id)
            .ok_or_else(|| SemanticError::LookupError {
                id: call.id.clone(),
                span: expr.span.clone(),
            })?;
        let called_type = self.types.get_node_type_or_instantiation(&called_node);
        Ok(called_type)
    }
    fn sem_func_call(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        call: &'a Call,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        let called_node = self
            .lookup(&call.id)
            .ok_or_else(|| SemanticError::LookupError {
                id: call.id.clone(),
                span: expr.span.clone(),
            })?;
        let called_type = self.types.get_node_type_or_instantiation(&called_node);
        let arg_types = call
            .args
            .iter()
            .map(|arg| self.sem_expr(inf_group, arg))
            .collect::<SemResult<Vec<_>>>()?;
        // *NOTE: Think about trying to apply the argument_types to the called_type, for performance perhaps.
        let expr_type = self.types.new_unknown();
        inf_group.insert_unification(
            called_type,
            Type::new_multi_arg_func(arg_types, expr_type.clone()),
            "function call must match function signature",
            &expr.span,
        );
        Ok(expr_type)
    }
    fn sem_constructor_call(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        call: &'a Call,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        let called_node = self
            .lookup(&call.id)
            .ok_or_else(|| SemanticError::LookupError {
                id: call.id.clone(),
                span: expr.span.clone(),
            })?;
        let constructor_type = self
            .types
            .get_node_type(&called_node)
            .expect("constructor node should have a type associated with it");
        let arg_types = call
            .args
            .iter()
            .map(|arg| self.sem_expr(inf_group, arg))
            .collect::<SemResult<Vec<_>>>()?;
        let expr_type = self.types.new_unknown();
        // *Note: partial constructor call is allowed. That's why we don't unify with the custom type directly.
        inf_group.insert_unification(
            constructor_type,
            Type::new_multi_arg_func(arg_types, expr_type.clone()),
            "constructor call must match constructor signature",
            &expr.span,
        );
        Ok(expr_type)
    }
    fn sem_array_access(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        array_access: &'a ArrayAccess,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        let array_node =
            self.lookup(&array_access.id)
                .ok_or_else(|| SemanticError::LookupError {
                    id: array_access.id.clone(),
                    span: expr.span.clone(),
                })?;
        let called_array_type = self
            .types
            .get_node_type(&array_node)
            .expect("array node should have a type associated with it");
        let index_types = array_access
            .indexes
            .iter()
            .map(|index| self.sem_expr(inf_group, index))
            .collect::<SemResult<Vec<_>>>()?;
        let contained_type = self.types.new_unknown();
        inf_group.insert_unification(
            called_array_type,
            Type::new_known_array(contained_type.clone(), index_types.len() as u32),
            "array access must match array signature",
            &expr.span,
        );
        Ok(Type::new_ref(contained_type))
    }
    fn sem_dim(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        dim: &'a Dim,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        let array_node = self
            .lookup(&dim.id)
            .ok_or_else(|| SemanticError::LookupError {
                id: dim.id.clone(),
                span: expr.span.clone(),
            })?;
        let called_array_type = self
            .types
            .get_node_type(&array_node)
            .expect("array node should have a type associated with it");
        inf_group.insert_unification(
            called_array_type,
            Type::new_bounded_array(self.types.new_unknown(), dim.dim as u32),
            "dim call must be on an array that has at least as meany dimensions as the call",
            &expr.span,
        );
        Ok(self.types.get_int())
    }
    fn sem_letin(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        letin: &'a LetIn,
    ) -> SemResult<Rc<Type>> {
        self.push_scope();
        self.sem_letdef(&letin.letdef)?;
        let expr_type = self.sem_expr(inf_group, &letin.expr)?;
        self.pop_scope();
        Ok(expr_type)
    }
    fn sem_if(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        if_expr: &'a If,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        let cond_type = self.sem_expr(inf_group, &if_expr.cond)?;
        inf_group.insert_unification(
            cond_type.clone(),
            self.types.get_bool(),
            "if expression condition must be a boolean",
            &expr.span,
        );
        let then_type = self.sem_expr(inf_group, &if_expr.then_body)?;
        let else_type = if_expr
            .else_body
            .as_ref()
            .map(|else_body| self.sem_expr(inf_group, else_body))
            .transpose()?
            .unwrap_or(self.types.get_unit());
        inf_group.insert_unification(
            then_type,
            else_type.clone(),
            "if expression branches must be of the same type",
            &expr.span,
        );
        Ok(else_type)
    }
    fn sem_while(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        while_expr: &'a While,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        let cond_type = self.sem_expr(inf_group, &while_expr.cond)?;
        inf_group.insert_unification(
            cond_type.clone(),
            self.types.get_bool(),
            "while expression condition must be a boolean",
            &expr.span,
        );
        let body_type = self.sem_expr(inf_group, &while_expr.body)?;
        inf_group.insert_unification(
            body_type,
            self.types.get_unit(),
            "while expression body must be of unit type",
            &expr.span,
        );
        Ok(self.types.get_unit())
    }
    fn sem_for(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        for_expr: &'a For,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        let init_type = self.sem_expr(inf_group, &for_expr.from)?;
        let end_type = self.sem_expr(inf_group, &for_expr.to)?;
        inf_group.insert_unification(
            init_type,
            self.types.get_int(),
            "for expression init and end bounds must be integers",
            &expr.span,
        );
        inf_group.insert_unification(
            end_type,
            self.types.get_int(),
            "for expression init and end bounds must be integers",
            &expr.span,
        );
        self.push_scope();
        self.insert_scope_binding(&for_expr.id, for_expr);
        self.types.insert(for_expr, self.types.get_int());
        let body_type = self.sem_expr(inf_group, &for_expr.body)?;
        inf_group.insert_unification(
            body_type,
            self.types.get_unit(),
            "for expression body must be of unit type",
            &expr.span,
        );
        self.pop_scope();
        Ok(self.types.get_unit())
    }
    fn sem_match(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        match_expr: &'a Match,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>> {
        let to_match_type = self.sem_expr(inf_group, &match_expr.to_match)?;
        let return_type = self.types.new_unknown();
        for clause in &match_expr.clauses {
            self.push_scope();
            self.sem_pattern(inf_group, &clause.pattern, to_match_type.clone())?;
            let clause_type = self.sem_expr(inf_group, &clause.expr)?;
            inf_group.insert_unification(
                clause_type,
                return_type.clone(),
                "match expression clauses must all have the same type",
                &expr.span,
            );
            self.pop_scope();
        }
        Ok(return_type)
    }
    fn sem_pattern(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        pattern: &'a Pattern,
        to_match_type: Rc<Type>,
    ) -> SemResult<()> {
        use PatternKind::*;
        match &pattern.kind {
            IntLiteral(_) | FloatLiteral(_) | BoolLiteral(_) | CharLiteral(_)
            | StringLiteral(_) => {
                let (literal_type, msg) = match &pattern.kind {
                    IntLiteral(_) => (
                        self.types.get_int(),
                        "integer literal pattern must match an integer",
                    ),
                    FloatLiteral(_) => (
                        self.types.get_float(),
                        "float literal pattern must match a float",
                    ),
                    BoolLiteral(_) => (
                        self.types.get_bool(),
                        "boolean literal pattern must match a boolean",
                    ),
                    CharLiteral(_) => (
                        self.types.get_char(),
                        "char literal pattern must match a char",
                    ),
                    StringLiteral(_) => (
                        Type::new_known_array(self.types.get_char(), 1),
                        "string literal pattern must match a string",
                    ),
                    _ => unreachable!(),
                };
                inf_group.insert_unification(to_match_type, literal_type, msg, &pattern.span);
            }
            IdLower(id) => {
                self.insert_scope_binding(id, pattern);
                self.types.insert(pattern, to_match_type);
            }
            IdUpper { id, args } => {
                let constructor_node =
                    self.lookup(id).ok_or_else(|| SemanticError::LookupError {
                        id: id.clone(),
                        span: pattern.span.clone(),
                    })?;
                let constructor_type = self
                    .types
                    .get_node_type(&constructor_node)
                    .expect("constructor node should have a type associated with it");
                if let Type::Func { lhs, rhs } = &*constructor_type {
                    let mut constr_param_types = vec![lhs.clone()];
                    let constr_ret_type = loop {
                        if let Type::Func { lhs, rhs } = &**rhs {
                            constr_param_types.push(lhs.clone());
                            if !matches!(&**rhs, Type::Func { .. }) {
                                break rhs.clone();
                            }
                        } else {
                            break rhs.clone();
                        }
                    };
                    if constr_param_types.len() != args.len() {
                        return Err(SemanticError::GeneralError {
                            msg: format!("partial application of constructor {} in match patterns is not allowed", id),
                            span: pattern.span.clone(),
                        });
                    }
                    for (arg, param_type) in args.iter().zip(constr_param_types) {
                        self.sem_pattern(inf_group, arg, param_type)?;
                    }
                    inf_group.insert_unification(
                        to_match_type,
                        constr_ret_type,
                        "constructor pattern must match the type of the matched expression",
                        &pattern.span,
                    );
                } else if matches!(&*constructor_type, Type::Custom { .. }) {
                    if !args.is_empty() {
                        return Err(SemanticError::GeneralError { 
                            msg: format!("constructor {} invoked with arguments in match pattern, but takes none", id),
                            span: pattern.span.clone(),
                        })
                    }
                    inf_group.insert_unification(
                        to_match_type,
                        constructor_type,
                        "constructor pattern must match the type of the matched expression",
                        &pattern.span,
                    )
                } else {    
                    return Err(SemanticError::GeneralError {
                        msg: format!(
                            "{} type signature is not a function (constructors are functions)",
                            id
                        ),
                        span: pattern.span.clone(),
                    });
                }
            }
            Tuple(elems) => {
                let mut elem_types = Vec::new();
                for elem in elems {
                    let elem_type = self.types.new_unknown();
                    self.sem_pattern(inf_group, elem, elem_type.clone())?;
                    elem_types.push(elem_type);
                }
                let tuple_type = Type::new_tuple(elem_types);
                inf_group.insert_unification(
                    to_match_type,
                    tuple_type,
                    "tuple pattern must match the type of the matched expression",
                    &pattern.span,
                );
            }
        }
        Ok(())
    }
}

trait SemExprHelpers<'a> {
    fn sem_unop(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        unop: &'a Unop,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
    fn sem_binop(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        binop: &'a Binop,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
    fn sem_constant_call(&mut self, call: &'a Call, expr: &'a Expr) -> SemResult<Rc<Type>>;
    fn sem_func_call(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        call: &'a Call,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
    fn sem_constructor_call(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        call: &'a Call,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
    fn sem_array_access(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        array_access: &'a ArrayAccess,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
    fn sem_dim(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        dim: &'a Dim,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
    fn sem_letin(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        letin: &'a LetIn,
    ) -> SemResult<Rc<Type>>;
    fn sem_if(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        if_expr: &'a If,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
    fn sem_while(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        while_expr: &'a While,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
    fn sem_for(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        for_expr: &'a For,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
    fn sem_match(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        match_expr: &'a Match,
        expr: &'a Expr,
    ) -> SemResult<Rc<Type>>;
    fn sem_pattern(
        &mut self,
        inf_group: &mut InferenceGroup<'a>,
        pattern: &'a Pattern,
        to_match_type: Rc<Type>,
    ) -> SemResult<()>;
}
