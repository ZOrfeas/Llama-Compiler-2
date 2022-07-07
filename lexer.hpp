#ifndef __LEXER_HPP__
#define __LEXER_HPP__

#include <vector>
#include <unordered_set>
#include <string_view>

#include "ast/forward.hpp"

int yylex();
void yyerror(ast::core::Program &the_program, std::string_view msg);
extern int yylineno;
extern std::unordered_set<std::string> filename_set;
extern std::vector<std::string> filename_stack;
#endif
