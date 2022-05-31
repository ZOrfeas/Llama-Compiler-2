#ifndef __AST_VISITOR_HPP__
#define __AST_VISITOR_HPP__

#include "../forward.hpp"

namespace ast::visit {
    class Visitor {
    public:
        virtual void visit(ast::core::Program const&) = 0;

        // ast::stmt & ast::def

        virtual void visit(ast::stmt::TypeStmt const&) = 0;
        virtual void visit(ast::def::TypeDef const&) = 0;

        virtual void visit(ast::stmt::LetStmt const&) = 0;
        // virtual void visit(ast::def::Def const&) = 0;
        virtual void visit(ast::def::Constant const&) = 0;
        virtual void visit(ast::def::Function const&) = 0;
        // virtual void visit(ast::def::Mutable const&) = 0;
        virtual void visit(ast::def::Array const&) = 0;
        virtual void visit(ast::def::Variable const&) = 0;

        // ast::expr

        // virtual void visit(ast::core::Expression const&) = 0;
        virtual void visit(ast::expr::LetIn const&) = 0;
        // virtual void visit(ast::expr::Literal const&) = 0;
        virtual void visit(ast::expr::literal::Unit const&) = 0;
        virtual void visit(ast::expr::literal::Int const&) = 0;
        virtual void visit(ast::expr::literal::Char const&) = 0;
        virtual void visit(ast::expr::literal::Bool const&) = 0;
        virtual void visit(ast::expr::literal::Float const&) = 0;
        virtual void visit(ast::expr::literal::String const&) = 0;
        virtual void visit(ast::expr::op::Binary const&) = 0;
        virtual void visit(ast::expr::op::Unary const&) = 0;
        virtual void visit(ast::expr::op::New const&) = 0;
        virtual void visit(ast::expr::While const&) = 0;
        virtual void visit(ast::expr::For const&) = 0;
        virtual void visit(ast::expr::If const&) = 0;
        virtual void visit(ast::expr::Dim const&) = 0;
        virtual void visit(ast::expr::IdCall const&) = 0;
        virtual void visit(ast::expr::FuncCall const&) = 0;
        virtual void visit(ast::expr::ConstrCall const&) = 0;
        virtual void visit(ast::expr::ArrayAccess const&) = 0;
        virtual void visit(ast::expr::Match const&) = 0;

        // ast::annotation
        virtual void visit(ast::annotation::BasicType const&) = 0;
        virtual void visit(ast::annotation::FunctionType const&) = 0;
        virtual void visit(ast::annotation::ArrayType const&) = 0;
        virtual void visit(ast::annotation::RefType const&) = 0;
        virtual void visit(ast::annotation::CustomType const&) = 0;

        // ast::utils::def
        virtual void visit(ast::utils::def::Constructor const&) = 0;
        virtual void visit(ast::utils::def::Param const&) = 0;

        // ast::utils::match
        // virtual void visit(ast::utils::match::Pattern const&) = 0;
        virtual void visit(ast::utils::match::PatLiteral const&) = 0;
        virtual void visit(ast::utils::match::PatId const&) = 0;
        virtual void visit(ast::utils::match::PatConstr const&) = 0;
        virtual void visit(ast::utils::match::Clause const&) = 0;

        virtual ~Visitor() = default;
    };
}

#endif // __AST_VISITOR_HPP__