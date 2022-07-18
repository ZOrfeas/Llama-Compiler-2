%require "3.8.2"
%language "C++"
%define api.value.type variant
%define api.token.constructor
/* This has a runtime overhead with RTTI but provides
      some more guarantees */
/* %define parser.assert */ 
%define parse.error detailed /* or 'verbose' */
%define parse.lac full
%define api.token.raw /* performance improved. cannot use plain chars in lexer */
/* %locations */

%code requires {
// #include <cstdio>
// #include <cstdlib>
#include <iostream>
#include <string>
#include <vector>
#include <memory>
#include <stdexcept>
#include <string_view>

#include "../ast/ast.hpp"
#include "../error/error.hpp"

// #define YYDEBUG 1 // comment out to disable debug feature compilation
class ParseDriver;
}
%expect 24
%param { ParseDriver& drv }

%code {
#include "driver.hpp"
}

%define api.token.prefix {TOK_}
%token 
    STOP  0  "end of file"
    AND "and"
    ARRAY "array"
    BEGIN "begin"
    BOOL "bool"
    CHAR "char"
    DELETE "delete"
    DIM "dim"
    DO "do"
    DONE "done"
    DOWNTO "downto"
    ELSE "else"
    END "end"
    FALSE "false"
    FLOAT "float"
    FOR "for"
    IF "if"
    IN "in"
    INT "int"
    LET "let"
    MATCH "match"
    MOD "mod"
    MUTABLE "mutable"
    NEW "new"
    NOT "not"
    OF "of"
    REC "rec"
    REF "ref"
    THEN "then"
    TO "to"
    TRUE "true"
    TYPE "type"
    UNIT "unit"
    WHILE "while"
    WITH "with"
;

%token<std::string> idlower 
%token<std::string> idupper 

%token<std::string> intconst 
%token<std::string> floatconst 
%token<std::string> charconst 
%token<std::string> stringliteral

%token 
    DASHGREATER "->"
    PLUSDOT "+."
    MINUSDOT "-."
    STARDOT "*."
    SLASHDOT "/."
    DBLSTAR "**"
    DBLAMPERSAND "&&"
    DBLBAR "||"
    LTGT "<>"
    LEQ "<="
    GEQ ">="
    DBLEQ "=="
    EXCLAMEQ "!="
    COLONEQ ":="
    SEMICOLON ";"
    EQ "="
    GT ">"
    LT "<"
    PLUS "+"
    MINUS "-"
    STAR "*"
    SLASH "/"
    COLON ":"
    COMMA ","
    LBRACKET "["
    RBRACKET "]"
    LPAREN "("
    RPAREN ")"
    BAR "|"
    EXCLAM "!"
;

/**
 * Associativity and precedence information 
 */

// Type definition necessary precedences
%right "->"
%precedence "ref"
%precedence ARRAYOF

// Operator precedences
%precedence LETIN
%left ";"
%right "then" "else"
%nonassoc ":="
%left "||"
%left "&&"
%nonassoc "=" "<>" ">" "<" "<=" ">=" "==" "!=" COMPOP
%left "+" "-" "+." "-." ADDOP
%left "*" "/" "*." "/." "mod" MULTOP
%right "**"
%precedence UNOPS

%nterm program

%nterm definition_choice 
%nterm letdef
%nterm typedef
%nterm program_list

%nterm def
%nterm and_def_opt_list
%nterm tdef
%nterm and_tdef_opt_list

%nterm par
%nterm par_list

%nterm type
%nterm of_type_opt_list at_least_one_type

%nterm constr
%nterm bar_constr_opt_list

%nterm bracket_star_opt comma_star_opt_list

%nterm expr expr_2
%nterm bracket_comma_expr_list comma_expr_opt_list expr_2_list

%nterm unop comp_operator add_operator mult_operator

%nterm clause
%nterm bar_clause_opt_list
%nterm pattern
%nterm pattern_list

/* %printer { yyo << $$; } <*>; */

%%

program 
    : program_list                          {  }
;

program_list
    : %empty                                { }
    | program_list definition_choice        { }
;

definition_choice               
    : letdef                                { }
    | typedef                               { }
;

letdef
    : "let" def and_def_opt_list            { }
    | "let" "rec" def and_def_opt_list      { }
;

typedef
    : "type" tdef and_tdef_opt_list         { }
;

def
    : idlower "=" expr                                    { }
    | idlower ":" type "=" expr                           { }
    | idlower par_list "=" expr                           { }
    | idlower par_list ":" type "=" expr                  { }
    | "mutable" idlower bracket_comma_expr_list           { }
    | "mutable" idlower bracket_comma_expr_list ":" type  { }
    | "mutable" idlower                                   { }
    | "mutable" idlower ":" type                          { }
