#ifndef __AST_VISITOR_HPP__
#define __AST_VISITOR_HPP__

#include <string_view>

#include "../forward.hpp"
#include "../../error/error.hpp"

namespace ast::visit {
    class Visitor {
    public:
        virtual void visit(ast::core::Program*) = 0;
        virtual void visit(ast::stmt::TypeStmt*) = 0;
        virtual void visit(ast::def::TypeDef*) = 0;
        virtual void visit(ast::stmt::LetStmt*) = 0;
        virtual void visit(ast::def::Constant*) = 0;
        virtual void visit(ast::def::Function*) = 0;
        virtual void visit(ast::def::Array*) = 0;
        virtual void visit(ast::def::Variable*) = 0;
        virtual void visit(ast::expr::LetIn*) = 0;
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
        virtual void visit(ast::annotation::BasicType*) = 0;
        virtual void visit(ast::annotation::FunctionType*) = 0;
        virtual void visit(ast::annotation::ArrayType*) = 0;
        virtual void visit(ast::annotation::RefType*) = 0;
        virtual void visit(ast::annotation::CustomType*) = 0;
        virtual void visit(ast::utils::def::Constructor*) = 0;
        virtual void visit(ast::utils::def::Param*) = 0;
        virtual void visit(ast::utils::match::PatLiteral*) = 0;
        virtual void visit(ast::utils::match::PatId*) = 0;
        virtual void visit(ast::utils::match::PatConstr*) = 0;
        virtual void visit(ast::utils::match::Clause*) = 0;
        virtual ~Visitor() = default;
    };
    class RelaxedVisitor : public Visitor {
    private:
        void crash(std::string_view node_type) {
            error::crash<error::INTERNAL>(
                "Visitor {}, called on node of type {}", visitor_name, node_type
            );
        }
        std::string_view visitor_name;
    public:
        RelaxedVisitor(std::string_view name): visitor_name(name) {}
        void visit(ast::core::Program*) override { crash("Program"); }
        void visit(ast::stmt::TypeStmt*) override { crash("TypeStmt"); }
        void visit(ast::def::TypeDef*) override { crash("TypeDef"); }
        void visit(ast::stmt::LetStmt*) override { crash("LetStmt"); }
        void visit(ast::def::Constant*) override { crash("Constant"); }
        void visit(ast::def::Function*) override { crash("Function"); }
        void visit(ast::def::Array*) override { crash("Array"); }
        void visit(ast::def::Variable*) override { crash("Variable"); }
        void visit(ast::expr::LetIn*) override { crash("LetIn"); }
        void visit(ast::expr::literal::Unit*) override { crash("Unit"); }
        void visit(ast::expr::literal::Int*) override { crash("Int"); }
        void visit(ast::expr::literal::Char*) override { crash("Char"); }
        void visit(ast::expr::literal::Bool*) override { crash("Bool"); }
        void visit(ast::expr::literal::Float*) override { crash("Float"); }
        void visit(ast::expr::literal::String*) override { crash("String"); }
        void visit(ast::expr::op::Binary*) override { crash("Binary"); }
        void visit(ast::expr::op::Unary*) override { crash("Unary"); }
        void visit(ast::expr::op::New*) override { crash("New"); }
        void visit(ast::expr::While*) override { crash("While"); }
        void visit(ast::expr::For*) override { crash("For"); }
        void visit(ast::expr::If*) override { crash("If"); }
        void visit(ast::expr::Dim*) override { crash("Dim"); }
        void visit(ast::expr::IdCall*) override { crash("IdCall"); }
        void visit(ast::expr::FuncCall*) override { crash("FuncCall"); }
        void visit(ast::expr::ConstrCall*) override { crash("ConstrCall"); }
        void visit(ast::expr::ArrayAccess*) override { crash("ArrayAccess"); }
        void visit(ast::expr::Match*) override { crash("Match"); }
        void visit(ast::annotation::BasicType*) override { crash("BasicType"); }
        void visit(ast::annotation::FunctionType*) override { crash("FunctionType"); }
        void visit(ast::annotation::ArrayType*) override { crash("ArrayType"); }
        void visit(ast::annotation::RefType*) override { crash("RefType"); }
        void visit(ast::annotation::CustomType*) override { crash("CustomType"); }
        void visit(ast::utils::def::Constructor*) override { crash("Constructor"); }
        void visit(ast::utils::def::Param*) override { crash("Param"); }
        void visit(ast::utils::match::PatLiteral*) override { crash("PatLiteral"); }
        void visit(ast::utils::match::PatId*) override { crash("PatId"); }
        void visit(ast::utils::match::PatConstr*) override { crash("PatConstr"); }
        void visit(ast::utils::match::Clause*) override { crash("Clause"); }
    };
}

#endif // __AST_VISITOR_HPP__