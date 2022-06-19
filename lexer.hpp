#ifndef __LEXER_HPP__
#define __LEXER_HPP__

#include <string_view>

#include "ast/forward.hpp"

int yylex();
void yyerror(ast::core::Program &the_program, std::string_view msg);
extern int yylineno;

#endif
