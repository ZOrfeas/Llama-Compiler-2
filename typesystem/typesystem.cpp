#include <type_traits>

#include "./typesystem.hpp"

namespace typesys {    
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
                this->element_type->to_string() + ")";        
    }

    // Ref //

    Ref::Ref(std::shared_ptr<Type> ref_type):
        ref_type(std::move(ref_type)) {}
    std::string Ref::to_string() const {
        return this->ref_type->to_string() + " ref";
    }
    // Function //
    
    Function::Function(std::shared_ptr<Type> return_type):
        return_type(std::move(return_type)) {}
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
        name(name) {}
    std::string Constructor::to_string() const {
        return this->name;
    }

    // Custom //

    Custom::Custom(std::string_view name):
        name(name) {}
    //!Note(orf): make sure no shadowing is allowed
    std::string Custom::to_string() const {
        return this->name;
    }

    // Unknown //

    unsigned long Unknown::next_id = 0;
    Unknown::Unknown(): id(next_id++) {}
    std::string Unknown::to_string() const {
        return "@" + std::to_string(this->id);
    }
}