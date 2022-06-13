#ifndef __TYPES__HPP__
#define __TYPES__HPP__

#include <vector>
#include <memory>
#include <string_view>

//!Note(orf): template method `as` that is used to safely downcast

namespace typesys {
    enum class TypeEnum {
        UNIT, INT, CHAR, BOOL, FLOAT,
        ARRAY, REF, FUNCTION, CUSTOM, UNKNOWN
    };
    inline const char* type_name(TypeEnum b) {
        static const char* builtin_name[] = {
            "unit", "int", "char", "bool", "float",
            "array", "ref", "function", "custom", "unknown"
        };
        return builtin_name[static_cast<int>(b)];
    }
    class Type {
    protected:
        Type(TypeEnum t): type(t) {}
    public:
        virtual ~Type() = default;
        TypeEnum type;
    };
    class Builtin : public Type {
    protected:
        Builtin(TypeEnum b) : Type(b) {}
    };
    class Unit : public Builtin {
    public:
        Unit() : Builtin(TypeEnum::UNIT) {}
    };
    class Int : public Builtin {
    public:
        Int() : Builtin(TypeEnum::INT) {}
    };
    class Char : public Builtin {
    public:
        Char() : Builtin(TypeEnum::CHAR) {}
    };
    class Bool : public Builtin {
    public:
        Bool() : Builtin(TypeEnum::BOOL) {}
    };
    class Float : public Builtin {
    public:
        Float() : Builtin(TypeEnum::FLOAT) {}
    };
    
    class Array : public Type {
    public:
        int dimensions;
        std::unique_ptr<Type> element_type;
        Array(int dimensions, Type* element_type)
            :   Type(TypeEnum::ARRAY),
                dimensions(dimensions),
                element_type(element_type) {}
    };
    class Ref : public Type {
    public:
        std::unique_ptr<Type> element_type;
        Ref(Type* element_type)
            :   Type(TypeEnum::REF),
                element_type(element_type) {}
    };
    class Function : public Type {
    public:
        std::vector<std::unique_ptr<Type>> param_types;
        std::unique_ptr<Type> return_type;
        // Function()
    };
    class Custom : public Type {};

    class Unknown : public Type {};
}

#endif // __TYPES__HPP__