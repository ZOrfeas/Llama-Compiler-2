%code requires {
// #include <cstdio>
// #include <cstdlib>
#include <iostream>
#include <string>
#include <vector>
#include <memory>
#include <string_view>

#include "lexer.hpp"
#include "ast/ast.hpp"

#include "passes/print/ast-print.hpp"

// using std::std::vector;
// using std::std::unique_ptr;
// #define YYDEBUG 1 // comment out to disable debug feature compilation
}
%define parse.error verbose
%expect 24
%parse-param { ast::core::Program *&the_program }

%union {
    ast::core::Program *program;
    std::vector<std::unique_ptr<ast::core::DefStmt>> *def_stmt_list;

    ast::core::DefStmt *def_stmt;
    ast::stmt::LetStmt *let_stmt;
    ast::stmt::TypeStmt *type_stmt;

    ast::def::Def *def;
    std::vector<std::unique_ptr<ast::def::Def>> *def_list;
    ast::def::TypeDef *type_def;
    std::vector<std::unique_ptr<ast::def::TypeDef>> *type_def_list;

    ast::utils::def::Param *param;
    std::vector<std::unique_ptr<ast::utils::def::Param>> *param_list;

    ast::core::Expression *expr;
    std::vector<std::unique_ptr<ast::core::Expression>> *expr_list;

    ast::utils::def::Constructor *constr;
    std::vector<std::unique_ptr<ast::utils::def::Constructor>> *constr_list;

    ast::core::TypeAnnotation *type_annotation;
    std::vector<std::unique_ptr<ast::core::TypeAnnotation>> *type_annotation_list;

    ast::utils::match::Clause *clause;
    std::vector<std::unique_ptr<ast::utils::match::Clause>> *clause_list;
    ast::utils::match::Pattern *pattern;
    std::vector<std::unique_ptr<ast::utils::match::Pattern>> *pattern_list;

    std::string *id;

    int op;
    int num;
    float flt;
    std::string *str;
}

%token T_and "and"
%token T_array "array"
%token T_begin "begin"
%token T_bool "bool"
%token T_char "char"
%token<op> T_delete "delete"
%token T_dim "dim"
%token T_do "do"
%token T_done "done"
%token T_downto "downto"
%token T_else "else"
%token T_end "end"
%token T_false "false"
%token T_float "float"
%token T_for "for"
%token T_if "if"
%token T_in "in"
%token T_int "int"
%token T_let "let"
%token T_match "match"
%token T_mod "mod"
%token T_mutable "mutable"
%token T_new "new"
%token<op> T_not "not"
%token T_of "of"
%token T_rec "rec"
%token T_ref "ref"
%token T_then "then"
%token T_to "to"
%token T_true "true"
%token T_type "type"
%token T_unit "unit"
%token T_while "while"
%token T_with "with"

%token<id> T_idlower 
%token<id> T_idupper 

%token<str> T_intconst 
%token<str> T_floatconst 
%token<str> T_charconst 
%token<str> T_stringliteral

%token T_dashgreater "->"
%token T_plusdot "+."
%token T_minusdot "-."
%token T_stardot "*."
%token T_slashdot "/."
%token T_dblstar "**"
%token T_dblampersand "&&"
%token T_dblbar "||"
%token T_lessgreater "<>"
%token T_leq "<="
%token T_geq ">="
%token T_dbleq "=="
%token T_exclameq "!="
%token T_coloneq ":="

/**
 * Associativity and precedence information 
 */

// Type definition necessary precedences
%right "->"
%precedence "ref"
%precedence ARRAYOF

// Operator precedences
%precedence LETIN
%left<op> ';'
%right "then" "else"
%nonassoc<op> ":="
%left<op> "||"
%left<op> "&&"
%nonassoc<op> '=' "<>" '>' '<' "<=" ">=" "==" "!=" COMPOP
%left<op> '+' '-' "+." "-." ADDOP
%left<op> '*' '/' "*." "/." "mod" MULTOP
%right<op> "**"
%precedence UNOPS

