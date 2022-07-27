#include <type_traits>

#include "./typesystem.hpp"

namespace typesys {
    auto Type::to_string() const -> std::string {
        using utils::match;
        return std::visit(match{
            []<BuiltinType T>(T const& t) -> std::string {
                return type_enum_to_str(T::type_enum);
            },
            []<ComplexType T>(T const& t) {
                return t.to_string();
            }
        }, *this);
    }
    auto operator<<(std::ostream& os, Type const& t) -> std::ostream& {
        return os << t.to_string();
    }
    auto Type::get_type_enum_str() const -> const char* {
        using utils::match;
        return std::visit(match{
            []<AnyType T>(T const& t) {
                return type_enum_to_str(T::type_enum);
            }
        }, *this);
    }
    auto Type::operator==(Type const& other) const -> bool {
        using std::make_tuple;
        using utils::match;
        return std::visit(match{
            []<BuiltinType T>(T const&, T const&) {
                return true;
            },
            []<ComplexType T>(T const& t1, T const& t2) {
                // They are the same || their contents are equal
                return t1 == t2 || &t1 == &t2;
            },
            [](auto& t1, auto& t2) { return false; }
        }, *this, other);
    }
    auto Type::operator!=(Type const& other) const -> bool {
        return !(*this == other);
    }

    // Array //

    Array::Array(std::shared_ptr<Type> element_type, int dimensions):
        element_type(std::move(element_type)),
        dimensions(dimensions) {}
    auto Array::to_string() const -> std::string {
        const auto dim_low_bound = *this->dim_low_bound_ptr;
        const auto dim_string = [&](){
            if (dim_low_bound != 0) {
                return "array of (at least" + 
                        std::to_string(dim_low_bound) + 
                        ") of";
            } else {
                std::string dim_string = " ";
                if (this->dimensions > 1) {
                    dim_string += "[";
                    for (int i = 0; i < this->dimensions-1; ++i) {
                        dim_string += "*,";
                    }
                    dim_string += "*]";
                }
                return "array" + dim_string + " of";
            }
        }();
        return "(" + dim_string + " " +
                this->element_type->to_string() + ")";        
    }
    auto Array::operator==(Array const& other) const -> bool {
        return *this->dim_low_bound_ptr == 0 &&
               *other.dim_low_bound_ptr == 0 &&
               this->dimensions == other.dimensions &&
               this->element_type == other.element_type;
    }
    auto Array::operator!=(Array const& other) const -> bool {
        return !(*this == other);
    }

    // Ref //

    Ref::Ref(std::shared_ptr<Type> ref_type):
        ref_type(std::move(ref_type)) {}
    auto Ref::to_string() const -> std::string {
        return this->ref_type->to_string() + " ref";
    }
    auto Ref::operator==(Ref const& other) const -> bool {
        return this->ref_type == other.ref_type;
    }
    auto Ref::operator!=(Ref const& other) const -> bool {
        return !(*this == other);
    }
    // Function //
    
    Function::Function(std::shared_ptr<Type> return_type):
        return_type(std::move(return_type)) {}
    auto Function::to_string() const -> std::string {
        auto param_string = [&]() -> std::string {
            if (this->param_types.size() == 0)
                return "unknown";
            std::string tmp_string = this->param_types[0].to_string();
            for (size_t i = 1; i < this->param_types.size(); i++)
                tmp_string += " -> " + this->param_types[i].to_string();
            return tmp_string;
        }();
        return "(" + param_string + " -> " +
                this->return_type->to_string() + ")";
    }
    auto Function::operator==(Function const& other) const -> bool {
        return this->param_types.size() == other.param_types.size() &&
               this->return_type == other.return_type &&
               std::equal(
                     this->param_types.begin(),
                     this->param_types.end(),
                     other.param_types.begin()
                );
    }
    auto Function::operator!=(Function const& other) const -> bool {
        return !(*this == other);
    }
    // Custom //

    Custom::Custom(std::string_view name):
        name(name) {}
    std::string Custom::to_string() const {
        return this->name;
    }
    //!Note(orf): make sure no shadowing is allowed
    auto Custom::operator==(Custom const& other) const -> bool {
        return this->name == other.name;
    }
    auto Custom::operator!=(Custom const& other) const -> bool {
        return !(*this == other);
    }
    // Constructor //
    
    Constructor::Constructor(
        std::string_view name,
        Custom const& custom_type
    ): name(name), custom_type(custom_type) {}
    std::string Constructor::to_string() const {
        return this->name + "(" + this->custom_type.to_string() + ")";
    }
    auto Constructor::operator==(Constructor const& other) const -> bool {
        return this->name == other.name;
    }
    auto Constructor::operator!=(Constructor const& other) const -> bool {
        return !(*this == other);
    }
    // Unknown //

    unsigned long Unknown::next_id = 0;
    Unknown::Unknown(): id(next_id++) {}
    std::string Unknown::to_string() const {
        return "@" + std::to_string(this->id);
    }
    auto Unknown::operator==(Unknown const& other) const -> bool {
        return this->id == other.id;
    }
    auto Unknown::operator!=(Unknown const& other) const -> bool {
        return !(*this == other);
    }
}