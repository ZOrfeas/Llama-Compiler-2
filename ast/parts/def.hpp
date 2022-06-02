#ifndef __AST_DEF_HPP__
#define __AST_DEF_HPP__

#include <string>
#include <vector>
#include <memory>

#include "../visitor/visitor.hpp"
#include "./core.hpp"
#include "./utils-def.hpp"

// Nodes for the various definitions
namespace ast::def {
    using std::string;
    using std::vector;
    using std::unique_ptr;
    using core::TypeAnnotation;
    using core::Expression;
    using namespace utils::def;
    
    class Def : public core::Node {
    public:
        string id;
        unique_ptr<TypeAnnotation> type_annotation;
    protected:
        Def(string id, TypeAnnotation *type_annotation = nullptr)
            : id(id), type_annotation(type_annotation) {}
    };
    class TypeDef : public core::Node {
    public:
        string id;
        unique_ptr<vector<unique_ptr<Constructor>>> constructor_list;
        TypeDef(string id, vector<unique_ptr<Constructor>> *constructor_list)
            : id(id), constructor_list(constructor_list) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class Constant : public Def {
    public:
        unique_ptr<Expression> expr;
        Constant(string id, Expression *expr, TypeAnnotation *type_annotation = nullptr)
            : Def(id, type_annotation), expr(unique_ptr<Expression>(expr)) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class Function : public Def {
    public:
        unique_ptr<vector<unique_ptr<Param>>> param_list;
        unique_ptr<Expression> expr;
        Function(
            string id,
            vector<unique_ptr<Param>> *param_list,
            Expression *expr,
            TypeAnnotation *type_annotation = nullptr
        ): Def(id, type_annotation), param_list(param_list), expr(expr) {}
    void accept(visit::Visitor *v) override { v->visit(this); }
    };

    class Mutable : public Def {
    protected: 
        Mutable(string id, TypeAnnotation *type_annotation = nullptr)
            : Def(id, type_annotation) {}
    };
    class Array : public Def {
    public:
        unique_ptr<vector<unique_ptr<Expression>>> dim_expr_list;
        Array(
            string id,
            vector<unique_ptr<Expression>> *dim_expr_list,
            TypeAnnotation *type_annotation = nullptr
        ): Def(id, type_annotation), dim_expr_list(dim_expr_list) {}
    void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class Variable : public Def {
    public:
        Variable(string id, TypeAnnotation *type_annotation = nullptr)
            : Def(id, type_annotation) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
}

#endif // __AST_UTILS_HPP__