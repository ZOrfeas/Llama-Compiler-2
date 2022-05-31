#ifndef __AST_ANNOTATION_HPP__
#define __AST_ANNOTATION_HPP__

#include <string>
#include <vector>
#include <memory>

#include "../../types.hpp"
#include "../visitor/visitor.hpp"
#include "./core.hpp"

// Nodes for explicitly notated types
namespace ast::annotation {
    using core::TypeAnnotation;
    using std::string;
    using std::vector;
    using std::unique_ptr;

    class BasicType : public TypeAnnotation {
    public:
        types::Builtin t;
        BasicType(types::Builtin t): t(t) {};
        void accept(visit::Visitor &v) override { v.visit(*this); }
    };
    class FunctionType : public TypeAnnotation {
    public:
        unique_ptr<TypeAnnotation> lhs,rhs;
        FunctionType(TypeAnnotation *lhs, TypeAnnotation *rhs)
            : lhs(lhs), rhs(rhs) {};
            void accept(visit::Visitor &v) override { v.visit(*this); }
    };
    class ArrayType : public TypeAnnotation {
    public:
        ssize_t dims;
        unique_ptr<TypeAnnotation> contained_type;
        ArrayType(ssize_t dimensions, TypeAnnotation *contained_type)
            : dims(dimensions), contained_type(contained_type) {};
            void accept(visit::Visitor &v) override { v.visit(*this); }
    };
    class RefType : public TypeAnnotation {
    public:
        unique_ptr<TypeAnnotation> contained_type;
        RefType(TypeAnnotation *contained_type)
            : contained_type(contained_type) {};
            void accept(visit::Visitor &v) override { v.visit(*this); }
    };
    class CustomType : public TypeAnnotation {
    public:
        string id;
        CustomType(string id): id(id) {};
        void accept(visit::Visitor &v) override { v.visit(*this); }
    };
} // namespace annotation

#endif // __AST_ANNOTATION_HPP__