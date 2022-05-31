

#include "./printVisitor.hpp"
#include "../../ast/ast.hpp"

const std::string PrintVisitor::full_vert = "│ ";
const std::string PrintVisitor::split_vert = "├─";
const std::string PrintVisitor::half_vert =  "└─";

// Constructor

PrintVisitor::PrintVisitor(std::ostream& out): out(out) {}

// Utilities

std::string PrintVisitor::gen_prefix() {
    std::string prefix = "";
    if (depth == 0) return "";
    for (int i = 0; i < depth-1; i++) {
        prefix += " " + full_vert;
    }
    if (is_last) prefix += " " + half_vert;
    else prefix += " " + split_vert;
}

void PrintVisitor::println_with_prefix(std::string_view s) {
    out << gen_prefix() << s << '\n';
}

// Visit method overrides

void PrintVisitor::visit(ast::core::Program const& program) {
    const auto stmt_cnt = program.defstmt_list->size();
    println_with_prefix("Program (" + std::to_string(stmt_cnt) + " statements)");
    depth++;
    for (auto const& stmt : *program.defstmt_list) {
        if (&stmt == &program.defstmt_list->back())
            is_last = true;
        stmt->accept(*this);
    }
    is_last = false;
    depth--;
}
void PrintVisitor::visit(ast::stmt::TypeStmt const& type_stmt) {
    const auto type_cnt = type_stmt.def_list->size();
    println_with_prefix("TypeStmt (" + std::to_string(type_cnt) + " typedefs)");
    depth++;
    for (auto const& type_def : *type_stmt.def_list) {
        if (&type_def == &type_stmt.def_list->back()) 
            is_last = true;
        type_def->accept(*this);
    }
    is_last = false;
    depth--;
}
void PrintVisitor::visit(ast::def::TypeDef const& type_def) {}
void PrintVisitor::visit(ast::stmt::LetStmt const& let_stmt) {}
void PrintVisitor::visit(ast::def::Constant const& cnst) {}
void PrintVisitor::visit(ast::def::Function const& fn) {}
void PrintVisitor::visit(ast::def::Array const& arr) {}
void PrintVisitor::visit(ast::def::Variable const& var) {}

void PrintVisitor::visit(ast::expr::LetIn const& let_in) {}
void PrintVisitor::visit(ast::expr::literal::Unit const& unit) {}
void PrintVisitor::visit(ast::expr::literal::Bool const& boolean) {}
void PrintVisitor::visit(ast::expr::literal::Int const& integer) {}
void PrintVisitor::visit(ast::expr::literal::Char const& chr) {}
void PrintVisitor::visit(ast::expr::literal::Float const& flt) {}
void PrintVisitor::visit(ast::expr::literal::String const& str) {}
void PrintVisitor::visit(ast::expr::op::Binary const& binop) {}
void PrintVisitor::visit(ast::expr::op::Unary const& unop) {}
void PrintVisitor::visit(ast::expr::op::New const& newop) {}
void PrintVisitor::visit(ast::expr::While const& while_expr) {}
void PrintVisitor::visit(ast::expr::For const& for_expr) {}
void PrintVisitor::visit(ast::expr::If const& if_expr) {}
void PrintVisitor::visit(ast::expr::Dim const& dim_expr) {}
void PrintVisitor::visit(ast::expr::IdCall const& id_call) {}
void PrintVisitor::visit(ast::expr::FuncCall const& fn_call) {}
void PrintVisitor::visit(ast::expr::ConstrCall const& cnstr_call) {}
void PrintVisitor::visit(ast::expr::ArrayAccess const& arr_access) {}
void PrintVisitor::visit(ast::expr::Match const& match_expr) {}

void PrintVisitor::visit(ast::annotation::BasicType const& basic_type) {}
void PrintVisitor::visit(ast::annotation::FunctionType const& fn_type) {}
void PrintVisitor::visit(ast::annotation::ArrayType const& arr_type) {}
void PrintVisitor::visit(ast::annotation::RefType const& ref_type) {}
void PrintVisitor::visit(ast::annotation::CustomType const& custom_type) {}

void PrintVisitor::visit(ast::utils::def::Constructor const& constructor) {}
void PrintVisitor::visit(ast::utils::def::Param const& param) {}

void PrintVisitor::visit(ast::utils::match::PatLiteral const& pat_literal) {}
void PrintVisitor::visit(ast::utils::match::PatId const& pat_id) {}
void PrintVisitor::visit(ast::utils::match::PatConstr const& pat_constr) {}
void PrintVisitor::visit(ast::utils::match::Clause const& clause) {}