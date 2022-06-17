
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

    int Type::get_dimensions() const {
        return this->safe_as<Array>("get_dimensions")->dimensions;
    }
    void Type::set_dimensions(int new_dims) {
        static constexpr auto this_func_name = "set_dimensions";
        auto arr = this->safe_as<Array>(this_func_name);
        arr->dimensions = new_dims;
    }
    int Type::get_low_bound() const {
        static constexpr auto this_func_name = "get_low_bound";
        return *(this->safe_as<Array>(this_func_name)->dim_low_bound_ptr);
    }
    void Type::set_low_bound(int new_low_bound) {
        static constexpr auto this_func_name = "set_low_bound";
        auto arr = this->safe_as<Array>(this_func_name);
        *(arr->dim_low_bound_ptr) = new_low_bound;
    }
    void Type::copy_low_bound_ptr_from(std::shared_ptr<Type> other) {
        static constexpr auto this_func_name = "copy_low_bound_ptr_from";
        auto arr = this->safe_as<Array>(this_func_name);
        arr->dim_low_bound_ptr = other->safe_as<Array>(this_func_name)->dim_low_bound_ptr;
    }
    void Type::set_element_type(std::shared_ptr<Type> new_elem_type) {
        static constexpr auto this_func_name = "set_element_type";
        auto arr = this->safe_as<Array>(this_func_name);
        arr->element_type = std::move(new_elem_type);
    }
    std::shared_ptr<Type> Type::get_element_type() const {
        static constexpr auto this_func_name = "get_element_type";
        return this->safe_as<Array>(this_func_name)->element_type;
    }

    // Ref //

    void Type::set_ref_type(std::shared_ptr<Type> new_ref_type) {
        static constexpr auto this_func_name = "set_ref_type";
        auto ref = this->safe_as<Ref>(this_func_name);
        ref->ref_type = std::move(new_ref_type);
    }
    std::shared_ptr<Type> Type::get_ref_type() const {
        static constexpr auto this_func_name = "get_ref_type";
        return this->safe_as<Ref>(this_func_name)->ref_type;
    }

    // Function //

    void Type::add_param(std::shared_ptr<Type> param_type) {
        static constexpr auto this_func_name = "add_param";
        auto func = this->safe_as<Function>(this_func_name);
        func->param_types.push_back(std::move(param_type));
    }
    void Type::set_param_type(int idx, std::shared_ptr<Type> new_param_type) {
        static constexpr auto this_func_name = "set_param_type";
        auto func = this->safe_as<Function>(this_func_name);
        func->param_types[idx] = std::move(new_param_type);
    }
    std::vector<std::shared_ptr<Type>> const& Type::get_param_types() const {
        static constexpr auto this_func_name = "get_param_types";
        return this->safe_as<Function>(this_func_name)->param_types;
    }
    void Type::set_return_type(std::shared_ptr<Type> new_return_type) {
        static constexpr auto this_func_name = "set_return_type";
        auto func = this->safe_as<Function>(this_func_name);
        func->return_type = std::move(new_return_type);
    }
    std::shared_ptr<Type> Type::get_return_type() const {
        static constexpr auto this_func_name = "get_return_type";
        return this->safe_as<Function>(this_func_name)->return_type;
    }

    // Constructor //

    void Type::set_custom_type(std::shared_ptr<Custom> owner_type) {
        static constexpr auto this_func_name = "set_custom_type";
        auto ctor = this->safe_as<Constructor>(this_func_name);
        ctor->custom_type = std::move(owner_type);
    }
    void Type::add_field(std::shared_ptr<Type> field_type) {
        static constexpr auto this_func_name = "add_field";
        auto ctor = this->safe_as<Constructor>(this_func_name);
        ctor->field_types.push_back(std::move(field_type));
    }
    std::vector<std::shared_ptr<Type>> const& Type::get_field_types() const {
        static constexpr auto this_func_name = "get_field_types";
        return this->safe_as<Constructor>(this_func_name)->field_types;
    }

    // Custom //

    void Type::add_constructor(std::shared_ptr<Constructor> constructor_type) {
        static constexpr auto this_func_name = "add_constructor";
        auto custom = this->safe_as<Custom>(this_func_name);
        custom->constructor_types.push_back(std::move(constructor_type));
    }
    std::vector<std::shared_ptr<Constructor>> const& Type::get_constructor_types() const {
        static constexpr auto this_func_name = "get_constructor_types";
        return this->safe_as<Custom>(this_func_name)->constructor_types;
    }

    // Unknown //
    //!Note possibly obsolete
    
    std::string Type::get_unknown_id() const {
        static constexpr auto this_func_name = "get_unknown_id";
        return this->to_string();
    }
}