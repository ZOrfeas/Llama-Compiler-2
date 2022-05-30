#ifndef __AST_EXPR_HPP__
#define __AST_EXPR_HPP__

#include <string>
#include <vector>
#include <memory>

#include "core.hpp"
#include "expr.hpp"
#include "stmt.hpp"

namespace ast::utils::match {
    class Clause;
}
namespace ast::expr {
    using core::Expression;
    using stmt::LetStmt;
    using std::string;
    using std::vector;
    using std::unique_ptr;

    class LetIn : public Expression {
    public:
        unique_ptr<LetStmt> def;
        unique_ptr<Expression> expr;
        LetIn(LetStmt *def, Expression *expr): def(def), expr(expr) {};
    };
    class Literal : public Expression {
    protected:
        Literal(string original_val = ""): original_val(original_val) {};
    public:
        string original_val;
    };
    namespace literal {
        class Unit : public Literal {
        public:
            Unit(): Literal("()") {};
        };
        class Int : public Literal {
        public:
            int val;
            Int(string original_val)
                : Literal(original_val), val(std::stoi(original_val)) {};
        };
        class Char : public Literal {
        public:
            char val;
            Char(string original_val)
                : Literal(original_val), val(val) {
                    // TODO: Transfer logic
                };
        };
        class Bool : public Literal {
        public:
            bool val;
            Bool(bool val): val(val) {};
        };
        class Float : public Literal {
        public:
            float val;
            Float(string original_val)
                : Literal(original_val), val(std::stof(original_val)) {};
        };
        class String : public Literal {
        public:
            string val;
            String(string original_val)
                : Literal(original_val), val(val) {
                    // TODO: Transfer logic
                };
        };
    } // namespace literal
    namespace op {
        class Binary : public Expression {
        public:
            unique_ptr<Expression> lhs, rhs;
            int op;
            Binary(Expression *lhs, int op, Expression *rhs)
                : lhs(lhs), rhs(rhs), op(op) {};
        };
        class Unary : public Expression {
        public:
            unique_ptr<Expression> expr;
            int op;
            Unary(int op, Expression *expr)
                : expr(expr), op(op) {};
        };
        using core::TypeAnnotation;
        class New : public Expression {
        public:
            unique_ptr<TypeAnnotation> t;
            New(TypeAnnotation *t): t(t) {};
        };
    } // namespace op
    class While : public Expression {
    public:
        unique_ptr<Expression> cond, body;
        While(Expression *cond, Expression *body)
            : cond(cond), body(body) {};
    };
    class For : public Expression {
    public:
        string id;
        unique_ptr<Expression> init, end, body;
        bool ascending;
        For(
            string id,
            Expression *init,
            bool ascending,
            Expression *end,
            Expression *body
        ): id(id), init(init), end(end), body(body), ascending(ascending) {};
    };
    class If : public Expression {
    public:
        unique_ptr<Expression> cond, then_expr, else_expr;
        If(Expression *cond, Expression *then_expr, Expression *else_expr)
            : cond(cond), then_expr(then_expr), else_expr(else_expr) {};
    };
    class Dim : public Expression {
    public:
        unique_ptr<literal::Int> dim;
        string id;
        Dim(literal::Int *dim, string id): dim(dim), id(id) {};
    };
    class IdentifierCall : public Expression {
    public:
        string id;
        IdentifierCall(string id): id(id) {};
    };
    class FuncConstrCall : public IdentifierCall {
    public:
        unique_ptr<vector<unique_ptr<Expression>>> arg_list;
        FuncConstrCall(string id, vector<unique_ptr<Expression>> *arg_list)
            : IdentifierCall(id), arg_list(arg_list) {};
    };
    class ArrayAccess : public IdentifierCall {
    public:
        unique_ptr<vector<unique_ptr<Expression>>> index_list;
        ArrayAccess(string id, vector<unique_ptr<Expression>> *index_list)
            : IdentifierCall(id), index_list(index_list) {};
    };
    using namespace utils::match;
    class Match : public Expression {
    public:
        unique_ptr<Expression> to_match;
        unique_ptr<vector<unique_ptr<Clause>>> clause_list;
        Match(Expression *to_match, vector<unique_ptr<Clause>> *clause_list)
            : to_match(to_match), clause_list(clause_list) {};
    };
}

#endif // __AST_EXPR_HPP__