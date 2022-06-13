#ifndef __AST_UTILSDEF_HPP__
#define __AST_UTILSDEF_HPP__

#include <vector>
#include <memory>

#include "../visitor/visitor.hpp"
#include "./core.hpp"

namespace ast::utils::def {
    using std::vector;
    using std::unique_ptr;
    using std::string;
    using core::TypeAnnotation;
    class Constructor : public core::Node {
    public:
        string id;
        unique_ptr<vector<unique_ptr<TypeAnnotation>>> type_list;
        Constructor(string id, vector<unique_ptr<TypeAnnotation>> *type_list):
            id(id), type_list(type_list) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class Param : public core::Node {
    public:
        string id;
        unique_ptr<TypeAnnotation> type_annotation;
        Param(string id, TypeAnnotation *type_annotation = nullptr):
            id(id), type_annotation(type_annotation) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
}

#endif // __AST_UTILSDEF_HPP__