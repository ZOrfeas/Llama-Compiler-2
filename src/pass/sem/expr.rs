use std::rc::Rc;

use super::types::{InferenceGroup, Type};
use super::{sem_table::SemTable, SemResult};
use crate::parse::ast::expr::{
    ArrayAccess, Binop, Call, Dim, Expr, ExprKind, For, If, LetIn, Match, Unop, While,
};
use crate::pass::sem::types::{ArrayDims, Constraint};

pub trait SemExpr<'a> {
    fn sem_expr(&mut self, inf_group: &mut InferenceGroup, expr: &'a Expr) -> SemResult<Rc<Type>>;
}
impl<'a> SemExpr<'a> for SemTable<'a> {
    fn sem_expr(&mut self, inf_group: &mut InferenceGroup, expr: &'a Expr) -> SemResult<Rc<Type>> {
        use ExprKind::*;
        let expr_type: Rc<Type> = match &expr.kind {
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
            Unop(unop) => self.sem_unop(inf_group, unop)?,
            Binop(binop) => self.sem_binop(inf_group, binop)?,
            Call(call) => {
                if call.args.len() == 0 {
                    self.sem_constant_call(inf_group, call)?
                } else {
                    self.sem_func_call(inf_group, call)?
                }
            }
            ConstrCall(call) => self.sem_constructor_call(inf_group, call)?,
            ArrayAccess(array_access) => self.sem_array_access(inf_group, array_access)?,
            Dim(dim) => self.sem_dim(inf_group, dim)?,
            New(annotation) => Type::new_ref(annotation.into()),
            LetIn(let_in) => self.sem_letin(inf_group, let_in)?,
            If(if_expr) => self.sem_if(inf_group, if_expr)?,
            While(while_expr) => self.sem_while(inf_group, while_expr)?,
            For(for_expr) => self.sem_for(inf_group, for_expr)?,
            Match(match_expr) => self.sem_match(inf_group, match_expr)?,
        };
        self.types.insert(expr, Rc::clone(&expr_type));
        Ok(expr_type)
    }
}

impl<'a> SemExprHelpers<'a> for SemTable<'a> {
    fn sem_unop(&mut self, inf_group: &mut InferenceGroup, unop: &'a Unop) -> SemResult<Rc<Type>> {
        let op_type = self.sem_expr(inf_group, &unop.operand)?;
        use crate::parse::ast::expr::UnopKind::*;
        match unop.op {
            Plus | Minus => {
                let unknown_numeric = self
                    .types
                    .new_unknown_with_constraint(Constraint::allow_numeric());
                inf_group.insert_unificiation(Rc::clone(&op_type), unknown_numeric);
                Ok(op_type)
            }
            Not => {
                inf_group.insert_unificiation(Rc::clone(&op_type), self.types.get_bool());
                Ok(self.types.get_bool())
            }
            Deref => {
                let inner = self.types.new_unknown();
                let unknown_ref = Type::new_ref(Rc::clone(&inner));
                inf_group.insert_unificiation(Rc::clone(&op_type), Rc::clone(&unknown_ref));
                Ok(inner)
            }
            Delete => {
                inf_group.insert_unificiation(Rc::clone(&op_type), self.types.new_unknown_ref());
                Ok(self.types.get_unit())
            }
        }
    }
    fn sem_binop(
        &mut self,
        inf_group: &mut InferenceGroup,
        binop: &'a Binop,
    ) -> SemResult<Rc<Type>> {
        todo!()
    }
    fn sem_constant_call(
        &mut self,
        inf_group: &mut InferenceGroup,
        call: &'a Call,
    ) -> SemResult<Rc<Type>> {
        todo!()
    }
    fn sem_func_call(
        &mut self,
        inf_group: &mut InferenceGroup,
        call: &'a Call,
    ) -> SemResult<Rc<Type>> {
        todo!()
    }
    fn sem_constructor_call(
        &mut self,
        inf_group: &mut InferenceGroup,
        call: &'a Call,
    ) -> SemResult<Rc<Type>> {
        todo!()
    }
    fn sem_array_access(
        &mut self,
        inf_group: &mut InferenceGroup,
        array_access: &'a ArrayAccess,
    ) -> SemResult<Rc<Type>> {
        todo!()
    }
    fn sem_dim(&mut self, inf_group: &mut InferenceGroup, dim: &'a Dim) -> SemResult<Rc<Type>> {
        todo!()
    }
    fn sem_letin(&mut self, inf_group: &mut InferenceGroup, new: &'a LetIn) -> SemResult<Rc<Type>> {
        todo!()
    }
    fn sem_if(&mut self, inf_group: &mut InferenceGroup, if_expr: &'a If) -> SemResult<Rc<Type>> {
        todo!()
    }
    fn sem_while(
        &mut self,
        inf_group: &mut InferenceGroup,
        while_expr: &'a While,
    ) -> SemResult<Rc<Type>> {
        todo!()
    }
    fn sem_for(
        &mut self,
        inf_group: &mut InferenceGroup,
        for_expr: &'a For,
    ) -> SemResult<Rc<Type>> {
        todo!()
    }
    fn sem_match(
        &mut self,
        inf_group: &mut InferenceGroup,
        if_expr: &'a Match,
    ) -> SemResult<Rc<Type>> {
        todo!()
    }
}

