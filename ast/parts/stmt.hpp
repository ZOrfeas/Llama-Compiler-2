#ifndef __AST_STMT_HPP__
#define __AST_STMT_HPP__

#include <string>
#include <vector>
#include <memory>

#include "../visitor/visitor.hpp"
#include "./core.hpp"
#include "./def.hpp"

namespace ast::stmt {
    using namespace def;
    using std::string;
    using std::vector;
    using std::unique_ptr;
    
    class LetStmt : public core::DefStmt {
    public:
        unique_ptr<vector<unique_ptr<Def>>> def_list;
        bool is_recursive;
        LetStmt(vector<unique_ptr<Def>> *def_list, bool is_recursive)
            : def_list(def_list), is_recursive(is_recursive) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class TypeStmt : public core::DefStmt {
    public:
        unique_ptr<vector<unique_ptr<TypeDef>>> def_list;
        TypeStmt(vector<unique_ptr<TypeDef>> *def_list)
            : def_list(def_list) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
} // namespace stmt


#endif // __AST_STMT_HPP__