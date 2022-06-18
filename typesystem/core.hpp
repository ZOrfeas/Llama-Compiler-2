#ifndef __TYPESYSTEM_CORE_HPP__
#define __TYPESYSTEM_CORE_HPP__

#include <string_view>
#include <variant>
#include <concepts>

#include "../../error/error.hpp"
#include "./forward.hpp"

//!Note: I do not like to have to implement templates in headers ... :(

namespace typesys {
    enum class TypeEnum {
        UNIT, INT, CHAR, BOOL, FLOAT,
        ARRAY, REF, FUNCTION, CUSTOM, UNKNOWN
    };
    inline const char* type_enum_to_str(TypeEnum b) {
        static const char* builtin_name[] = {
            "unit", "int", "char", "bool", "float",
            "array", "ref", "function", "custom", "unknown"
        };
        return builtin_name[static_cast<int>(b)];
    }

    // TODO(orf): Move this ↓↓
        template <typename T, template <typename...> class Z>
        struct is_specialization_of : std::false_type {};
        template <typename... Args, template <typename...> class Z>
        struct is_specialization_of<Z<Args...>, Z> : std::true_type {};
        template <typename T, template <typename...> class Z>
        inline constexpr bool is_specialization_of_v = is_specialization_of<T,Z>::value;
        template <typename T>
        concept IsSharedPtr = is_specialization_of_v<T, std::shared_ptr>;
    // TODO(orf): Move this ↑↑

    template<typename T>
    concept BuiltinType = std::is_same_v<T, Unit> ||
                          std::is_same_v<T, Int> ||
                          std::is_same_v<T, Char> ||
                          std::is_same_v<T, Bool> ||
                          std::is_same_v<T, Float>;
    template<typename T>
    concept BuiltinTypePtr = IsSharedPtr<T> &&
                             BuiltinType<typename T::element_type>;
    template<typename T>
    concept ComplexType = std::is_same_v<T, Array> ||
                          std::is_same_v<T, Ref> ||
                          std::is_same_v<T, Function> ||
                          std::is_same_v<T, Custom> ||
                          std::is_same_v<T, Unknown>;
    template<typename T>
    concept ComplexTypePtr = IsSharedPtr<T> &&
                             ComplexType<typename T::element_type>;
    template<typename T>
    concept AnyType = BuiltinType<T> || ComplexType<T>;
    template<typename T>
    concept AnyTypePtr = BuiltinTypePtr<T> || ComplexTypePtr<T>;

    class Type {
    private:
        std::variant<
            std::shared_ptr<Unit>,
            std::shared_ptr<Int>,
            std::shared_ptr<Char>,
            std::shared_ptr<Bool>,
            std::shared_ptr<Float>,
            std::shared_ptr<Array>,
            std::shared_ptr<Ref>,
            std::shared_ptr<Function>,
            std::shared_ptr<Custom>,
            std::shared_ptr<Unknown>
        > type_variant;
        template<class... Ts> struct overloaded : Ts... { using Ts::operator()...; };
    protected:
    public:
        template<AnyType T>
        bool is() const {
            return std::holds_alternative<std::shared_ptr<T>>(type_variant);
        }
        template<AnyType T>
        std::shared_ptr<T> as(std::string_view caller = "") const {
            if (auto inner = std::get_if<std::shared_ptr<T>>(type_variant)) {
                return *inner;
            }
            error::internal(
                "Tried to downcast " +
                std::string(type_enum_str()) +
                " to " + std::string(type_enum_to_str(T::tEnum)) +
                caller != "" ? " in " + caller : "";
            );
        }
        bool operator==(Type const& other) const;
        bool operator!=(Type const& other) const;

        std::string get_type_enum_str() const;
        std::string to_string() const;
    };
}

#endif // __TYPESYSTEM_CORE_HPP__