%type<program> program

%type<def_stmt> definition_choice 
%type<let_stmt> letdef
%type<type_stmt> typedef
%type<def_stmt_list> program_list

%type<def> def
%type<def_list> and_def_opt_list
%type<type_def> tdef
%type<type_def_list> and_tdef_opt_list

%type<param> par
%type<param_list> par_list

%type<type_annotation> type
%type<type_annotation_list> of_type_opt_list at_least_one_type

%type<constr> constr
%type<constr_list> bar_constr_opt_list

%type<num> bracket_star_opt comma_star_opt_list

%type<expr> expr expr_2
%type<expr_list> bracket_comma_expr_list comma_expr_opt_list expr_2_list

%type<op> unop comp_operator add_operator mult_operator

%type<clause> clause
%type<clause_list> bar_clause_opt_list
%type<pattern> pattern
%type<pattern_list> pattern_list
%%

program 
    : program_list                          { the_program = new ast::core::Program($1); /* the_program = $$;*/ /* auto v = PrintVisitor(); $$->accept(v);*/ }
;

program_list
    : %empty                                { $$ = new std::vector<std::unique_ptr<ast::core::DefStmt>>(); }
    | program_list definition_choice        { $1->push_back(std::unique_ptr<ast::core::DefStmt>($2)); $$ = $1; }
;

definition_choice               
    : letdef                                { $$ = $1; }
    | typedef                               { $$ = $1; }
;

letdef
    : "let" def and_def_opt_list            { $3->insert($3->begin(), std::unique_ptr<ast::def::Def>($2)); $$ = new ast::stmt::LetStmt($3, false); }
    | "let" "rec" def and_def_opt_list      { $4->insert($4->begin(), std::unique_ptr<ast::def::Def>($3)); $$ = new ast::stmt::LetStmt($4, true);  }
;

typedef
    : "type" tdef and_tdef_opt_list         { $3->insert($3->begin(), std::unique_ptr<ast::def::TypeDef>($2)); $$ = new ast::stmt::TypeStmt($3); }
;

def
    : T_idlower '=' expr                                    { $$ = new ast::def::Constant(*$1, $3); delete $1;         }
    | T_idlower ':' type '=' expr                           { $$ = new ast::def::Constant(*$1, $5, $3); delete $1;     }
    | T_idlower par_list '=' expr                           { $$ = new ast::def::Function(*$1, $2, $4); delete $1;     }
    | T_idlower par_list ':' type '=' expr                  { $$ = new ast::def::Function(*$1, $2, $6, $4); delete $1; }
    | "mutable" T_idlower bracket_comma_expr_list           { $$ = new ast::def::Array(*$2, $3); delete $2;            }
    | "mutable" T_idlower bracket_comma_expr_list ':' type  { $$ = new ast::def::Array(*$2, $3, $5); delete $2;        }
    | "mutable" T_idlower                                   { $$ = new ast::def::Variable(*$2); delete $2;             }
    | "mutable" T_idlower ':' type                          { $$ = new ast::def::Variable(*$2, $4); delete $2;         }
;

par_list
    : par                                           { $$ = new std::vector<std::unique_ptr<ast::utils::def::Param>>(); $$->push_back(std::unique_ptr<ast::utils::def::Param>($1)); }
    | par_list par                                  { $1->push_back(std::unique_ptr<ast::utils::def::Param>($2)); $$ = $1; }
;

bracket_comma_expr_list
    : '[' expr comma_expr_opt_list ']'              { $3->insert($3->begin(), std::unique_ptr<ast::core::Expression>($2)); $$ = $3; }
;

comma_expr_opt_list
    : %empty                                        { $$ = new std::vector<std::unique_ptr<ast::core::Expression>>(); }
    | comma_expr_opt_list ',' expr                  { $1->push_back(std::unique_ptr<ast::core::Expression>($3)); $$ = $1; }