trait SemExprHelpers<'a> {
    fn sem_unop(&mut self, inf_group: &mut InferenceGroup, unop: &'a Unop) -> SemResult<Rc<Type>>;
    fn sem_binop(
        &mut self,
        inf_group: &mut InferenceGroup,
        binop: &'a Binop,
    ) -> SemResult<Rc<Type>>;
    fn sem_constant_call(
        &mut self,
        inf_group: &mut InferenceGroup,
        call: &'a Call,
    ) -> SemResult<Rc<Type>>;
    fn sem_func_call(
        &mut self,
        inf_group: &mut InferenceGroup,
        call: &'a Call,
    ) -> SemResult<Rc<Type>>;
    fn sem_constructor_call(
        &mut self,
        inf_group: &mut InferenceGroup,
        call: &'a Call,
    ) -> SemResult<Rc<Type>>;
    fn sem_array_access(
        &mut self,
        inf_group: &mut InferenceGroup,
        array_access: &'a ArrayAccess,
    ) -> SemResult<Rc<Type>>;
    fn sem_dim(&mut self, inf_group: &mut InferenceGroup, dim: &'a Dim) -> SemResult<Rc<Type>>;
    fn sem_letin(&mut self, inf_group: &mut InferenceGroup, new: &'a LetIn) -> SemResult<Rc<Type>>;
    fn sem_if(&mut self, inf_group: &mut InferenceGroup, if_expr: &'a If) -> SemResult<Rc<Type>>;
    fn sem_while(
        &mut self,
        inf_group: &mut InferenceGroup,
        while_expr: &'a While,
    ) -> SemResult<Rc<Type>>;
    fn sem_for(&mut self, inf_group: &mut InferenceGroup, for_expr: &'a For)
        -> SemResult<Rc<Type>>;
    fn sem_match(
        &mut self,
        inf_group: &mut InferenceGroup,
        if_expr: &'a Match,
    ) -> SemResult<Rc<Type>>;
}

// fn sem_unop<'a>(unop: &'a Unop, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
//     let operand = sem_expr(&unop.operand, sem_table)?;
//     match unop.op {
//         UnopKind::Delete => todo!(),
//         UnopKind::Deref => todo!(),
//         UnopKind::Minus | UnopKind::Plus => todo!(),
//         UnopKind::Not => todo!(),
//     }
// }
// fn sem_binop<'a>(binop: &'a Binop, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
//     let lhs = sem_expr(&binop.lhs, sem_table)?;
//     let rhs = sem_expr(&binop.rhs, sem_table)?;
//     match binop.op {
//         BinopKind::Add | BinopKind::Sub | BinopKind::Mul | BinopKind::Div | BinopKind::Pow => {
//             todo!()
//         }
//         BinopKind::Mod => todo!(),
//         BinopKind::NatEq | BinopKind::NatNotEq | BinopKind::StrEq | BinopKind::StrNotEq => todo!(),
//         BinopKind::Lt | BinopKind::Gt | BinopKind::LEq | BinopKind::GEq => todo!(),
//         BinopKind::And | BinopKind::Or => todo!(),
//         BinopKind::Semicolon => todo!(),
//         BinopKind::Assign => todo!(),
//     }
// }
