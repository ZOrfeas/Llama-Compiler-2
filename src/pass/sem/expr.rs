use super::types::Type;
use super::{sem_table::SemTable, SemResult};
use crate::parse::ast::expr::{
    ArrayAccess, Binop, Call, Dim, Expr, For, If, LetIn, Match, Unop, UnopKind, While,
};
use crate::parse::ast::expr::{BinopKind, ExprKind};

pub fn sem_expr<'a, 'b>(expr: &'a Expr, sem_table: &'b mut SemTable<'a>) -> SemResult<&'b Type> {
    let expr_type: Type = match &expr.kind {
        ExprKind::UnitLiteral => Type::Unit,
        ExprKind::IntLiteral(_) => Type::Int,
        ExprKind::FloatLiteral(_) => Type::Float,
        ExprKind::CharLiteral(_) => Type::Char,
        ExprKind::StringLiteral(_) => Type::Array {
            dim_cnt: 1,
            inner: Box::new(Type::Char),
        },
        ExprKind::BoolLiteral(_) => Type::Bool,
        ExprKind::Tuple(expr_vec) => Type::Tuple(
            expr_vec
                .iter()
                .map(|expr| sem_expr(expr, sem_table).cloned())
                .collect::<Result<Vec<_>, _>>()?,
        ),
        ExprKind::Unop(unop) => sem_unop(unop, sem_table)?,
        ExprKind::Binop(binop) => sem_binop(binop, sem_table)?,
        ExprKind::Call(call) => {
            if call.args.len() == 0 {
                sem_constant_call(call, sem_table)?
            } else {
                sem_func_call(call, sem_table)?
            }
        }
        ExprKind::ConstrCall(call) => sem_constructor_call(call, sem_table)?,
        ExprKind::ArrayAccess(array_access) => sem_array_access(array_access, sem_table)?,
        ExprKind::Dim(dim) => sem_dim(dim, sem_table)?,
        ExprKind::New(type_) => Type::Ref(Box::new(type_.clone())),
        ExprKind::LetIn(let_in) => sem_letin(let_in, sem_table)?,
        ExprKind::If(if_expr) => sem_if(if_expr, sem_table)?,
        ExprKind::While(while_expr) => sem_while(while_expr, sem_table)?,
        ExprKind::For(for_expr) => sem_for(for_expr, sem_table)?,
        ExprKind::Match(match_expr) => sem_match(match_expr, sem_table)?,
    };
    Ok(sem_table.insert_type(expr, expr_type))
}

fn sem_unop<'a>(unop: &'a Unop, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
    let operand = sem_expr(&unop.operand, sem_table)?;
    match unop.op {
        UnopKind::Delete => todo!(),
        UnopKind::Deref => todo!(),
        UnopKind::Minus | UnopKind::Plus => todo!(),
        UnopKind::Not => todo!(),
    }
}
fn sem_binop<'a>(binop: &'a Binop, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
    let lhs = sem_expr(&binop.lhs, sem_table)?;
    let rhs = sem_expr(&binop.rhs, sem_table)?;
    match binop.op {
        BinopKind::Add | BinopKind::Sub | BinopKind::Mul | BinopKind::Div | BinopKind::Pow => {
            todo!()
        }
        BinopKind::Mod => todo!(),
        BinopKind::NatEq | BinopKind::NatNotEq | BinopKind::StrEq | BinopKind::StrNotEq => todo!(),
        BinopKind::Lt | BinopKind::Gt | BinopKind::LEq | BinopKind::GEq => todo!(),
        BinopKind::And | BinopKind::Or => todo!(),
        BinopKind::Semicolon => todo!(),
        BinopKind::Assign => todo!(),
    }
}
fn sem_func_call<'a>(call: &'a Call, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
    todo!()
}
fn sem_constant_call<'a>(call: &'a Call, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
    todo!()
}
fn sem_constructor_call<'a>(call: &'a Call, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
    todo!()
}
fn sem_array_access<'a>(
    array_access: &'a ArrayAccess,
    sem_table: &mut SemTable<'a>,
) -> SemResult<Type> {
    todo!()
}
fn sem_dim<'a>(dim: &'a Dim, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
    todo!()
}
fn sem_letin<'a>(let_in: &'a LetIn, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
    todo!()
}
fn sem_if<'a>(if_expr: &'a If, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
    todo!()
}
fn sem_while<'a>(while_expr: &'a While, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
    todo!()
}
fn sem_for<'a>(for_expr: &'a For, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
    todo!()
}
fn sem_match<'a>(match_expr: &'a Match, sem_table: &mut SemTable<'a>) -> SemResult<Type> {
    todo!()
}
