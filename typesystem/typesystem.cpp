#include <type_traits>

#include "./typesystem.hpp"

namespace typesys {    
    // Builtins //

    Builtin::Builtin(TypeEnum b): Type(b) {}
    bool Builtin::equals(Type const* o) const {
        return this->type == o->type;
    }
    std::string Builtin::to_string() const {
        return type_name(this->type);
    }
    
    Unit::Unit(): Builtin(TypeEnum::UNIT) {}
    Int::Int(): Builtin(TypeEnum::INT) {}
    Char::Char(): Builtin(TypeEnum::CHAR) {}
    Bool::Bool(): Builtin(TypeEnum::BOOL) {}
    Float::Float(): Builtin(TypeEnum::FLOAT) {}

    // Array //

    Array::Array(std::shared_ptr<Type> element_type, int dimensions):
        Type(TypeEnum::ARRAY),
        element_type(std::move(element_type)),
        dimensions(dimensions) {}
    bool Array::equals(Type const* o) const {
        if (Array* o = o->as<Array>()) {
            return *this->dim_low_bound_ptr == 0 &&
                   *o->dim_low_bound_ptr == 0 &&
                   this->dimensions == o->dimensions &&
                   this->element_type->equals(o->element_type);
        }
        return false;
    }
    std::string Array::to_string() const {
        const auto dim_low_bound = *this->dim_low_bound_ptr;
        const auto dim_string = [&](){
            if (dim_low_bound != 0) {
                return "array of (at least" + 
                        std::to_string(dim_low_bound) + 
                        ") of";
            } else {
                std::string dim_string = "";
                if (this->dimensions > 1) {

                }
                return "array " + dim_string + " of";
            }
        }();
        return "(" + dim_string + " " +
                this->element_type->to_string() + ")";        
    }

    // Ref //

    Ref::Ref(std::shared_ptr<Type> ref_type):
        Type(TypeEnum::REF),
        ref_type(std::move(ref_type)) {}
    bool Ref::equals(Type const* o) const {
        if (Ref* o = o->as<Ref>())
            return this->ref_type->equals(o->ref_type);
        return false;
    }
    std::string Ref::to_string() const {
        return this->ref_type->to_string() + " ref";
    }
    // Function //
    
    Function::Function(std::shared_ptr<Type> return_type):
        Type(TypeEnum::FUNCTION),
        return_type(std::move(return_type)) {}
    bool Function::equals(Type const* o) const {
        if (Function* o = o->as<Function>()) {
            if (this->param_types.size() != o->param_types.size())
                return false;
            for (size_t i = 0; i < this->param_types.size(); i++)
                if (!(this->param_types[i]->equals(o->param_types[i])))
                    return false;
            return this->return_type->equals(o->return_type);
        }
        return false;
    }
    std::string Function::to_string() const {
        auto param_string = [&]() -> std::string {
            if (this->param_types.size() == 0)
                return "unknown";
            std::string tmp_string = this->param_types[0]->to_string();
            for (size_t i = 1; i < this->param_types.size(); i++)
                tmp_string += " -> " + this->param_types[i]->to_string();
            return tmp_string;
        }();
        return "(" + param_string + " -> " +
                this->return_type->to_string() + ")";
    }
    
    // Constructor //

    Constructor::Constructor(std::string_view name):
        Type(TypeEnum::RECORD),
        name(name) {}
    bool Constructor::equals(Type const* o) const {
        if (Constructor* o = o->as<Constructor>())
            return this->custom_type->Type::equals(o->custom_type) &&
                   this->name == o->name;
        return false;
    }
    std::string Constructor::to_string() const {
        return this->name;
    }

    // Custom //

    Custom::Custom(std::string_view name):
        Type(TypeEnum::CUSTOM),
        name(name) {}
    bool Custom::equals(Type const* o) const {
        if (Custom* o = o->as<Custom>())
            //!Note(orf): make sure no shadowing is allowed
            return this->name == o->name;
        return false;
    }
    std::string Custom::to_string() const {
        return this->name;
    }

    // Unknown //

    unsigned long Unknown::next_id = 0;
    Unknown::Unknown(): Type(TypeEnum::UNKNOWN), id(next_id++) {}
    bool Unknown::equals(Type const* o) const {
        if (Unknown* o = o->as<Unknown>())
            return this->id == o->id;
    }
    std::string Unknown::to_string() const {
        return "@" + std::to_string(this->id);
    }
}