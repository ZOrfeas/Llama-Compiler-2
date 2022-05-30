#ifndef __LEXER_HPP__
#define __LEXER_HPP__

#include <string_view>

int yylex();
void yyerror(std::string_view msg);
extern int yylineno;

#endif
