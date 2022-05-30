#ifndef __AST_CORE_HPP__
#define __AST_CORE_HPP__

#include <vector>
#include <memory>

namespace ast::core {
    using std::vector;
    using std::unique_ptr;
    class Node {
    protected: virtual ~Node() = default;
    };
    class DefStmt : public Node {
    protected: DefStmt() = default;
    };
    class Program : public Node {
    public:
        unique_ptr<vector<unique_ptr<DefStmt>>> defstmt_list;
        Program(vector<unique_ptr<DefStmt>> *statements)
            : defstmt_list(statements) {}
    };

    class TypeAnnotation : public Node {
    protected: TypeAnnotation() = default;
    };
    class Expression : public Node {
    protected: Expression() = default;
    };
}


#endif // __AST_CORE_HPP__