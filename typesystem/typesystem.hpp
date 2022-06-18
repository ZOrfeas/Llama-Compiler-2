#ifndef __TYPESYSTEM_HPP__
#define __TYPESYSTEM_HPP__

#include <string>
#include <vector>
#include <memory>
#include <string_view>

#include "./core.hpp"

//!NOTE: consider singleton pattern for Builtins

namespace typesys {
    struct Unit {
        static constexpr TypeEnum type_enum = TypeEnum::UNIT;
    };
    struct Int {
        static constexpr TypeEnum type_enum = TypeEnum::INT;
    };
    struct Char {
        static constexpr TypeEnum type_enum = TypeEnum::CHAR;
    };
    struct Bool {
        static constexpr TypeEnum type_enum = TypeEnum::BOOL;
    };
    struct Float {
        static constexpr TypeEnum type_enum = TypeEnum::FLOAT;
    };
    class Array {
    // class invariant:
    // dim_low_bound_ptr contains 0 if dimensions is exact. (i.e. dim_low_bound_ptr is disabled)
    // otherwise dimensions is to be ignored
    public:
        static constexpr TypeEnum type_enum = TypeEnum::ARRAY;
        std::shared_ptr<int> dim_low_bound_ptr = std::make_shared<int>(0);
        Type element_type;
        int dimensions;
        Array(Type element_type, int dimensions = 0);
        std::string to_string() const;
    };
    class Ref {
    public:
        static constexpr TypeEnum type_enum = TypeEnum::REF;
        Type ref_type;
        Ref(Type ref_type);
        std::string to_string() const;
    };
    class Function {
    public:
        static constexpr TypeEnum type_enum = TypeEnum::FUNCTION;
        std::vector<Type> param_types;
        Type return_type;
        Function(Type return_type);
        std::string to_string() const;
    };
    class Custom {
    public:
        static constexpr TypeEnum type_enum = TypeEnum::CUSTOM;
        class Constructor {
        protected: 
            Constructor(std::string_view name, Custom const& custom_type);
        public:
            std::string name;
            Custom const& custom_type;
            std::vector<Type> field_types;
        };
        std::string name;
        std::vector<Constructor> constructor_types;
        Custom(std::string_view name);
        std::string to_string() const;
    };

    class Unknown {
    private:
        static unsigned long next_id;
    public:
        static constexpr TypeEnum type_enum = TypeEnum::UNKNOWN;
        unsigned long id;
        Unknown();
        std::string to_string() const;
    };
}


#endif // __TYPESYSTEM_HPP__