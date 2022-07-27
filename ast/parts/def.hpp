#ifndef AST_DEF_HPP
#define AST_DEF_HPP

#include <memory>
#include <string>
#include <vector>

#include "../forward.hpp"
#include "core.hpp"
#include "../../utils/utils.hpp"

// Nodes for the various definitions
namespace ast::defs {

    template<typename T>
    struct LetDefCommons : public NodeCommons<T> {
        LetDefCommons(
            std::string id,
            std::unique_ptr<annotation::TypeAnnotation> type_annotation = {}
        ) : id(std::move(id)), type_annotation(std::move(type_annotation)) {}
        std::string id;
        std::unique_ptr<annotation::TypeAnnotation> type_annotation;
    };

    struct Constant : public LetDefCommons<Constant> {
        Constant(
            std::string id,
            std::unique_ptr<exprs::Expression> expr,
            std::unique_ptr<annotation::TypeAnnotation> type_annotation = {}
        ): LetDefCommons(std::move(id), std::move(type_annotation)), expr(std::move(expr)) {}
        std::unique_ptr<exprs::Expression> expr;
    };
    struct Param : public LetDefCommons<Param> {
        using LetDefCommons::LetDefCommons;
    };
    struct Function : public LetDefCommons<Function> {
        Function(
            std::string id,
            std::vector<std::unique_ptr<Param>> params,
            std::unique_ptr<exprs::Expression> body,
            std::unique_ptr<annotation::TypeAnnotation> type_annotation = {}
        ): LetDefCommons(std::move(id), std::move(type_annotation)), params(std::move(params)), body(std::move(body)) {}
        std::vector<std::unique_ptr<Param>> params;
        std::unique_ptr<exprs::Expression> body;
    };
    struct Array : public LetDefCommons<Array> {
        Array(
            std::string id,
            std::vector<std::unique_ptr<exprs::Expression>> dim_exprs,
            std::unique_ptr<annotation::TypeAnnotation> type_annotation = {}
        ) : LetDefCommons(std::move(id), std::move(type_annotation)), dim_exprs(std::move(dim_exprs)) {}
        std::vector<std::unique_ptr<exprs::Expression>> dim_exprs;
    };
    struct Variable : public LetDefCommons<Variable> {
        using LetDefCommons::LetDefCommons;
    };

    struct LetDef : public utils::Variant<
        Constant, Function, Array, Variable
    >, public utils::enable_make_variant<LetDef> {
        using type::type;
    };

    struct Constructor : public NodeCommons<Constructor> {
        Constructor(
            std::string id,
            std::vector<std::unique_ptr<annotation::TypeAnnotation>> param_types
        ) : id(std::move(id)), param_types(std::move(param_types)) {}
        std::string id;
        std::vector<std::unique_ptr<annotation::TypeAnnotation>> param_types;
    };
    struct TypeDef : public NodeCommons<TypeDef> {
        TypeDef(
            std::string id,
            std::vector<std::unique_ptr<Constructor>> constructors
        ) : id(std::move(id)), constructors(std::move(constructors)) {}
        std::string id;
        std::vector<std::unique_ptr<Constructor>> constructors;
    };

}

#endif // AST_UTILS_HPP