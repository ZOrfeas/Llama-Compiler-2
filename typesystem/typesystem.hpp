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
    
    class Array : public Type {
    protected:
        bool equals(Type const* o) const override;
    public:
        static constexpr TypeEnum tEnum = TypeEnum::ARRAY;
        int dimensions;
        std::shared_ptr<int> dim_low_bound_ptr = std::make_shared<int>(0);
        std::shared_ptr<Type> element_type;
        Array(int dimensions, std::shared_ptr<Type> element_type);
        std::string to_string() const override;
    };
    class Ref : public Type {
    protected:
        bool equals(Type const* o) const override;
    public:
        static constexpr TypeEnum tEnum = TypeEnum::REF;
        std::shared_ptr<Type> element_type;
        Ref(std::shared_ptr<Type> element_type);
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
        void add_param(std::shared_ptr<Type> param_type);
        std::string to_string() const override;
    };
    class Custom;
    class Constructor : public Type {
    protected: 
        bool equals(Type const* o) const override;
    public:
        static constexpr TypeEnum tEnum = TypeEnum::RECORD;
        std::string name;
        std::shared_ptr<Custom> custom_type;
        std::vector<std::shared_ptr<Type>> field_types;
        Constructor(std::string_view name);
        void set_custom_type(std::shared_ptr<Custom> owner);
        void add_field(std::shared_ptr<Type> field);
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
        void add_constructor(std::shared_ptr<Constructor> constructor);
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