;

tdef
    : T_idlower '=' constr bar_constr_opt_list      { $4->insert($4->begin(), std::unique_ptr<ast::utils::def::Constructor>($3)); 
                                                      $$ = new ast::def::TypeDef(*$1, $4);
                                                      delete $1; }
;

bar_constr_opt_list
    : %empty                                        { $$ = new std::vector<std::unique_ptr<ast::utils::def::Constructor>>(); }
    | bar_constr_opt_list '|' constr                { $1->push_back(std::unique_ptr<ast::utils::def::Constructor>($3)); $$ = $1; }
;

and_def_opt_list
    : %empty                                        { $$ = new std::vector<std::unique_ptr<ast::def::Def>>(); }
    | and_def_opt_list "and" def                    { $1->push_back(std::unique_ptr<ast::def::Def>($3)); $$ = $1; }
;

and_tdef_opt_list
    : %empty                                        { $$ = new std::vector<std::unique_ptr<ast::def::TypeDef>>(); }
    | and_tdef_opt_list "and" tdef                  { $1->push_back(std::unique_ptr<ast::def::TypeDef>($3)); $$ = $1; }
;

constr
    : T_idupper of_type_opt_list                { $$ = new ast::utils::def::Constructor(*$1, $2); delete $1; }
;

of_type_opt_list
    : %empty                                    { $$ = new std::vector<std::unique_ptr<ast::core::TypeAnnotation>>(); }
    | "of" at_least_one_type                    { $$ = $2; }
;

at_least_one_type
    : type                                      { $$ = new std::vector<std::unique_ptr<ast::core::TypeAnnotation>>(); $$->push_back(std::unique_ptr<ast::core::TypeAnnotation>($1)); }
    | at_least_one_type type                    { $1->push_back(std::unique_ptr<ast::core::TypeAnnotation>($2)); $$ = $1; }
;

par
    : T_idlower                     { $$ = new ast::utils::def::Param(*$1); delete $1; }
    | '(' T_idlower ':' type ')'    { $$ = new ast::utils::def::Param(*$2, $4); delete $2; }
;

type
    : "unit"                                            { $$ = new ast::annotation::BasicType(types::Builtin::UNIT);  }
    | "int"                                             { $$ = new ast::annotation::BasicType(types::Builtin::INT);   }
    | "char"                                            { $$ = new ast::annotation::BasicType(types::Builtin::CHAR);  }
    | "bool"                                            { $$ = new ast::annotation::BasicType(types::Builtin::BOOL);  }
    | "float"                                           { $$ = new ast::annotation::BasicType(types::Builtin::FLOAT); }
    | '(' type ')'                                      { $$ = $2;                                           }
    | type "->" type                                    { $$ = new ast::annotation::FunctionType($1, $3);    }
    | type "ref"                                        { $$ = new ast::annotation::RefType($1);             }
    | "array" bracket_star_opt "of" type %prec ARRAYOF  { $$ = new ast::annotation::ArrayType($2, $4);       }
    | T_idlower                                         { $$ = new ast::annotation::CustomType(*$1);  delete $1;         }
;

bracket_star_opt
    : %empty                                    { $$ = 1; }
    | '[' '*' comma_star_opt_list ']'           { $$ = 1 + $3; }
;

comma_star_opt_list
    : %empty                                    { $$ = 0; }
    | comma_star_opt_list ',' '*'               { $$ = 1 + $1; }
;

