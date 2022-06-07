#ifndef __AST_VISITOR_HPP__
#define __AST_VISITOR_HPP__

#include "../forward.hpp"

namespace ast::visit {
    class Visitor {
    public:
        virtual void visit(ast::core::Program*) = 0;

        // ast::stmt & ast::def

        virtual void visit(ast::stmt::TypeStmt*) = 0;
        virtual void visit(ast::def::TypeDef*) = 0;

        virtual void visit(ast::stmt::LetStmt*) = 0;
        // virtual void visit(ast::def::Def*) = 0;
        virtual void visit(ast::def::Constant*) = 0;
        virtual void visit(ast::def::Function*) = 0;
        // virtual void visit(ast::def::Mutable*) = 0;
        virtual void visit(ast::def::Array*) = 0;
        virtual void visit(ast::def::Variable*) = 0;

        // ast::expr

        // virtual void visit(ast::core::Expression*) = 0;
        virtual void visit(ast::expr::LetIn*) = 0;
        // virtual void visit(ast::expr::Literal*) = 0;
        virtual void visit(ast::expr::literal::Unit*) = 0;
        virtual void visit(ast::expr::literal::Int*) = 0;
        virtual void visit(ast::expr::literal::Char*) = 0;
        virtual void visit(ast::expr::literal::Bool*) = 0;
        virtual void visit(ast::expr::literal::Float*) = 0;
        virtual void visit(ast::expr::literal::String*) = 0;
        virtual void visit(ast::expr::op::Binary*) = 0;
        virtual void visit(ast::expr::op::Unary*) = 0;
        virtual void visit(ast::expr::op::New*) = 0;
        virtual void visit(ast::expr::While*) = 0;
        virtual void visit(ast::expr::For*) = 0;
        virtual void visit(ast::expr::If*) = 0;
        virtual void visit(ast::expr::Dim*) = 0;
        virtual void visit(ast::expr::IdCall*) = 0;
        virtual void visit(ast::expr::FuncCall*) = 0;
        virtual void visit(ast::expr::ConstrCall*) = 0;
        virtual void visit(ast::expr::ArrayAccess*) = 0;
        virtual void visit(ast::expr::Match*) = 0;

        // ast::annotation
        virtual void visit(ast::annotation::BasicType*) = 0;
        virtual void visit(ast::annotation::FunctionType*) = 0;
        virtual void visit(ast::annotation::ArrayType*) = 0;
        virtual void visit(ast::annotation::RefType*) = 0;
        virtual void visit(ast::annotation::CustomType*) = 0;

        // ast::utils::def
        virtual void visit(ast::utils::def::Constructor*) = 0;
        virtual void visit(ast::utils::def::Param*) = 0;

        // ast::utils::match
        // virtual void visit(ast::utils::match::Pattern*) = 0;
        virtual void visit(ast::utils::match::PatLiteral*) = 0;
        virtual void visit(ast::utils::match::PatId*) = 0;
        virtual void visit(ast::utils::match::PatConstr*) = 0;
        virtual void visit(ast::utils::match::Clause*) = 0;

        virtual ~Visitor() = default;
    };
}

#endif // __AST_VISITOR_HPP__