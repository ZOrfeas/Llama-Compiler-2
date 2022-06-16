#ifndef __TYPESYSTEM_HPP__
#define __TYPESYSTEM_HPP__

#include <string>
#include <vector>
#include <memory>
#include <string_view>

#include "./core.hpp"

//!NOTE: consider singleton pattern for Builtins

namespace typesys {
    class Builtin : public Type {
    protected:
        Builtin(TypeEnum b);
        bool equals(Type const* o) const override;
    public:
        std::string to_string() const override;
    };

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
    // class invariant:
    // dim_low_bound_ptr contains 0 if dimensions is exact. (i.e. dim_low_bound_ptr is disabled)
    // otherwise dimensions is to be ignored
    class Array : public Type {
    protected:
        bool equals(Type const* o) const override;
    public:
        static constexpr TypeEnum tEnum = TypeEnum::ARRAY;
        std::shared_ptr<int> dim_low_bound_ptr = std::make_shared<int>(0);
        std::shared_ptr<Type> element_type;
        int dimensions;
        Array(std::shared_ptr<Type> element_type, int dimensions = 0);
        std::string to_string() const override;
    };
    class Ref : public Type {
    protected:
        bool equals(Type const* o) const override;
    public:
        static constexpr TypeEnum tEnum = TypeEnum::REF;
        std::shared_ptr<Type> ref_type;
        Ref(std::shared_ptr<Type> ref_type);
        std::string to_string() const override;
    };
    class Function : public Type {
    protected:
        bool equals(Type const* o) const override;
    public:
        static constexpr TypeEnum tEnum = TypeEnum::FUNCTION;
        std::vector<std::shared_ptr<Type>> param_types;
        std::shared_ptr<Type> return_type;
        Function(std::shared_ptr<Type> return_type);
        std::string to_string() const override;
    };
    class Constructor : public Type {
    protected: 
        bool equals(Type const* o) const override;
    public:
        static constexpr TypeEnum tEnum = TypeEnum::RECORD;
        std::string name;
        std::shared_ptr<Custom> custom_type;
        std::vector<std::shared_ptr<Type>> field_types;
        Constructor(std::string_view name);
        std::string to_string() const override;
    };
    class Custom : public Type {
    protected:
        bool equals(Type const* o) const override;
    public:
        static constexpr TypeEnum tEnum = TypeEnum::CUSTOM;
        std::string name;
        std::vector<std::shared_ptr<Constructor>> constructor_types;
        Custom(std::string_view name);
        std::string to_string() const override;
    };

    class Unknown : public Type {
    private:
        static unsigned long next_id;
    protected:
        bool equals(Type const* o) const override;
    public:
        static constexpr TypeEnum tEnum = TypeEnum::UNKNOWN;
        unsigned long id;
        Unknown();
        std::string to_string() const override;
    };
}


#endif // __TYPESYSTEM_HPP__