#ifndef __AST_UTILSMATCH_HPP__
#define __AST_UTILSMATCH_HPP__

#include <string>
#include <vector>
#include <memory>

#include "../visitor/visitor.hpp"
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
        PatLiteral(expr::Literal *literal): literal(literal) {}
        void accept(visit::Visitor &v) override { v.visit(*this); }
    };
    class PatId : public Pattern {
    public:
        string id;
        PatId(string id): id(id) {}
        void accept(visit::Visitor &v) override { v.visit(*this); }
    };
    class PatConstr : public Pattern {
    public:
        string id;
        unique_ptr<vector<unique_ptr<Pattern>>> pattern_list;
        PatConstr(string id, vector<unique_ptr<Pattern>> *pattern_list)
            : id(id), pattern_list(pattern_list) {}
        void accept(visit::Visitor &v) override { v.visit(*this); }
    };
    using core::Expression;
    class Clause : public core::Node {
    public:
        unique_ptr<Pattern> pattern;
        unique_ptr<Expression> expr;
        Clause(Pattern *pattern, Expression *expr)
            : pattern(pattern), expr(expr) {}
        void accept(visit::Visitor &v) override { v.visit(*this); }
    };
}

#endif // __AST_UTILSMATCH_HPP__