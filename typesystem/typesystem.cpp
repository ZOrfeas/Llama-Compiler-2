#include <type_traits>

#include "./typesystem.hpp"

namespace typesys {
    Type::Type(TypeEnum t): type(t) {}
    bool Type::is_same(Type const* o) const {
        return this == o;
    }
    bool Type::equals(std::shared_ptr<Type> o) const {
        if (this->is_same(o.get())) return true;
        return this->equals(o.get());
    }
    
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

    Array::Array(int dimensions, std::shared_ptr<Type> element_type):
        Type(TypeEnum::ARRAY),
        dimensions(dimensions),
        element_type(std::move(element_type)) {}
    bool Array::equals(Type const* o) const {
        if (Array* o = o->as<Array>()) {
            return this->dimensions == o->dimensions &&
                   this->element_type->equals(o->element_type);
        }
        return false;
    }
    std::string Array::to_string() const {
        // TODO: implement
    }

    // Ref //

    Ref::Ref(std::shared_ptr<Type> element_type):
        Type(TypeEnum::REF),
        element_type(std::move(element_type)) {}
    bool Ref::equals(Type const* o) const {
        if (Ref* o = o->as<Ref>()) {
            return this->element_type->equals(o->element_type);
        }
        return false;
    }
    std::string Ref::to_string() const {
        // TODO: implement
    }
    // Function //
    
    Function::Function(std::shared_ptr<Type> return_type):
        Type(TypeEnum::FUNCTION),
        return_type(std::move(return_type)) {}
    void Function::add_param(std::shared_ptr<Type> param_type) {
        param_types.push_back(std::move(param_type));
    }
    bool Function::equals(Type const* o) const {
        if (Function* o = o->as<Function>()) {
            if (this->param_types.size() != o->param_types.size()) {
                return false;
            }
            for (size_t i = 0; i < this->param_types.size(); i++) {
                if (!(this->param_types[i]->equals(o->param_types[i]))) {
                    return false;
                }
            }
            return this->return_type->equals(o->return_type);
        }
        return false;
    }
    std::string Function::to_string() const {
        // TODO: implement
    }
    
    // Constructor //

    Constructor::Constructor(std::string_view name):
        Type(TypeEnum::RECORD),
        name(name) {}
    bool Constructor::equals(Type const* o) const {
        if (Constructor* o = o->as<Constructor>()) {
            return this->custom_type->Type::equals(o->custom_type) &&
                   this->name == o->name;
        }
        return false;
    }
    std::string Constructor::to_string() const {
        // TODO: implement
    }
    void Constructor::set_custom_type(std::shared_ptr<Custom> owner) {
        custom_type = std::move(owner);
    }
    void Constructor::add_field(std::shared_ptr<Type> field) {
        field_types.push_back(std::move(field));
    }

    // Custom //

    Custom::Custom(std::string_view name):
        Type(TypeEnum::CUSTOM),
        name(name) {}
    bool Custom::equals(Type const* o) const {
        if (Custom* o = o->as<Custom>()) {
            //!Note(orf): make sure no shadowing is allowed
            return this->name == o->name;
        }
        return false;
    }
    std::string Custom::to_string() const {
        // TODO: implement
    }

    // Unknown //

    unsigned long Unknown::next_id = 0;
    Unknown::Unknown(): Type(TypeEnum::UNKNOWN), id(next_id++) {}
    bool Unknown::equals(Type const* o) const {
        if (Unknown* o = o->as<Unknown>()) {
            return this->id == o->id;
        }
    }
    std::string Unknown::to_string() const {
        // TODO: implement
    }
}