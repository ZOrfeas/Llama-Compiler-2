
#include "./sem.hpp"
#include "../../ast/ast.hpp"

// Constructor

SemVisitor::SemVisitor() {}

// Utilities

// Visit method overloads

void SemVisitor::visit(ast::core::Program* program) {}
void SemVisitor::visit(ast::stmt::TypeStmt* type_stmt) {}
void SemVisitor::visit(ast::def::TypeDef* type_def) {}
void SemVisitor::visit(ast::stmt::LetStmt* let_stmt) {}
void SemVisitor::visit(ast::def::Constant* cnst) {}
void SemVisitor::visit(ast::def::Function* fn) {}
void SemVisitor::visit(ast::def::Array* arr) {}
void SemVisitor::visit(ast::def::Variable* var) {}

void SemVisitor::visit(ast::expr::LetIn* let_in) {}
void SemVisitor::visit(ast::expr::literal::Unit* unit) {}
void SemVisitor::visit(ast::expr::literal::Bool* boolean) {}
void SemVisitor::visit(ast::expr::literal::Int* integer) {}
void SemVisitor::visit(ast::expr::literal::Char* chr) {}
void SemVisitor::visit(ast::expr::literal::Float* flt) {}
void SemVisitor::visit(ast::expr::literal::String* str) {}
void SemVisitor::visit(ast::expr::op::Binary* binop) {}
void SemVisitor::visit(ast::expr::op::Unary* unop) {}
void SemVisitor::visit(ast::expr::op::New* newop) {}
void SemVisitor::visit(ast::expr::While* while_expr) {}
void SemVisitor::visit(ast::expr::For* for_expr) {}
void SemVisitor::visit(ast::expr::If* if_expr) {}
void SemVisitor::visit(ast::expr::Dim* dim_expr) {}
void SemVisitor::visit(ast::expr::IdCall* id_call) {}
void SemVisitor::visit(ast::expr::FuncCall* fn_call) {}
void SemVisitor::visit(ast::expr::ConstrCall* cnstr_call) {}
void SemVisitor::visit(ast::expr::ArrayAccess* arr_access) {}
void SemVisitor::visit(ast::expr::Match* match_expr) {}

void SemVisitor::visit(ast::annotation::BasicType* basic_type) {}
void SemVisitor::visit(ast::annotation::FunctionType* fn_type) {}
void SemVisitor::visit(ast::annotation::ArrayType* arr_type) {}
void SemVisitor::visit(ast::annotation::RefType* ref_type) {}
void SemVisitor::visit(ast::annotation::CustomType* custom_type) {}

void SemVisitor::visit(ast::utils::def::Constructor* constructor) {}
void SemVisitor::visit(ast::utils::def::Param* param) {}

void SemVisitor::visit(ast::utils::match::PatLiteral* pat_literal) {}
void SemVisitor::visit(ast::utils::match::PatId* pat_id) {}
void SemVisitor::visit(ast::utils::match::PatConstr* pat_constr) {}
void SemVisitor::visit(ast::utils::match::Clause* clause) {}
