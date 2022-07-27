#ifndef AST_STMT_HPP
#define AST_STMT_HPP

#include <memory>
#include <vector>

#include "core.hpp"
#include "../forward.hpp"
#include "../../utils/utils.hpp"

namespace ast::stmts {

    struct LetStmt : public NodeCommons<LetStmt> {
        LetStmt(
            std::vector<std::unique_ptr<defs::LetDef>> definitions,
            bool is_recursive
        ) : definitions(std::move(definitions)), is_recursive(is_recursive) {}
        std::vector<std::unique_ptr<defs::LetDef>> definitions;
        bool is_recursive;
    };
    struct TypeStmt : public NodeCommons<TypeStmt> {
        TypeStmt(
            std::vector<std::unique_ptr<defs::TypeDef>> definitions
        ) : definitions(std::move(definitions)) {}
        std::vector<std::unique_ptr<defs::TypeDef>> definitions;
    };

    struct DefStmt : public utils::Variant<
        LetStmt, TypeStmt
    >, public utils::enable_make_variant<DefStmt>  {
        using type::type;
    };

} // namespace stmt


#endif // AST_STMT_HPP