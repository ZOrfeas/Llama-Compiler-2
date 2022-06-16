#ifndef __TYPESYSTEM_CORE_HPP__
#define __TYPESYSTEM_CORE_HPP__

#include <string_view>
#include <concepts>

#include "../../error/error.hpp"

//!Note: I do not like to have to implement templates in headers ... :(

namespace typesys {
    enum class TypeEnum {
        UNIT, INT, CHAR, BOOL, FLOAT,
        ARRAY, REF, FUNCTION, RECORD, CUSTOM, UNKNOWN
    };
    inline const char* type_name(TypeEnum b) {
        static const char* builtin_name[] = {
            "unit", "int", "char", "bool", "float",
            "array", "ref", "function", "record", "custom", "unknown"
        };
        return builtin_name[static_cast<int>(b)];
    }
    
    template<typename T>
    concept IsInstantiableType = requires(T t) {
        { T::tEnum } -> std::convertible_to<TypeEnum>;
    };
    class Type {
    protected:
        Type(TypeEnum t);
        template<IsInstantiableType T>
        bool is() const {
            return this->type == T::tEnum;
        }
        template<IsInstantiableType T>
        T* as() const {
            if (this->is<T>()) {
                return static_cast<T*>(t);
            }
            return nullptr;
        }
        template<IsInstantiableType T>
        T* safe_as(std::string_view caller = "") const {
            if (T* t = this->as<T>()) {
                return t;
            }
            error::internal(
                "Tried to downcast " +
                std::string(type_name(t->type)) +
                " to " + std::string(type_name(T::tEnum)) +
                caller != "" ? " in " + caller : "";
            );
        }
        bool is_same(Type const* o) const;
        virtual bool equals(Type const* o) const = 0;
    public:
        TypeEnum const type;
        virtual ~Type() = default;
        
        bool equals(std::shared_ptr<Type> o) const;
        virtual std::string to_string() const = 0;

        // Utility functions for subclasses without virtual
        // TODO: implement them for Type, no need to dispatch
        // TODO:    to each subclass after static casting

    };

}

#endif // __TYPESYSTEM_CORE_HPP__