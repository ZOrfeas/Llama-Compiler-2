#ifndef AST_CORE_HPP
#define AST_CORE_HPP

#include <memory>
#include <vector>

#include "../forward.hpp"

namespace ast {

    //!Note: This will be a base class for all nodes in the AST.
    //!Note:   We can use it to store a `location` struct for each node.
    template<typename T>
    struct NodeCommons {
        // template<typename... Args>
        // static auto make(Args&&... args) -> std::unique_ptr<T> {
        //     // used allow make_unique on bracket-initalized structs
        //     return std::unique_ptr<T>(
        //         new T{std::forward<Args>(args)...}
        //     );
        // }
    };

    struct Program : public NodeCommons<Program> {
        Program(
            std::vector<std::unique_ptr<stmts::DefStmt>> def_stmts = {}
        ) : def_stmts(std::move(def_stmts)) {}
        std::vector<std::unique_ptr<stmts::DefStmt>> def_stmts;
    };
}


#endif // AST_CORE_HPP