expr
    : letdef "in" expr %prec LETIN                              { $$ = new ast::expr::LetIn($1, $3); }
    | expr ';' expr                                             { $$ = new ast::expr::op::Binary($1, $2, $3); }
    | "if" expr "then" expr "else" expr                         { $$ = new ast::expr::If($2, $4, $6); }
    | "if" expr "then" expr                                     { $$ = new ast::expr::If($2, $4, nullptr); }
    | expr ":=" expr                                            { $$ = new ast::expr::op::Binary($1, $2, $3); }
    | expr "||" expr                                            { $$ = new ast::expr::op::Binary($1, $2, $3); }
    | expr "&&" expr                                            { $$ = new ast::expr::op::Binary($1, $2, $3); }
    | expr comp_operator expr %prec COMPOP                      { $$ = new ast::expr::op::Binary($1, $2, $3); }
    | expr add_operator expr %prec ADDOP                        { $$ = new ast::expr::op::Binary($1, $2, $3); }
    | expr mult_operator expr %prec MULTOP                      { $$ = new ast::expr::op::Binary($1, $2, $3); }
    | expr "**" expr                                            { $$ = new ast::expr::op::Binary($1, $2, $3); }
    | unop expr %prec UNOPS                                     { $$ = new ast::expr::op::Unary($1, $2); }
    | "while" expr "do" expr "done"                             { $$ = new ast::expr::While($2, $4); }
    | "for" T_idlower '=' expr "to" expr "do" expr "done"       { $$ = new ast::expr::For(*$2, $4, true, $6, $8); delete $2;  }
    | "for" T_idlower '=' expr "downto" expr "do" expr "done"   { $$ = new ast::expr::For(*$2, $4, false, $6, $8); delete $2; }
    | "match" expr "with" clause bar_clause_opt_list "end"      { $5->insert($5->begin(), std::unique_ptr<ast::utils::match::Clause>($4)); $$ = new ast::expr::Match($2, $5); }
    | "dim" T_intconst T_idlower                                { $$ = new ast::expr::Dim(new ast::expr::literal::Int(*$2), *$3);  }
    | "dim" T_idlower                                           { $$ = new ast::expr::Dim(new ast::expr::literal::Int("1"), *$2); }
    | T_idlower expr_2_list                                     { $$ = new ast::expr::FuncCall(*$1, $2); delete $1; }
    | T_idupper expr_2_list                                     { $$ = new ast::expr::ConstrCall(*$1, $2); delete $1; }
    | expr_2                                                    { $$ = $1; }
;

expr_2
    : T_intconst                            { $$ = new ast::expr::literal::Int(*$1); delete $1;            }
    | T_floatconst                          { $$ = new ast::expr::literal::Float(*$1);                     }
    | T_charconst                           { $$ = new ast::expr::literal::Char(*$1); delete $1;           }
    | T_stringliteral                       { $$ = new ast::expr::literal::String(*$1); delete $1;         }
    | T_idlower                             { $$ = new ast::expr::IdCall(*$1); delete $1;                  }
    | T_idupper                             { $$ = new ast::expr::ConstrCall(*$1); delete $1;     }
    | "true"                                { $$ = new ast::expr::literal::Bool(true);                     }
    | "false"                               { $$ = new ast::expr::literal::Bool(false);                    }
    | '(' ')'                               { $$ = new ast::expr::literal::Unit();                         }
    | '!' expr_2                            { $$ = new ast::expr::op::Unary('!', $2);                      }
    | T_idlower bracket_comma_expr_list     { $$ = new ast::expr::ArrayAccess(*$1, $2); delete $1;         }
    | "new" type                            { $$ = new ast::expr::op::New($2);                             }
    | '(' expr ')'                          { $$ = $2;                                                     }
    | "begin" expr "end"                    { $$ = $2;                                                     }
;

expr_2_list
    : expr_2                { $$ = new std::vector<std::unique_ptr<ast::core::Expression>>(); $$->push_back(std::unique_ptr<ast::core::Expression>($1)); }
    | expr_2_list expr_2    { $1->push_back(std::unique_ptr<ast::core::Expression>($2)); $$ = $1; }
;

unop
    : '+'       { $$ = $1; }
    | '-'       { $$ = $1; }
    | "+."      { $$ = $1; }
    | "-."      { $$ = $1; }
    | "not"     { $$ = $1; }
    | "delete"  { $$ = $1; }
