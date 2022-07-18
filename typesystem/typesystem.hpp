#ifndef TYPESYSTEM_HPP
#define TYPESYSTEM_HPP

#include <string>
#include <vector>
#include <memory>
#include <string_view>

#include "forward.hpp"
#include "../utils/utils.hpp"

//!NOTE: consider singleton pattern for Builtins

namespace typesys {
    inline const char* type_enum_to_str(TypeEnum b) {
        static const char* builtin_name[] = {
            "unit", "int", "char", "bool", "float",
            "array", "ref", "function", "custom", 
            "constructor", "unknown"
        };
        return builtin_name[static_cast<int>(b)];
    }
    template<typename T>
    concept BuiltinType = std::is_same_v<T, Unit> ||
                          std::is_same_v<T, Int> ||
                          std::is_same_v<T, Char> ||
                          std::is_same_v<T, Bool> ||
                          std::is_same_v<T, Float>;
    template<typename T>
    concept ComplexType = std::is_same_v<T, Array> ||
                          std::is_same_v<T, Ref> ||
                          std::is_same_v<T, Function> ||
                          std::is_same_v<T, Custom> ||
                          std::is_same_v<T, Constructor> ||
                          std::is_same_v<T, Unknown>;
    template<typename T>
    concept AnyType = BuiltinType<T> || ComplexType<T>;
    
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
        std::shared_ptr<Type> element_type;
        int dimensions;
        Array(std::shared_ptr<Type> element_type, int dimensions = 0);
        std::string to_string() const;
        bool operator==(Array const&) const;
        bool operator!=(Array const&) const;
    };
    class Ref {
    public:
        static constexpr TypeEnum type_enum = TypeEnum::REF;
        std::shared_ptr<Type> ref_type;
        Ref(std::shared_ptr<Type> ref_type);
        std::string to_string() const;
        bool operator==(Ref const&) const;
        bool operator!=(Ref const&) const;
    };
    class Function {
    public:
        static constexpr TypeEnum type_enum = TypeEnum::FUNCTION;
        std::vector<Type> param_types;
        std::shared_ptr<Type> return_type;
        Function(std::shared_ptr<Type> return_type);
        std::string to_string() const;
        bool operator==(Function const&) const;
        bool operator!=(Function const&) const;
    };
    class Constructor {
    public:
        static constexpr TypeEnum type_enum = TypeEnum::CONSTRUCTOR;
        std::string name;
        Custom const& custom_type;
        std::vector<Type> field_types;
        Constructor(std::string_view name, Custom const& custom_type);
        std::string to_string() const;
        bool operator==(Constructor const&) const;
        bool operator!=(Constructor const&) const;
    };

    class Custom {
    public:
        static constexpr TypeEnum type_enum = TypeEnum::CUSTOM;
        std::string name;
        std::vector<Constructor> constructor_types;
        Custom(std::string_view name);
        std::string to_string() const;
        bool operator==(Custom const&) const;
        bool operator!=(Custom const&) const;
    };
    class Unknown {
    private:
        static unsigned long next_id;
    public:
        static constexpr TypeEnum type_enum = TypeEnum::UNKNOWN;
        unsigned long id;
        Unknown();
        std::string to_string() const;
        bool operator==(Unknown const&) const;
        bool operator!=(Unknown const&) const;
    };

    using namespace std::string_literals;
    class Type : public utils::Variant<
        Unit, Int, Char, Bool, Float, Array, Ref,
        Function, Custom, Constructor, Unknown
    > {
    protected:
        using Base = utils::Variant<
            Unit, Int, Char, Bool, Float, Array, Ref,
            Function, Custom, Constructor, Unknown
        >;
        using Base::Base;
    public:
        template<BuiltinType T>
        static std::shared_ptr<Type> get() {
            static auto instance = T();
            return std::make_shared<Type>(instance);
        }
        //!Note: Make sure the errors when giving wrong args are not too bad
        template<ComplexType T, typename... Args>
        static std::shared_ptr<Type> get(Args&&... args) {
            return std::make_shared<Type>(T(
                std::forward<Args>(args)...
            ));
        }
        template<AnyType T>
        T& as(std::string_view caller = "") const {
            const auto msg = spdlog::fmt_lib::format(
               "Tried to downcast {} to {} {}",
                get_type_enum_str(), type_enum_to_str(T::tEnum),
                caller != "" ? " in " + std::string(caller) : ""
            );
            return Base::as<T>(msg);
        }
        bool operator==(Type const& other) const;
        bool operator!=(Type const& other) const;

        const char* get_type_enum_str() const;
        friend std::ostream& operator<<(std::ostream&, Type const&);
        std::string to_string() const;
    };


}


#endif // TYPESYSTEM_HPP