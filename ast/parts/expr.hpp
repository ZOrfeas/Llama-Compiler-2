#ifndef AST_EXPR_HPP
#define AST_EXPR_HPP

#include <cstdlib>
#include <string>
#include <type_traits>
#include <vector>
#include <memory>

#include "core.hpp"
#include "../forward.hpp"
#include "../../utils/utils.hpp"


namespace ast::exprs {

    struct LetIn : public NodeCommons<LetIn> {
        LetIn(
            std::unique_ptr<stmts::LetStmt> let_stmt,
            std::unique_ptr<Expression> expr)
        : let_stmt(std::move(let_stmt)), expr(std::move(expr)) {}
        std::unique_ptr<stmts::LetStmt> let_stmt;
        std::unique_ptr<Expression> expr;
    };
    namespace literals {
        template<typename T>
        struct LiteralCommons : public NodeCommons<T> { 
            LiteralCommons(std::string original)
                : original(std::move(original)) {}
            std::string original; 
        };
        struct Unit : public NodeCommons<Unit> {};
        struct Int : public LiteralCommons<Int> {
            Int(std::string original)
                : LiteralCommons(std::move(original)), val(std::atoi(original.c_str())) {}
            int val;
        };
        struct Char : public LiteralCommons<Char> {
            Char(std::string original)
                : LiteralCommons(std::move(original)), val(extract_char(original)) {}
            char val;
            static auto extract_char(std::string_view s) -> char {
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
                            log::crash("Invalid character literal");
                    }
                }
                log::crash("extract_char reached impossible state");
                return '\0';
            }
        };
        struct Bool : public NodeCommons<Bool> { 
            Bool(bool val): val(val) {}
            bool val;
        };
        struct Float : public LiteralCommons<Float> { 
            Float(std::string original)
                : LiteralCommons(std::move(original)), val(std::stof(original)) {}
            float val;
        };
        struct String : public LiteralCommons<String> {
            String(std::string original)
                : LiteralCommons(std::move(original)), val(extract_string(original)) {}
            std::string val;
            static auto extract_string(std::string s) -> std::string {
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
        };
    }
    struct Literal : public utils::Variant<
        literals::Unit, literals::Int, literals::Char,
        literals::Bool, literals::Float, literals::String
    > {};
    enum class Op {
        // Binops and/or Unops
        StructEq, StructNe, NatEq, NatNe,
        Gt, Lt, Ge, Le, 
        Plus, Minus, PlusFlt, MinusFlt,
        Mult, Div, MultFlt, DivFlt, Mod,
        Pow, Semicolon, Assign, Or, And,

        
        // Unops
        Not, Delete, Deref, 
    };
    struct UnaryOp : public NodeCommons<UnaryOp> {
        UnaryOp(Op op, std::unique_ptr<Expression> expr)
            : op(op), expr(std::move(expr)) {}
        Op op;
        std::unique_ptr<Expression> expr;
    };
    struct BinaryOp : public NodeCommons<BinaryOp> {
        BinaryOp(
            Op op, std::unique_ptr<Expression> lhs,
            std::unique_ptr<Expression> rhs
        ) : op(op), lhs(std::move(lhs)), rhs(std::move(rhs)) {}
        Op op;
        std::unique_ptr<Expression> lhs, rhs;
    };
    struct NewOp : public NodeCommons<NewOp> {
        NewOp(std::unique_ptr<annotation::TypeAnnotation> type)
            : type(std::move(type)) {}
        std::unique_ptr<annotation::TypeAnnotation> type;
    };
    struct While : public NodeCommons<While> {
        While(std::unique_ptr<Expression> cond, std::unique_ptr<Expression> body)
            : cond(std::move(cond)), body(std::move(body)) {}
        std::unique_ptr<Expression> cond, body;
    };
    struct For : public NodeCommons<For> {
        For(
            std::string id, std::unique_ptr<Expression> init,
            std::unique_ptr<Expression> end, std::unique_ptr<Expression> body,
            bool is_ascending
        ) : id(std::move(id)), init(std::move(init)), end(std::move(end)),
            body(std::move(body)), is_ascending(is_ascending) {}
        std::string id;
        std::unique_ptr<Expression> init, end, body;
        bool is_ascending;
    };
    struct If : public NodeCommons<If> {
        If(
            std::unique_ptr<Expression> cond,
            std::unique_ptr<Expression> then_branch,
            std::unique_ptr<Expression> else_branch = {}
        ) : cond(std::move(cond)), then_branch(std::move(then_branch)),
                else_branch(std::move(else_branch)) {}
        std::unique_ptr<Expression> cond, then_branch, else_branch;
    };
    struct Dim : public NodeCommons<Dim> {
        Dim(std::string id, literals::Int dim = literals::Int("1"))
            : id(std::move(id)), dim(std::move(dim)) {}
        std::string id;
        literals::Int dim;
    };
    struct IdCall : public NodeCommons<IdCall> {
        IdCall(std::string id): id(std::move(id)) {}
        std::string id;
    };
    struct FuncCall : public NodeCommons<FuncCall> {
        FuncCall(std::string id, std::vector<std::unique_ptr<Expression>> args)
            : id(std::move(id)), args(std::move(args)) {}
        std::string id;
        std::vector<std::unique_ptr<Expression>> args;
    };
    struct ConstrCall : public NodeCommons<ConstrCall> {
        ConstrCall(
            std::string id,
            std::vector<std::unique_ptr<Expression>> args = {}
        ) : id(std::move(id)), args(std::move(args)) {}
        std::string id;
        std::vector<std::unique_ptr<Expression>> args;
    };
    struct ArrayAccess : public NodeCommons<ArrayAccess> {
        ArrayAccess(std::string id, std::vector<std::unique_ptr<Expression>> indexes)
            : id(std::move(id)), indexes(std::move(indexes)) {}
        std::string id;
        std::vector<std::unique_ptr<Expression>> indexes;
    };
    namespace match {
        struct IdPattern : public NodeCommons<IdPattern> {
            IdPattern(std::string id): id(std::move(id)) {}
            std::string id;
        };
        struct ConstrPattern : public NodeCommons<ConstrPattern> {
            ConstrPattern(std::string id, std::vector<std::unique_ptr<Pattern>> args = {})
                : id(std::move(id)), args(std::move(args)) {}
            std::string id;
            std::vector<std::unique_ptr<Pattern>> args;
        };
        struct Clause : public NodeCommons<Clause> {
            Clause(std::unique_ptr<Pattern> pattern, std::unique_ptr<Expression> expr)
                : pattern(std::move(pattern)), expr(std::move(expr)) {}
            std::unique_ptr<Pattern> pattern;
            std::unique_ptr<Expression> expr;
        };
        struct Pattern : public utils::concat_variants_t<
                Literal, utils::Variant<IdPattern, ConstrPattern>
        >, public utils::enable_make_variant<Pattern>{
            using type::type;
        };
    }
    struct Match : public NodeCommons<Match> {
        Match(
            std::unique_ptr<Expression> to_match,
            std::vector<std::unique_ptr<match::Clause>> clauses
        ) : to_match(std::move(to_match)), clauses(std::move(clauses)) {}
        std::unique_ptr<Expression> to_match;
        std::vector<std::unique_ptr<match::Clause>> clauses;
    };
    struct Expression : public utils::concat_variants_t<
        utils::Variant<
            LetIn, UnaryOp, BinaryOp, NewOp, While, For, If, Dim, IdCall,
            FuncCall, ConstrCall, ArrayAccess, Match
        >, Literal
    >, public utils::enable_make_variant<Expression> {
        using type::type;
    };
}

#endif // AST_EXPR_HPP