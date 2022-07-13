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

namespace lexer {
    std::string_view get_current_file();
    void push_source_file(std::string_view);
}
#endif
