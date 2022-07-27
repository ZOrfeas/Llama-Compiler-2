#ifndef AST_ANNOTATION_HPP
#define AST_ANNOTATION_HPP

#include <memory>
#include <string>
#include <utility>

#include "core.hpp"
#include "../forward.hpp"
#include "../../typesystem/forward.hpp"
#include "../../utils/utils.hpp"

namespace ast::annotation {

    struct BasicType : public NodeCommons<BasicType> {
        BasicType(typesys::TypeEnum t) : t(t) {}
        typesys::TypeEnum t;
    };
    struct FunctionType : public NodeCommons<FunctionType> {
        FunctionType(
            std::unique_ptr<TypeAnnotation> lhs,
            std::unique_ptr<TypeAnnotation> rhs)
        : lhs(std::move(lhs)), rhs(std::move(rhs)) {}
        std::unique_ptr<TypeAnnotation> lhs, rhs;        
    };
    struct ArrayType : public NodeCommons<ArrayType> {
        ArrayType(int dims, std::unique_ptr<TypeAnnotation> elem_type)
            : dims(dims), elem_type(std::move(elem_type)) {}
        int dims;
        std::unique_ptr<TypeAnnotation> elem_type;
    };
    struct RefType : public NodeCommons<RefType> {
        RefType(std::unique_ptr<TypeAnnotation> elem_type)
            : elem_type(std::move(elem_type)) {}
        std::unique_ptr<TypeAnnotation> elem_type;
    };
    struct CustomType : public NodeCommons<CustomType> {
        CustomType(std::string name): name(std::move(name)) {}
        std::string name;
    };

    ////

    struct TypeAnnotation : public utils::Variant<
        BasicType, FunctionType, ArrayType, RefType, CustomType
    >, public utils::enable_make_variant<TypeAnnotation> {
        using type::type;
    };
    
} // namespace annotation

#endif // AST_ANNOTATION_HPP