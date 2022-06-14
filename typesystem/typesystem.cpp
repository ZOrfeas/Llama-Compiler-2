#include "./typesystem.hpp"

#include <type_traits>

namespace typesys {
    const char* type_name(TypeEnum b) {
        static const char* builtin_name[] = {
            "unit", "int", "char", "bool", "float",
            "array", "ref", "function", "custom", "unknown"
        };
        return builtin_name[static_cast<int>(b)];
    }

    Type::Type(TypeEnum t): type(t) {}
    Builtin::Builtin(TypeEnum b): Type(b) {}
    
    Unit::Unit(): Builtin(TypeEnum::UNIT) {}
    Int::Int(): Builtin(TypeEnum::INT) {}
    Char::Char(): Builtin(TypeEnum::CHAR) {}
    Bool::Bool(): Builtin(TypeEnum::BOOL) {}
    Float::Float(): Builtin(TypeEnum::FLOAT) {}

    Array::Array(int dimensions, Type* element_type):
        Type(TypeEnum::ARRAY),
        dimensions(dimensions),
        element_type(element_type) {}
    Ref::Ref(Type* element_type):
        Type(TypeEnum::REF),
        element_type(element_type) {}
    Function::Function(Type* return_type):
        Type(TypeEnum::FUNCTION),
        return_type(return_type) {}
    void Function::add_param_type(Type* param_type) {
        param_types.push_back(std::unique_ptr<Type>(param_type));
    }
    Custom::Custom(std::string_view name):
        Type(TypeEnum::CUSTOM),
        name(name) {}
    unsigned long Unknown::next_id = 0;
    Unknown::Unknown(): Type(TypeEnum::UNKNOWN), id(next_id++) {}
}