#ifndef __AST_EXPR_HPP__
#define __AST_EXPR_HPP__

#include <string>
#include <vector>
#include <memory>

#include "../visitor/visitor.hpp"
#include "./core.hpp"
#include "./expr.hpp"
#include "./stmt.hpp"

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
        LetIn(LetStmt *def, Expression *expr): def(def), expr(expr) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class Literal : public Expression {
    protected:
        Literal(string original_val = ""): original_val(original_val) {}
    public:
        string original_val;
    };
    namespace literal {
        class Unit : public Literal {
        public:
            Unit(): Literal("()") {}
            void accept(visit::Visitor *v) override { v->visit(this); }
        };
        class Int : public Literal {
        public:
            int val;
            Int(string original_val)
                : Literal(original_val), val(std::stoi(original_val)) {}
            void accept(visit::Visitor *v) override { v->visit(this); }
        };
        class Char : public Literal {
        public:
            static char extract_char(string const& s) {
                if (s[0] != '\\') return s[0];
                else if (s[1] == 'x') {
                    const char hex[2] = {s[2], s[3]};
                    return static_cast<char>(std::stol(hex, nullptr, 16));
                } else {
                    switch (s[1]) {
                        case 'n': return '\n';
                        case 't': return '\t';
                        case 'r': return '\r';
                        case '0': return static_cast<char>(0);
                        case '\\': // fallthrough
                        case '\'': // fallthrough
                        case '\"': return s[1];
                        default:
                            throw std::runtime_error("Invalid character literal");
                    }
                }
            }
            char val;
            Char(string original_val)
                : Literal(original_val), val(extract_char(original_val)) {}
            void accept(visit::Visitor *v) override { v->visit(this); }
        };
        class Bool : public Literal {
        public:
            bool val;
            Bool(bool val): val(val) {}
            void accept(visit::Visitor *v) override { v->visit(this); }
        };
        class Float : public Literal {
        public:
            float val;
            Float(string original_val)
                : Literal(original_val), val(std::stof(original_val)) {}
            void accept(visit::Visitor *v) override { v->visit(this); }
        };
        class String : public Literal {
        public:
            static std::string extract_string(std::string const& s) {
                std::string res;
                for (size_t i = 0; i < s.size(); i++) {
                    if (s[i] != '\\') res.push_back(s[i]);
                    else if (s[i+1] == 'x') {
                        res.push_back(Char::extract_char(s.substr(i, 4)));
                        i += 3;
                    } else {
                        res.push_back(Char::extract_char(s.substr(i, 2)));
                        i++;
                    }
                }
                return res;
            }
            string val;
            String(string original_val)
                : Literal(original_val), val(extract_string(original_val)) {}
            void accept(visit::Visitor *v) override { v->visit(this); }
        };
    } // namespace literal
    namespace op {
        class Binary : public Expression {
        public:
            unique_ptr<Expression> lhs, rhs;
            int op;
            Binary(Expression *lhs, int op, Expression *rhs)
                : lhs(lhs), rhs(rhs), op(op) {}
            void accept(visit::Visitor *v) override { v->visit(this); }
        };
        class Unary : public Expression {
        public:
            unique_ptr<Expression> expr;
            int op;
            Unary(int op, Expression *expr)
                : expr(expr), op(op) {}
            void accept(visit::Visitor *v) override { v->visit(this); }
        };
        using core::TypeAnnotation;
        class New : public Expression {
        public:
            unique_ptr<TypeAnnotation> t;
            New(TypeAnnotation *t): t(t) {}
            void accept(visit::Visitor *v) override { v->visit(this); }
        };
    } // namespace op
    class While : public Expression {
    public:
        unique_ptr<Expression> cond, body;
        While(Expression *cond, Expression *body)
            : cond(cond), body(body) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
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
        ): id(id), init(init), end(end), body(body), ascending(ascending) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class If : public Expression {
    public:
        unique_ptr<Expression> cond, then_expr, else_expr;
        If(Expression *cond, Expression *then_expr, Expression *else_expr)
            : cond(cond), then_expr(then_expr), else_expr(else_expr) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class Dim : public Expression {
    public:
        unique_ptr<literal::Int> dim;
        string id;
        Dim(literal::Int *dim, string id): dim(dim), id(id) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class IdCall : public Expression {
    public:
        string id;
        IdCall(string id): id(id) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class FuncCall : public Expression  {
    public:
        string id;
        unique_ptr<vector<unique_ptr<Expression>>> arg_list;
        FuncCall(string id, vector<unique_ptr<Expression>> *arg_list)
            : id(id), arg_list(arg_list) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class ConstrCall : public Expression {
    public:
        string id;
        unique_ptr<vector<unique_ptr<Expression>>> arg_list;
        ConstrCall(string id, vector<unique_ptr<Expression>> *arg_list = nullptr)
            : id(id), arg_list(arg_list) {}
        void accept(visit::Visitor *v) override { v->visit(this); }
    };
    class ArrayAccess : public IdCall {
    public:
        unique_ptr<vector<unique_ptr<Expression>>> index_list;
        ArrayAccess(string id, vector<unique_ptr<Expression>> *index_list)
            : IdCall(id), index_list(index_list) {}
        void accept(visit::Visitor *v) override { v->visit(this); }  
    };
    using namespace utils::match;
    class Match : public Expression {
    public:
        unique_ptr<Expression> to_match;
        unique_ptr<vector<unique_ptr<Clause>>> clause_list;
        Match(Expression *to_match, vector<unique_ptr<Clause>> *clause_list)
            : to_match(to_match), clause_list(clause_list) {}
        void accept(visit::Visitor *v) override { v->visit(this); };
    };
}

#endif // __AST_EXPR_HPP__