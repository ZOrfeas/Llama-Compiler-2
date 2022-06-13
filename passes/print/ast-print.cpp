
#include <string>

#include "../../types.hpp"
#include "./ast-print.hpp"
#include "../../ast/ast.hpp"

const std::string PrintVisitor::full_vert = "│";
const std::string PrintVisitor::split_vert = "├─";
const std::string PrintVisitor::half_vert =  "└─";

// Constructor

PrintVisitor::PrintVisitor(std::ostream& out): out(out) {}

// Utilities

void PrintVisitor::visit(ast::core::Program *program) {}
void PrintVisitor::visit(ast::stmt::TypeStmt *type_stmt) {}
void PrintVisitor::visit(ast::def::TypeDef *type_def) {
}
void PrintVisitor::visit(ast::stmt::LetStmt *let_stmt) {
}
void PrintVisitor::visit(ast::def::Constant *cnst) {
}
void PrintVisitor::visit(ast::def::Function *fn) {
}
void PrintVisitor::visit(ast::def::Array *arr) {
}
void PrintVisitor::visit(ast::def::Variable *var) {
}

void PrintVisitor::visit(ast::expr::LetIn *let_in) {
}
void PrintVisitor::visit(ast::expr::literal::Unit *unit) {
}
void PrintVisitor::visit(ast::expr::literal::Bool *boolean) {
}
void PrintVisitor::visit(ast::expr::literal::Int *integer) {
}
void PrintVisitor::visit(ast::expr::literal::Char *chr) {
}
void PrintVisitor::visit(ast::expr::literal::Float *flt) {
}
void PrintVisitor::visit(ast::expr::literal::String *str) {
}
void PrintVisitor::visit(ast::expr::op::Binary *binop) {
}
void PrintVisitor::visit(ast::expr::op::Unary *unop) {
}
void PrintVisitor::visit(ast::expr::op::New *newop) {
}
void PrintVisitor::visit(ast::expr::While *while_expr) {
}
void PrintVisitor::visit(ast::expr::For *for_expr) {
}
void PrintVisitor::visit(ast::expr::If *if_expr) {
}
void PrintVisitor::visit(ast::expr::Dim *dim_expr) {
}
void PrintVisitor::visit(ast::expr::IdCall *id_call) {
}
void PrintVisitor::visit(ast::expr::FuncCall *fn_call) {
}
void PrintVisitor::visit(ast::expr::ConstrCall *cnstr_call) {
}
void PrintVisitor::visit(ast::expr::ArrayAccess *arr_access) {
}
void PrintVisitor::visit(ast::expr::Match *match_expr) {
}

void PrintVisitor::visit(ast::annotation::BasicType *basic_type) {
}
void PrintVisitor::visit(ast::annotation::FunctionType *fn_type) {
}
void PrintVisitor::visit(ast::annotation::ArrayType *arr_type) {
}
void PrintVisitor::visit(ast::annotation::RefType *ref_type) {}
void PrintVisitor::visit(ast::annotation::CustomType *custom_type) {}

void PrintVisitor::visit(ast::utils::def::Constructor *constructor) {}
void PrintVisitor::visit(ast::utils::def::Param *param) {}

void PrintVisitor::visit(ast::utils::match::PatLiteral *pat_literal) {}
void PrintVisitor::visit(ast::utils::match::PatId *pat_id) {}
void PrintVisitor::visit(ast::utils::match::PatConstr *pat_constr) {}
void PrintVisitor::visit(ast::utils::match::Clause *clause) {}