;

par_list
    : par                                           { }
    | par_list par                                  { }
;

bracket_comma_expr_list
    : "[" expr comma_expr_opt_list "]"              { }
;

comma_expr_opt_list
    : %empty                                        { }
    | comma_expr_opt_list "," expr                  { }
;

tdef
    : idlower "=" constr bar_constr_opt_list      { }
;

bar_constr_opt_list
    : %empty                                        { }
    | bar_constr_opt_list "|" constr                { }
;

and_def_opt_list
    : %empty                                        { }
    | and_def_opt_list "and" def                    { }
;

and_tdef_opt_list
    : %empty                                        { }
    | and_tdef_opt_list "and" tdef                  { }
;

constr
    : idupper of_type_opt_list                { }
;

of_type_opt_list
    : %empty                                    { }
    | "of" at_least_one_type                    { }
;

at_least_one_type
    : type                                      { }
    | at_least_one_type type                    { }
;

par
    : idlower                     { }
    | "(" idlower ":" type ")"    { }
;

type
    : "unit"                                            { }
    | "int"                                             { }
    | "char"                                            { }
    | "bool"                                            { }
    | "float"                                           { }
    | "(" type ")"                                      { }
    | type "->" type                                    { }
    | type "ref"                                        { }
    | "array" bracket_star_opt "of" type %prec ARRAYOF  { }
    | idlower                                           { }
;

bracket_star_opt
    : %empty                                    { }
    | "[" "*" comma_star_opt_list "]"           { }
;

comma_star_opt_list
    : %empty                                    { }
    | comma_star_opt_list "," "*"               { }
;

expr
    : letdef "in" expr %prec LETIN                              { }
    | expr ";" expr                                             { }
    | "if" expr "then" expr "else" expr                         { }
    | "if" expr "then" expr                                     { }
    | expr ":=" expr                                            { }
    | expr "||" expr                                            { }
    | expr "&&" expr                                            { }
    | expr comp_operator expr %prec COMPOP                      { }
    | expr add_operator expr %prec ADDOP                        { }
    | expr mult_operator expr %prec MULTOP                      { }
    | expr "**" expr                                            { }
    | unop expr %prec UNOPS                                     { }
    | "while" expr "do" expr "done"                             { }
    | "for" idlower "=" expr "to" expr "do" expr "done"         { }
    | "for" idlower "=" expr "downto" expr "do" expr "done"     { }
    | "match" expr "with" clause bar_clause_opt_list "end"      { }
    | "dim" intconst idlower                                    { }
    | "dim" idlower                                             { }
    | idlower expr_2_list                                       { }
    | idupper expr_2_list                                       { }
    | expr_2                                                    { }
;

expr_2
    : intconst                              { }
    | floatconst                            { }
    | charconst                             { }
    | stringliteral                         { }
    | idlower                               { }
    | idupper                               { }
    | "true"                                { }
    | "false"                               { }
    | "(" ")"                               { }
    | "!" expr_2                            { }
    | idlower bracket_comma_expr_list       { }
    | "new" type                            { }
    | "(" expr ")"                          { }
    | "begin" expr "end"                    { }
;

expr_2_list
    : expr_2                { }
    | expr_2_list expr_2    { }
;

unop
    : "+"       { }
    | "-"       { }
    | "+."      { }
    | "-."      { }
    | "not"     { }
    | "delete"  { }
;

comp_operator
    : "="       { }
    | "<>"      { }
    | ">"       { }
    | "<"       { }
    | "<="      { }
    | ">="      { }
    | "=="      { }
    | "!="      { }
;

add_operator
    : "+"       { }
    | "-"       { }
    | "+."      { }
    | "-."      { }
;

mult_operator
    : "*"       { }
    | "/"       { }
    | "*."      { }
    | "/."      { }
    | "mod"     { }
;

bar_clause_opt_list
    : %empty                            { }
    | bar_clause_opt_list "|" clause    { }
;

clause
    : pattern "->" expr         { }
;

pattern
    : "+" intconst              { }
    | "-" intconst              { }
    | intconst                  { }
    | "+." floatconst           { }
    | "-." floatconst           { }
    | floatconst                { }
    | charconst                 { }
    | "true"                    { }
    | "false"                   { }
    | idupper                   { }
    | idlower                   { }
    | "(" pattern ")"           { }
    |  idupper pattern_list     { }
;

pattern_list
    : pattern                   { }
    | pattern_list pattern      { }
;


%%
namespace yy {
void parser::error (const std::string & msg) {
    drv.error(msg);
}
}
