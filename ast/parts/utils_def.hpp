#ifndef __AST_UTILSDEF_HPP__
#define __AST_UTILSDEF_HPP__

#include <vector>
#include <memory>

#include "./core.hpp"

namespace ast::utils::def {
    using std::vector;
    using std::unique_ptr;
    using std::string;
    using core::TypeAnnotation;
    class Constructor : public core::Node {
    public:
        string id;
        unique_ptr<vector<unique_ptr<TypeAnnotation>>> type_list;
        Constructor(string id, vector<unique_ptr<TypeAnnotation>> *type_list):
            id(id), type_list(type_list) {};
    };
    class Param : public core::Node {
    public:
        string id;
        unique_ptr<TypeAnnotation> type;
        Param(string id, TypeAnnotation *type = nullptr):
            id(id), type(type) {};
    };
}

#endif // __AST_UTILSDEF_HPP__