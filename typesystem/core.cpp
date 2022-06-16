
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

    // Array //

    int Type::get_dimensions() const {}
    void Type::set_dimensions(int new_dims) {}
    int Type::get_low_bound() const {}
    void Type::set_low_bound(int new_low_bound) {}
    void Type::copy_low_bound_ptr_from(std::shared_ptr<Type> other) {}
    void Type::set_element_type(std::shared_ptr<Type> new_elem_type) {}
    std::shared_ptr<Type> Type::get_element_type() const {}

    // Ref //

    void Type::set_ref_type(std::shared_ptr<Type> new_ref_type) {}
    std::shared_ptr<Type> Type::get_ref_type() const {}

    // Function //

    void Type::add_param(std::shared_ptr<Type> param_type) {}
    void Type::set_param_type(int idx, std::shared_ptr<Type> new_param_type) {}
    std::vector<std::shared_ptr<Type>> const& Type::get_param_types() const {}
    void Type::set_return_type(std::shared_ptr<Type> new_return_type) {}
    std::shared_ptr<Type> Type::get_return_type() const {}

    // Constructor //

    void Type::set_custom_type(std::shared_ptr<Custom> owner_type) {}
    void Type::add_field(std::shared_ptr<Type> field_type) {}
    std::vector<std::shared_ptr<Type>> const& Type::get_field_types() const {}

    // Custom //

    void Type::add_constructor(std::shared_ptr<Constructor> constructor_type) {}
    std::vector<std::shared_ptr<Constructor>> const& Type::get_constructor_types() const {}

    // Unknown //

    std::string Type::get_unknown_id() const {}
}