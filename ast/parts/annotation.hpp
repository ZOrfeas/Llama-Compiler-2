#ifndef __AST_ANNOTATION_HPP__
#define __AST_ANNOTATION_HPP__

#include <string>
#include <vector>
#include <memory>

#include "../../types.hpp"
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
    };
    class FunctionType : public TypeAnnotation {
    public:
        unique_ptr<TypeAnnotation> lhs,rhs;
        FunctionType(TypeAnnotation *lhs, TypeAnnotation *rhs)
            : lhs(lhs), rhs(rhs) {};
    };
    class ArrayType : public TypeAnnotation {
    public:
        ssize_t dims;
        unique_ptr<TypeAnnotation> contained_type;
        ArrayType(ssize_t dimensions, TypeAnnotation *contained_type)
            : dims(dimensions), contained_type(contained_type) {};
    };
    class RefType : public TypeAnnotation {
    public:
        unique_ptr<TypeAnnotation> contained_type;
        RefType(TypeAnnotation *contained_type)
            : contained_type(contained_type) {};
    };
    class CustomType : public TypeAnnotation {
    public:
        string id;
        CustomType(string id): id(id) {};
    };
} // namespace annotation

#endif // __AST_ANNOTATION_HPP__