#ifndef __AST_UTILSMATCH_HPP__
#define __AST_UTILSMATCH_HPP__

#include <string>
#include <vector>
#include <memory>

#include "./core.hpp"
#include "./expr.hpp"

namespace ast::utils::match {
    using core::Expression;
    using std::string;
    using std::vector;
    using std::unique_ptr;
    class Pattern : public core::Node {
    protected: Pattern() = default;
    };
    class PatLiteral : public Pattern {
    public:
        unique_ptr<expr::Literal> literal;
        PatLiteral(expr::Literal *literal): literal(literal) {};
    };
    class PatIdentifier : public Pattern {
    public:
        string id;
        PatIdentifier(string id)
            : id(id) {};
    };
    class PatConstructor : public Pattern {
    public:
        string id;
        unique_ptr<vector<unique_ptr<Pattern>>> pattern_list;
        PatConstructor(string id, vector<unique_ptr<Pattern>> *pattern_list)
            : id(id), pattern_list(pattern_list) {};
    };
    using core::Expression;
    class Clause : public core::Node {
    public:
        unique_ptr<Pattern> pattern;
        unique_ptr<Expression> expr;
        Clause(Pattern *pattern, Expression *expr)
            : pattern(pattern), expr(expr) {};
    };
}

#endif // __AST_UTILSMATCH_HPP__