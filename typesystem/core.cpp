
#include "./typesystem.hpp"

namespace typesys {
    Type::Type() {}
    // possibly nice syntax for matching variants:
    // https://github.com/AVasK/vx/blob/main/vx.hpp

    std::string Type::to_string() const {
        return std::visit(overloaded{
            []<BuiltinTypePtr T>(T const& t) -> std::string {
                return type_enum_to_str(T::type_enum);
            },
            []<ComplexTypePtr T>(T const& t) {
                return t->to_string();
            },
        }, type_variant);
    }
    std::string Type::get_type_enum_str() const {
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
}