;

comp_operator
    : '='       { $$ = $1; }
    | "<>"      { $$ = $1; }
    | '>'       { $$ = $1; }
    | '<'       { $$ = $1; }
    | "<="      { $$ = $1; }
    | ">="      { $$ = $1; }
    | "=="      { $$ = $1; }
    | "!="      { $$ = $1; }
;

add_operator
    : '+'       { $$ = $1; }
    | '-'       { $$ = $1; }
    | "+."      { $$ = $1; }
    | "-."      { $$ = $1; }
;

mult_operator
    : '*'       { $$ = $1; }
    | '/'       { $$ = $1; }
    | "*."      { $$ = $1; }
    | "/."      { $$ = $1; }
    | "mod"     { $$ = $1; }
;

bar_clause_opt_list
    : %empty                            { $$ = new std::vector<std::unique_ptr<ast::utils::match::Clause>>(); }
    | bar_clause_opt_list '|' clause    { $1->push_back(std::unique_ptr<ast::utils::match::Clause>($3)); $$ = $1; }
;

clause
    : pattern "->" expr         { $$ = new ast::utils::match::Clause($1, $3); }
;

pattern
    : '+' T_intconst            { $$ = new ast::utils::match::PatLiteral(new ast::expr::literal::Int(*$2)); delete $2; }
    | '-' T_intconst            { $$ = new ast::utils::match::PatLiteral(new ast::expr::literal::Int("-" + *$2)); delete $2; }
    | T_intconst                { $$ = new ast::utils::match::PatLiteral(new ast::expr::literal::Int(*$1)); delete $1; }
    | "+." T_floatconst         { $$ = new ast::utils::match::PatLiteral(new ast::expr::literal::Float(*$2)); delete $2; }
    | "-." T_floatconst         { $$ = new ast::utils::match::PatLiteral(new ast::expr::literal::Float("-" + *$2)); delete $2; }
    | T_floatconst              { $$ = new ast::utils::match::PatLiteral(new ast::expr::literal::Float(*$1)); delete $1; }
    | T_charconst               { $$ = new ast::utils::match::PatLiteral(new ast::expr::literal::Char(*$1)); delete $1; }
    | "true"                    { $$ = new ast::utils::match::PatLiteral(new ast::expr::literal::Bool(true)); }
    | "false"                   { $$ = new ast::utils::match::PatLiteral(new ast::expr::literal::Bool(false)); }
    | T_idlower                 { $$ = new ast::utils::match::PatId(*$1); delete $1; }
    | T_idupper                 { $$ = new ast::utils::match::PatConstr(*$1); delete $1; }
    | '(' pattern ')'           { $$ = $2; }
    |  T_idupper pattern_list   { $$ = new ast::utils::match::PatConstr(*$1, $2); delete $1; }
;

pattern_list
    : pattern                   { $$ = new std::vector<std::unique_ptr<ast::utils::match::Pattern>>(); $$->push_back(std::unique_ptr<ast::utils::match::Pattern>($1)); }
    | pattern_list pattern      { $1->push_back(std::unique_ptr<ast::utils::match::Pattern>($2)); $$ = $1; }
;


%%

void yyerror(ast::core::Program *&the_program, std::string_view msg) {
    std::cerr << "Error at line " << yylineno << ": " << msg << std::endl;
    /* fprintf(stderr, "Error at line %d: %s\n", yylineno, msg); */
    std::exit(1);
}

// int main() {
//     /* yydebug = 1; // default val is zero so just comment this to disable */
//     ast::core::Program *program = nullptr;
//     int result = yyparse(program);
//     /* if (program == nullptr) std::cout << "Test"; */
//     auto v = PrintVisitor();
//     program->accept(v);
//     if (result == 0) std::cout << "Success\n";
//     return result;
// }
