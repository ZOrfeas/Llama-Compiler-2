#ifndef __LEXER_HPP__
#define __LEXER_HPP__

#include <string>
#include <vector>
#include <unordered_set>
#include <string_view>

#include "ast/forward.hpp"

int yylex();
void yyerror(ast::core::Program&, std::string_view);
extern int yylineno;

class IncludeStack {
private:
    std::unordered_set<std::string> filename_set;
    std::vector<std::string> filename_stack;
public:
    IncludeStack() = default;
    bool is_empty() const;
    bool has(std::string_view) const;
    void push(std::string_view);
    bool pop();
    std::string_view top() const;
};
extern IncludeStack include_stack;

#endif
