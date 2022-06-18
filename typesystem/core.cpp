
#include "./typesystem.hpp"

namespace typesys {
    Type::Type() {}

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
    std::string Type::type_enum_str() const {
        return std::visit(overloaded{
            []<AnyTypePtr T>(T const& t) {
                return type_enum_to_str(T::element_type::type_enum);
            }
        }, type_variant);
    }

}