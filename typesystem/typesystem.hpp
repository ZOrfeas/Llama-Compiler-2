#ifndef __TYPESYSTEM_HPP__
#define __TYPESYSTEM_HPP__

#include <string>
#include <vector>
#include <memory>
#include <string_view>

//!Note(orf): template method `as` that is used to safely downcast

namespace typesys {
    enum class TypeEnum {
        UNIT, INT, CHAR, BOOL, FLOAT,
        ARRAY, REF, FUNCTION, CUSTOM, UNKNOWN
    };
    const char* type_name(TypeEnum b);

    class Type {
    protected:
        Type(TypeEnum t);
    public:
        virtual ~Type() = default;
        TypeEnum type;
    };
    class Builtin : public Type {
    protected:
        Builtin(TypeEnum b);        
    };
    //!NOTE: consider singleton pattern
    class Unit : public Builtin {
    public:
        static constexpr TypeEnum tEnum = TypeEnum::UNIT;
        Unit();
    };
    class Int : public Builtin {
    public:
        static constexpr TypeEnum tEnum = TypeEnum::INT;
        Int();
    };
    class Char : public Builtin {
    public:
        static constexpr TypeEnum tEnum = TypeEnum::CHAR;
        Char();
    };
    class Bool : public Builtin {
    public:
        static constexpr TypeEnum tEnum = TypeEnum::BOOL;
        Bool();
    };
    class Float : public Builtin {
    public:
        static constexpr TypeEnum tEnum = TypeEnum::FLOAT;
        Float();
    };
    
    class Array : public Type {
    public:
        static constexpr TypeEnum tEnum = TypeEnum::ARRAY;
        int dimensions;
        std::unique_ptr<Type> element_type;
        Array(int dimensions, Type* element_type);
    };
    class Ref : public Type {
    public:
        static constexpr TypeEnum tEnum = TypeEnum::REF;
        std::unique_ptr<Type> element_type;
        Ref(Type* element_type);
    };
    class Function : public Type {
    public:
        static constexpr TypeEnum tEnum = TypeEnum::FUNCTION;
        std::vector<std::unique_ptr<Type>> param_types;
        std::unique_ptr<Type> return_type;
        Function(Type* return_type);
        void add_param_type(Type* param_type);
    };
    class Custom : public Type {
    public:
        static constexpr TypeEnum tEnum = TypeEnum::CUSTOM;
        std::string name;
        Custom(std::string_view name);
    };

    class Unknown : public Type {
    private:
        static unsigned long next_id;
    public:
        static constexpr TypeEnum tEnum = TypeEnum::UNKNOWN;
        unsigned long id;
        Unknown();
    };
}


#endif // __TYPESYSTEM_HPP__