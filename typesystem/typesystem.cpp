#include <type_traits>

#include "./types.hpp"

namespace typesys {
    Type::Type() {}
    // possibly nice syntax for matching variants:
    // https://github.com/AVasK/vx/blob/main/vx.hpp

    std::string Type::to_string() const {
        return std::visit(overloaded{
            []<BuiltinTypePtr T>(T const& t) -> std::string {
                return type_enum_to_str(T::element_type::type_enum);
            },
            []<ComplexTypePtr T>(T const& t) {
                return t->to_string();
            },
        }, type_variant);
    }
    const char* Type::get_type_enum_str() const {
        return std::visit(overloaded{
            []<AnyTypePtr T>(T const& t) {
                return type_enum_to_str(T::element_type::type_enum);
            }
        }, type_variant);
    }
    bool Type::operator==(Type const& other) const {
        return std::visit(overloaded{
            []<AnyTypePtr T>(T const& t1, T const& t2) {
                // They are the safe || their contents are equal
                return t1 == t2 || *t1 == *t2;
            },
            [](auto&& t1, auto&& t2) { return false; }
        }, type_variant, other.type_variant);
    }
    bool Type::operator!=(Type const& other) const {
        return !(*this == other);
    }

    // Array //

    Array::Array(Type element_type, int dimensions):
        element_type(std::move(element_type)),
        dimensions(dimensions) {}
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
                this->element_type.to_string() + ")";        
    }
    bool Array::operator==(Array const& other) const {
        return *this->dim_low_bound_ptr == 0 &&
               *other.dim_low_bound_ptr == 0 &&
               this->dimensions == other.dimensions &&
               this->element_type == other.element_type;
    }
    bool Array::operator!=(Array const& other) const {
        return !(*this == other);
    }

    // Ref //

    Ref::Ref(Type ref_type):
        ref_type(std::move(ref_type)) {}
    std::string Ref::to_string() const {
        return this->ref_type.to_string() + " ref";
    }
    bool Ref::operator==(Ref const& other) const {
        return this->ref_type == other.ref_type;
    }
    bool Ref::operator!=(Ref const& other) const {
        return !(*this == other);
    }
    // Function //
    
    Function::Function(Type return_type):
        return_type(std::move(return_type)) {}
    std::string Function::to_string() const {
        auto param_string = [&]() -> std::string {
            if (this->param_types.size() == 0)
                return "unknown";
            std::string tmp_string = this->param_types[0].to_string();
            for (size_t i = 1; i < this->param_types.size(); i++)
                tmp_string += " -> " + this->param_types[i].to_string();
            return tmp_string;
        }();
        return "(" + param_string + " -> " +
                this->return_type.to_string() + ")";
    }
    bool Function::operator==(Function const& other) const {
        return this->param_types.size() == other.param_types.size() &&
               this->return_type == other.return_type &&
               std::equal(
                     this->param_types.begin(),
                     this->param_types.end(),
                     other.param_types.begin()
                );
    }
    bool Function::operator!=(Function const& other) const {
        return !(*this == other);
    }
    
    // Custom //

    Custom::Custom(std::string_view name):
        name(name) {}
    //!Note(orf): make sure no shadowing is allowed
    std::string Custom::to_string() const {
        return this->name;
    }
    bool Custom::operator==(Custom const& other) const {
        return this->name == other.name;
    }
    bool Custom::operator!=(Custom const& other) const {
        return !(*this == other);
    }
    Custom::Constructor::Constructor(
        std::string_view name,
        Custom const& custom_type
    ): name(name), custom_type(custom_type) {}


    // Unknown //

    unsigned long Unknown::next_id = 0;
    Unknown::Unknown(): id(next_id++) {}
    std::string Unknown::to_string() const {
        return "@" + std::to_string(this->id);
    }
    bool Unknown::operator==(Unknown const& other) const {
        return this->id == other.id;
    }
    bool Unknown::operator!=(Unknown const& other) const {
        return !(*this == other);
    }
}