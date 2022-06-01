#ifndef __PRINTVISITOR_HPP__

#define __PRINTVISITOR_HPP__
#include <iostream>
#include <string>
#include <vector>
#include <string_view>

#include "../../ast/ast.hpp"


class PrintVisitor : public ast::visit::Visitor {
private:
    std::ostream& out;
    static const std::string full_vert;
    static const std::string split_vert;
    static const std::string half_vert;
    int depth = 0;
    bool is_last = false;
    std::vector<bool> stack;
    std::string gen_prefix();
    void incr_depth();
    void decr_depth();
    void println_with_prefix(std::string_view s);
public:
    PrintVisitor(std::ostream& out = std::cout);
    void visit(ast::core::Program const&) override;
    void visit(ast::stmt::TypeStmt const&) override;
    void visit(ast::def::TypeDef const&) override;
    void visit(ast::stmt::LetStmt const&) override;
    void visit(ast::def::Constant const&) override;
    void visit(ast::def::Function const&) override;
    void visit(ast::def::Array const&) override;
    void visit(ast::def::Variable const&) override;
    
    void visit(ast::expr::LetIn const&) override;
    void visit(ast::expr::literal::Unit const&) override;
    void visit(ast::expr::literal::Bool const&) override;
    void visit(ast::expr::literal::Int const&) override;
    void visit(ast::expr::literal::Char const&) override;
    void visit(ast::expr::literal::Float const&) override;
    void visit(ast::expr::literal::String const&) override;
    void visit(ast::expr::op::Binary const&) override;
    void visit(ast::expr::op::Unary const&) override;
    void visit(ast::expr::op::New const&) override;
    void visit(ast::expr::While const&) override;
    void visit(ast::expr::For const&) override;
    void visit(ast::expr::If const&) override;
    void visit(ast::expr::Dim const&) override;
    void visit(ast::expr::IdCall const&) override;
    void visit(ast::expr::FuncCall const&) override;
    void visit(ast::expr::ConstrCall const&) override;
    void visit(ast::expr::ArrayAccess const&) override;
    void visit(ast::expr::Match const&) override;

    void visit(ast::annotation::BasicType const&) override;
    void visit(ast::annotation::FunctionType const&) override;
    void visit(ast::annotation::ArrayType const&) override;
    void visit(ast::annotation::RefType const&) override;
    void visit(ast::annotation::CustomType const&) override;

    void visit(ast::utils::def::Constructor const& constructor) override;
    void visit(ast::utils::def::Param const& param) override;

    void visit(ast::utils::match::PatLiteral const& pat_literal) override;
    void visit(ast::utils::match::PatId const& pat_id) override;
    void visit(ast::utils::match::PatConstr const& pat_constr) override;
    void visit(ast::utils::match::Clause const& clause) override;
};

#endif // __PRINTVISITOR_HPP__

// Program
// │
// ├─ Stmt    
// │   └─ DefStmt