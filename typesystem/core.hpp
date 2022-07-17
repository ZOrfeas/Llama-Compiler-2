#ifndef __TYPESYSTEM_CORE_HPP__
#define __TYPESYSTEM_CORE_HPP__

#include <memory>
#include <ostream>
#include <string>
#include <string_view>
#include <variant>
#include <concepts>

#include "../utils/utils.hpp"
#include "../error/error.hpp"
#include "./forward.hpp"

//!Note: I do not like to have to implement templates in headers ... :(

namespace typesys {
    enum class TypeEnum {
        UNIT, INT, CHAR, BOOL, FLOAT,
        ARRAY, REF, FUNCTION, CUSTOM, CONSTRUCTOR, UNKNOWN
    };
    inline const char* type_enum_to_str(TypeEnum b) {
        static const char* builtin_name[] = {
            "unit", "int", "char", "bool", "float",
            "array", "ref", "function", "custom", "constructor", "unknown"
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
    concept BuiltinTypePtr = utils::IsSharedPtr<T> &&
                             BuiltinType<typename T::element_type>;
    template<typename T>
    concept ComplexType = std::is_same_v<T, Array> ||
                          std::is_same_v<T, Ref> ||
                          std::is_same_v<T, Function> ||
                          std::is_same_v<T, Custom> ||
                          std::is_same_v<T, Constructor> ||
                          std::is_same_v<T, Unknown>;
    template<typename T>
    concept ComplexTypePtr = utils::IsSharedPtr<T> &&
                             ComplexType<typename T::element_type>;
    template<typename T>
    concept AnyType = BuiltinType<T> || ComplexType<T>;
    template<typename T>
    concept AnyTypePtr = BuiltinTypePtr<T> || ComplexTypePtr<T>;
    
    using namespace std::string_literals;
    class Type : public utils::SharedPtrVariant<
        Unit, Int, Char, Bool, Float, Array, Ref,
        Function, Custom, Constructor, Unknown
    > {
    protected:
        using Base = utils::SharedPtrVariant<
            Unit, Int, Char, Bool, Float, Array, Ref,
            Function, Custom, Constructor, Unknown
        >;
        using Base::Base;
    public:
        template<BuiltinType T>
        static Type get() {
            static auto instance = std::make_shared<T>();
            return Type(instance);
        }
        //!Note: Make sure the errors when giving wrong args are not too bad
        template<ComplexType T, typename... Args>
        static Type get(Args&&... args) {
            return Type(std::make_shared<T>(
                std::forward<Args>(args)...
            ));
        }
        template<AnyType T>
        std::shared_ptr<T> safe_as(std::string_view caller = "") const {
            const auto msg = "Tried to downcast "s +
                get_type_enum_str() +
                " to " + 
                type_enum_to_str(T::tEnum) +
                caller != "" ? " in " + std::string(caller) :   "";
            return Base::safe_as<T>(msg);
        }
        bool operator==(Type const& other) const;
        bool operator!=(Type const& other) const;

        const char* get_type_enum_str() const;
        friend std::ostream& operator<<(std::ostream&, Type const&);
        std::string to_string() const;
    };
}

#endif // __TYPESYSTEM_CORE_HPP__
