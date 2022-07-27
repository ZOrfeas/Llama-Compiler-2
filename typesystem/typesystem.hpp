#ifndef TYPESYSTEM_HPP
#define TYPESYSTEM_HPP

#include <string>
#include <vector>
#include <memory>
#include <string_view>

#include "fmt/color.h"
#include "forward.hpp"
#include "../utils/utils.hpp"

//!NOTE: consider singleton pattern for Builtins

namespace typesys {
    inline auto type_enum_to_str(TypeEnum b) -> const char* {
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
        auto to_string() const -> std::string;
        auto operator==(Array const&) const -> bool;
        auto operator!=(Array const&) const -> bool;
    };
    class Ref {
    public:
        static constexpr TypeEnum type_enum = TypeEnum::REF;
        std::shared_ptr<Type> ref_type;
        Ref(std::shared_ptr<Type> ref_type);
        auto to_string() const -> std::string;
        auto operator==(Ref const&) const -> bool;
        auto operator!=(Ref const&) const -> bool;
    };
    class Function {
    public:
        static constexpr TypeEnum type_enum = TypeEnum::FUNCTION;
        std::vector<Type> param_types;
        std::shared_ptr<Type> return_type;
        Function(std::shared_ptr<Type> return_type);
        auto to_string() const -> std::string;
        auto operator==(Function const&) const -> bool;
        auto operator!=(Function const&) const -> bool;
    };
    class Constructor {
    public:
        static constexpr TypeEnum type_enum = TypeEnum::CONSTRUCTOR;
        std::string name;
        Custom const& custom_type;
        std::vector<Type> field_types;
        Constructor(std::string_view name, Custom const& custom_type);
        auto to_string() const -> std::string;
        auto operator==(Constructor const&) const -> bool;
        auto operator!=(Constructor const&) const -> bool;
    };
    class Custom {
    public:
        static constexpr TypeEnum type_enum = TypeEnum::CUSTOM;
        std::string name;
        std::vector<Constructor> constructor_types;
        Custom(std::string_view name);
        auto to_string() const -> std::string;
        auto operator==(Custom const&) const -> bool;
        auto operator!=(Custom const&) const -> bool;
    };
    class Unknown {
    private:
        // TODO: This static disallows "re-entrancy" of the type system.
        // TODO: Wrapping all typesystem "permanent" state in a class, that can be recreated.
        static unsigned long next_id;
    public:
        static constexpr TypeEnum type_enum = TypeEnum::UNKNOWN;
        unsigned long id;
        Unknown();
        auto to_string() const -> std::string;
        auto operator==(Unknown const&) const -> bool;
        auto operator!=(Unknown const&) const -> bool;
    };

    using namespace std::string_literals;
    class Type : public utils::Variant<
        Unit, Int, Char, Bool, Float, Array, Ref,
        Function, Custom, Constructor, Unknown
    > {
    protected:
        using Variant::Variant;
    public:
        template<BuiltinType T>
        static auto get() -> std::shared_ptr<Type> {
            static auto instance = T();
            return std::make_shared<Type>(instance);
        }
        //!Note: Make sure the errors when giving wrong args are not too bad
        template<ComplexType T, typename... Args>
        static auto get(Args&&... args) -> std::shared_ptr<Type> {
            return std::make_shared<Type>(T(
                std::forward<Args>(args)...
            ));
        }
        template<AnyType T>
        T& as(std::string_view caller = "") const {
            return Variant::as<T>(fmt::format(
                "Tried to downcast {} to {} {}", 
                get_type_enum_str(), type_enum_to_str(T::tEnum),
                caller != "" ? " in " + std::string(caller) : ""));
        }
        bool operator==(Type const& other) const;
        bool operator!=(Type const& other) const;

        const char* get_type_enum_str() const;
        friend std::ostream& operator<<(std::ostream&, Type const&);
        std::string to_string() const;
    };


}


#endif // TYPESYSTEM_HPP