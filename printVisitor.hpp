#ifndef __PRINTVISITOR_HPP__
#define __PRINTVISITOR_HPP__

#include <iostream>
#include <string>

#include "ast/ast.hpp"

class PrintVisitor : public ast::visit::Visitor {
private:
    int depth = 0;
    bool is_last = false;
    static const std::string full_vert = "│ ";
    static const std::string split_vert = "├─";
    static const std::string half_vert =  "└─";
    // static const std::string horizontal = "─";
    std::string gen_prefix() {
        std::string prefix = "";
        if (depth == 0) return "";
        for (int i = 0; i < depth-1; i++) {
            prefix += " " + full_vert;
        }
        if (is_last) prefix += " " + half_vert;
        else prefix += " " + split_vert;
    }
    void println_with_prefix(std::string s) {
        std::cout << gen_prefix() << s << '\n';
    }
public:
    void visit(ast::core::Program const& program) override {
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
    void visit(ast::stmt::TypeStmt const& type_stmt) override {
        const auto type_cnt = type_stmt.def_list->size();
        println_with_prefix("TypeStmt (" std::to_string(type_cnt) + " typedefs)");
        depth++;
        for (auto const& type_def : *type_stmt.def_list) {
            if (&type_def == &type_stmt.def_list->back()) 
                is_last = true;
            type_def->accept(*this);
        }
        is_last = false;
        depth--;
    }
    void visit(ast::def::TypeDef const& type_def) override;
    void visit(ast::stmt::LetStmt const& let_stmt) override;
    void visit(ast::def::Constant const& cnst) override;
    void visit(ast::def::Function const& fn) override;
    void visit(ast::def::Array const& arr) override;
    void visit(ast::def::Variable const& var) override;
    
    void visit(ast::expr::LetIn const& let_in) override;
    void visit(ast::expr::literal::Unit const& unit) override;
    void visit(ast::expr::literal::Bool const& boolean) override;
    void visit(ast::expr::literal::Int const& integer) override;
    void visit(ast::expr::literal::Char const& chr) override;
    void visit(ast::expr::literal::Float const& flt) override;
    void visit(ast::expr::literal::String const& str) override;

    void visit(ast::expr::op::Binary const& binop) override;
    void visit(ast::expr::op::Unary const& unop) override;
    void visit(ast::expr::op::New const& newop) override;
    void visit(ast::expr::While const& while_expr) override;
    void visit(ast::expr::For const& for_expr) override;
    void visit(ast::expr::If const& if_expr) override;
    void visit(ast::expr::Dim const& dim_expr) override;
    void visit(ast::expr::IdCall const& id_call) override;
    void visit(ast::expr::FuncCall const& fn_call) override;
    void visit(ast::expr::ConstrCall const& constr_call) override;
    void visit(ast::expr::ArrayAccess const& array_access) override;
    void visit(ast::expr::Match const& match_expr) override;

    void visit(ast::annotation::BasicType const& basic_type) override;
    void visit(ast::annotation::FunctionType const& fn_type) override;
    void visit(ast::annotation::ArrayType const& arr_type) override;
    void visit(ast::annotation::RefType const& ref_type) override;
    void visit(ast::annotation::CustomType const& custom_type) override;

    void visit(ast::utils::def::Constructor const& constructor) override;
    void visis(ast::utils::def::Param const& param) override;

    void visit(ast::utils::match::Pattern const& pattern) override;
    void visit(ast::utils::match::PatLiteral const& pat_literal) override;
    void visit(ast::utils::match::PatId const& pat_id) override;
    void visit(ast::utils::match::PatConstr const& pat_constr) override;
    void visit(ast::utils::match::Clause const& clause) override;
}

#endif // __PRINTVISITOR_HPP__

// Program
// │
// ├─ Stmt    
// │   └─ DefStmt