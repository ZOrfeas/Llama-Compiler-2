#ifndef __SEM_HPP__
#define __SEM_HPP__

#include "../../ast/visitor/visitor.hpp"

class SemVisitor : public ast::visit::Visitor {
private:
    
public:
    SemVisitor();
    void visit(ast::core::Program*) override;
    void visit(ast::stmt::TypeStmt*) override;
    void visit(ast::def::TypeDef*) override;
    void visit(ast::stmt::LetStmt*) override;
    void visit(ast::def::Constant*) override;
    void visit(ast::def::Function*) override;
    void visit(ast::def::Array*) override;
    void visit(ast::def::Variable*) override;
    
    void visit(ast::expr::LetIn*) override;
    void visit(ast::expr::literal::Unit*) override;
    void visit(ast::expr::literal::Bool*) override;
    void visit(ast::expr::literal::Int*) override;
    void visit(ast::expr::literal::Char*) override;
    void visit(ast::expr::literal::Float*) override;
    void visit(ast::expr::literal::String*) override;
    void visit(ast::expr::op::Binary*) override;
    void visit(ast::expr::op::Unary*) override;
    void visit(ast::expr::op::New*) override;
    void visit(ast::expr::While*) override;
    void visit(ast::expr::For*) override;
    void visit(ast::expr::If*) override;
    void visit(ast::expr::Dim*) override;
    void visit(ast::expr::IdCall*) override;
    void visit(ast::expr::FuncCall*) override;
    void visit(ast::expr::ConstrCall*) override;
    void visit(ast::expr::ArrayAccess*) override;
    void visit(ast::expr::Match*) override;

    void visit(ast::annotation::BasicType*) override;
    void visit(ast::annotation::FunctionType*) override;
    void visit(ast::annotation::ArrayType*) override;
    void visit(ast::annotation::RefType*) override;
    void visit(ast::annotation::CustomType*) override;

    void visit(ast::utils::def::Constructor*) override;
    void visit(ast::utils::def::Param*) override;

    void visit(ast::utils::match::PatLiteral*) override;
    void visit(ast::utils::match::PatId*) override;
    void visit(ast::utils::match::PatConstr*) override;
    void visit(ast::utils::match::Clause*) override;
};

#endif // __SEM_HPP__