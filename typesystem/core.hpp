#ifndef __TYPESYSTEM_CORE_HPP__
#define __TYPESYSTEM_CORE_HPP__

#include <string_view>
#include <concepts>

#include "../../error/error.hpp"
#include "./forward.hpp"

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
        bool is_same(Type const* o) const;
        virtual bool equals(Type const* o) const = 0;
    public:
        TypeEnum const type;
        virtual ~Type() = default;
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
        bool equals(std::shared_ptr<Type> o) const;
        virtual std::string to_string() const = 0;

        // Utility functions for subclasses (manual downcasting check)       
        // Array //

        int get_dimensions() const;
        void set_dimensions(int);
        int get_low_bound() const;
        void set_low_bound(int);
        void copy_low_bound_ptr_from(std::shared_ptr<Type>);
        void set_element_type(std::shared_ptr<Type>);
        std::shared_ptr<Type> get_element_type() const;

        // Ref //

        void set_ref_type(std::shared_ptr<Type>);
        std::shared_ptr<Type> get_ref_type() const;

        // Function //

        void add_param(std::shared_ptr<Type>);
        void set_param_type(int, std::shared_ptr<Type>);
        std::vector<std::shared_ptr<Type>> const& get_param_types() const;
        void set_return_type(std::shared_ptr<Type>);
        std::shared_ptr<Type> get_return_type() const;

        // Constructor //

        void set_custom_type(std::shared_ptr<Custom>);
        void add_field(std::shared_ptr<Type>);
        std::vector<std::shared_ptr<Type>> const& get_field_types() const;

        // Custom //

        void add_constructor(std::shared_ptr<Constructor>);
        std::vector<std::shared_ptr<Constructor>> const& get_constructor_types() const;

        // Unknown //

        std::string get_unknown_id() const;
    };

}

#endif // __TYPESYSTEM_CORE_HPP__