%require "3.8.2"
%skeleton "lalr1.cc"

%defines
%define api.token.constructor
%define api.value.type variant
/* %define parser.assert */ /* This has a runtime overhead with RTTI but provides some more guarantees */ 
%define parse.lac full /* improves error reporting (supposedly without much overhead)*/
%define api.token.raw /* performance improved(a bit). cannot use plain chars in scanner */
%define api.value.automove /* performance improved. Care needed to not use same $n var twice */
%define api.parser.class { Parser }
%define api.namespace { ast }
/* %locations */

%code requires {
    #include <iostream>
    #include <string>
    #include <cstdlib>
    #include <vector>
    #include <memory>
    #include <string_view>
    #include "../ast/ast.hpp"

    #include "../log/log.hpp"

    // #define YYDEBUG 1 // comment out to disable debug feature compilation
    namespace ast {
        class Scanner;
        class Generator;
    }
}
%expect 24
%define parse.error detailed /* or 'verbose' */

%code top {
    // check which of those are necessary
    #include "scanner.hpp"
    #include "generator.hpp"
    // #include "parser.hpp"
    static auto yylex(ast::Scanner& scanner) -> ast::Parser::symbol_type {
        return scanner.get_next_token();
    }
    using namespace ast;
    using namespace ast::annotation;
    using namespace ast::defs;
    using namespace ast::exprs;
    using namespace ast::stmts;
}
%lex-param { ast::Scanner& scanner }
%parse-param { ast::Scanner& scanner }
%parse-param { ast::Generator& gen }
/* %locations */

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

%nterm<std::unique_ptr<ast::Program>> program

%nterm<std::unique_ptr<ast::stmts::DefStmt>> definition_choice 
%nterm<std::unique_ptr<ast::stmts::LetStmt>> letdef
%nterm<std::unique_ptr<ast::stmts::TypeStmt>> typedef
%nterm<std::vector<std::unique_ptr<ast::stmts::DefStmt>>> program_list

%nterm<std::unique_ptr<ast::defs::LetDef>> def
%nterm<std::vector<std::unique_ptr<ast::defs::LetDef>>> and_def_opt_list
%nterm<std::unique_ptr<ast::defs::TypeDef>> tdef
%nterm<std::vector<std::unique_ptr<ast::defs::TypeDef>>> and_tdef_opt_list

%nterm<std::unique_ptr<ast::defs::Param>> par
%nterm<std::vector<std::unique_ptr<ast::defs::Param>>> par_list

%nterm<std::unique_ptr<ast::annotation::TypeAnnotation>> type
%nterm<std::vector<std::unique_ptr<ast::annotation::TypeAnnotation>>> of_type_opt_list at_least_one_type

%nterm<std::unique_ptr<ast::defs::Constructor>> constr
%nterm<std::vector<std::unique_ptr<ast::defs::Constructor>>> bar_constr_opt_list

%nterm<int> bracket_star_opt comma_star_opt_list

%nterm<std::unique_ptr<ast::exprs::Expression>> expr expr_2
%nterm<std::vector<std::unique_ptr<ast::exprs::Expression>>> bracket_comma_expr_list comma_expr_opt_list expr_2_list

%nterm<ast::exprs::Op> unop comp_operator add_operator mult_operator

%nterm<std::unique_ptr<ast::exprs::match::Clause>> clause
%nterm<std::vector<std::unique_ptr<ast::exprs::match::Clause>>> bar_clause_opt_list
%nterm<std::unique_ptr<ast::exprs::match::Pattern>> pattern
%nterm<std::vector<std::unique_ptr<ast::exprs::match::Pattern>>> pattern_list

/* %printer { yyo << $$; } <*>; */

%start program
%%
program
    : program_list {
        auto ast = std::make_unique<Program>($1);
        gen.set_ast(std::move(ast));
    }
;

program_list
    : %empty                         { $$ = std::vector<std::unique_ptr<stmts::DefStmt>>(); }
    | program_list definition_choice { $$ = $1; $$.push_back($2); }
;

definition_choice
    : letdef  { $$ = std::make_unique<DefStmt>(std::move(*$1)); }
    | typedef { $$ = std::make_unique<DefStmt>(std::move(*$1)); }
;

letdef
    : "let" def and_def_opt_list {
        auto vec = $3;
        vec.insert(vec.begin(), $2);
        $$ = std::make_unique<stmts::LetStmt>(std::move(vec), false);
    }
    | "let" "rec" def and_def_opt_list {
        auto vec = $4;
        vec.insert(vec.begin(), $3);
        $$ = std::make_unique<stmts::LetStmt>(std::move(vec), true);
    }
;

typedef
    : "type" tdef and_tdef_opt_list {
        auto vec = $3;
        vec.insert(vec.begin(), $2);
        $$ = std::make_unique<stmts::TypeStmt>(std::move(vec));
    }
;

def
    : idlower "=" expr                                    { $$ = LetDef::make<Constant>($1, $3); }
    | idlower ":" type "=" expr                           { $$ = LetDef::make<Constant>($1, $5, $3); }
    | idlower par_list "=" expr                           { $$ = LetDef::make<Function>($1, $2, $4); }
    | idlower par_list ":" type "=" expr                  { $$ = LetDef::make<Function>($1, $2, $6, $4); }
    | "mutable" idlower bracket_comma_expr_list           { $$ = LetDef::make<Array>($2, $3); }
    | "mutable" idlower bracket_comma_expr_list ":" type  { $$ = LetDef::make<Array>($2, $3, $5); }
    | "mutable" idlower                                   { $$ = LetDef::make<Variable>($2); }
    | "mutable" idlower ":" type                          { $$ = LetDef::make<Variable>($2, $4); }
;

par_list
    : par {
        $$ = std::vector<std::unique_ptr<defs::Param>>();
        $$.push_back($1);
    }
    | par_list par { $$ = $1; $$.push_back($2); }
;

bracket_comma_expr_list
    : "[" expr comma_expr_opt_list "]" {
        auto vec = $3;
        vec.insert(vec.begin(), $2);
        $$ = std::move(vec);
    }
;

comma_expr_opt_list
    : %empty                       { $$ = std::vector<std::unique_ptr<exprs::Expression>>(); }
    | comma_expr_opt_list "," expr { $$ = $1; $$.push_back($3); }
;

tdef
    : idlower "=" constr bar_constr_opt_list {
        auto vec = $4;
        vec.insert(vec.begin(), $3);
        $$ = std::make_unique<defs::TypeDef>($1, std::move(vec));
    }
;

bar_constr_opt_list
    : %empty                         { $$ = std::vector<std::unique_ptr<defs::Constructor>>(); }
    | bar_constr_opt_list "|" constr { $$ = $1; $$.push_back($3); }
;

and_def_opt_list
    : %empty                      { $$ = std::vector<std::unique_ptr<defs::LetDef>>(); }
    | and_def_opt_list "and" def  { $$ = $1; $$.push_back($3); }
;

and_tdef_opt_list
    : %empty                       { $$ = std::vector<std::unique_ptr<defs::TypeDef>>(); }
    | and_tdef_opt_list "and" tdef { $$ = $1; $$.push_back($3); }
;

constr
    : idupper of_type_opt_list { $$ = std::make_unique<defs::Constructor>($1, $2); }
;

of_type_opt_list
    : %empty { $$ = std::vector<std::unique_ptr<annotation::TypeAnnotation>>(); }
    | "of" at_least_one_type { $$ = $2; }
;

at_least_one_type
    : type {
        $$ = std::vector<std::unique_ptr<annotation::TypeAnnotation>>();
        $$.push_back($1);
    }
    | at_least_one_type type {
        $$ = $1;
        $$.push_back($2);
    }
;

par
    : idlower                     { $$ = std::make_unique<defs::Param>($1); }
    | "(" idlower ":" type ")"    { $$ = std::make_unique<defs::Param>($2, $4); }
;

type
    : "unit"                                            { $$ = TypeAnnotation::make<BasicType>(typesys::TypeEnum::UNIT); }
    | "int"                                             { $$ = TypeAnnotation::make<BasicType>(typesys::TypeEnum::INT); }
    | "char"                                            { $$ = TypeAnnotation::make<BasicType>(typesys::TypeEnum::CHAR); }
    | "bool"                                            { $$ = TypeAnnotation::make<BasicType>(typesys::TypeEnum::BOOL); }
    | "float"                                           { $$ = TypeAnnotation::make<BasicType>(typesys::TypeEnum::FLOAT); }
    | "(" type ")"                                      { $$ = $2; }
    | type "->" type                                    { $$ = TypeAnnotation::make<FunctionType>($1, $3); }
    | type "ref"                                        { $$ = TypeAnnotation::make<RefType>($1); }
    | "array" bracket_star_opt "of" type %prec ARRAYOF  { $$ = TypeAnnotation::make<ArrayType>($2, $4); }
    | idlower                                           { $$ = TypeAnnotation::make<CustomType>($1); }
;

bracket_star_opt
    : %empty                                    { $$ = 1; }
    | "[" "*" comma_star_opt_list "]"           { $$ = 1 + $3; }
;

comma_star_opt_list
    : %empty                                    { $$ = 0; }
    | comma_star_opt_list "," "*"               { $$ = $1 + 1; }
;

expr
    : letdef "in" expr %prec LETIN                          { $$ = Expression::make<LetIn>($1, $3); }
    | expr ";" expr                                         { $$ = Expression::make<BinaryOp>(Op::Semicolon, $1, $3); }
    | "if" expr "then" expr "else" expr                     { $$ = Expression::make<If>($2, $4, $6); }
    | "if" expr "then" expr                                 { $$ = Expression::make<If>($2, $4, nullptr); }
    | expr ":=" expr                                        { $$ = Expression::make<BinaryOp>(Op::Assign, $1, $3); }
    | expr "||" expr                                        { $$ = Expression::make<BinaryOp>(Op::Or, $1, $3); }
    | expr "&&" expr                                        { $$ = Expression::make<BinaryOp>(Op::And, $1, $3); }
    | expr comp_operator expr %prec COMPOP                  { $$ = Expression::make<BinaryOp>($2, $1, $3); }
    | expr add_operator expr %prec ADDOP                    { $$ = Expression::make<BinaryOp>($2, $1, $3); }
    | expr mult_operator expr %prec MULTOP                  { $$ = Expression::make<BinaryOp>($2, $1, $3); }
    | expr "**" expr                                        { $$ = Expression::make<BinaryOp>(Op::Pow, $1, $3); }
    | unop expr %prec UNOPS                                 { $$ = Expression::make<UnaryOp>($1, $2); }
    | "while" expr "do" expr "done"                         { $$ = Expression::make<While>($2, $4); }
    | "for" idlower "=" expr "to" expr "do" expr "done"     { $$ = Expression::make<For>($2, $4, $6, $8, true); }
    | "for" idlower "=" expr "downto" expr "do" expr "done" { $$ = Expression::make<For>($2, $4, $6, $8, false); }
    | "match" expr "with" clause bar_clause_opt_list "end" {
        auto vec = $5;
        vec.insert(vec.begin(), $4);
        $$ = Expression::make<Match>($2, std::move(vec));
    }
    | "dim" intconst idlower                                { $$ = Expression::make<Dim>($3, literals::Int($2)); }
    | "dim" idlower                                         { $$ = Expression::make<Dim>($2); }
    | idlower expr_2_list                                   { $$ = Expression::make<FuncCall>($1, $2); }
    | idupper expr_2_list                                   { $$ = Expression::make<ConstrCall>($1, $2); }
    | expr_2                                                { $$ = $1; }
;

expr_2
    : intconst                        { $$ = Expression::make<literals::Int>($1); }
    | floatconst                      { $$ = Expression::make<literals::Float>($1); }
    | charconst                       { $$ = Expression::make<literals::Char>($1); }
    | stringliteral                   { $$ = Expression::make<literals::String>($1); }
    | idlower                         { $$ = Expression::make<IdCall>($1); }
    | idupper                         { $$ = Expression::make<ConstrCall>($1); }
    | "true"                          { $$ = Expression::make<literals::Bool>(true); }
    | "false"                         { $$ = Expression::make<literals::Bool>(false); }
    | "(" ")"                         { $$ = Expression::make<literals::Unit>(); }
    | "!" expr_2                      { $$ = Expression::make<UnaryOp>(Op::Deref, $2); }
    | idlower bracket_comma_expr_list { $$ = Expression::make<ArrayAccess>($1, $2); }
    | "new" type                      { $$ = Expression::make<NewOp>($2);}
    | "(" expr ")"                    { $$ = $2; }
    | "begin" expr "end"              { $$ = $2; }
;

expr_2_list
    : expr_2 {
        $$ = std::vector<std::unique_ptr<ast::exprs::Expression>>();
        $$.push_back(std::move($1));
    }
    | expr_2_list expr_2 {
        auto vec = $1;
        vec.push_back(std::move($2));
        $$ = std::move(vec);
    }
;

unop
    : "+"       { $$ = exprs::Op::Plus;     }
    | "-"       { $$ = exprs::Op::Minus;    }
    | "+."      { $$ = exprs::Op::PlusFlt;  }
    | "-."      { $$ = exprs::Op::MinusFlt; }
    | "not"     { $$ = exprs::Op::Not;      }
    | "delete"  { $$ = exprs::Op::Delete;   }
;

comp_operator
    : "="       { $$ = exprs::Op::StructEq; }
    | "<>"      { $$ = exprs::Op::StructNe; }
    | ">"       { $$ = exprs::Op::Gt;       }
    | "<"       { $$ = exprs::Op::Lt;       }
    | "<="      { $$ = exprs::Op::Le;       }
    | ">="      { $$ = exprs::Op::Ge;       }
    | "=="      { $$ = exprs::Op::NatEq;    }
    | "!="      { $$ = exprs::Op::NatNe;    }
;

add_operator
    : "+"       { $$ = exprs::Op::Plus;     }
    | "-"       { $$ = exprs::Op::Minus;    }
    | "+."      { $$ = exprs::Op::PlusFlt;  }
    | "-."      { $$ = exprs::Op::MinusFlt; }
;

mult_operator
    : "*"       { $$ = exprs::Op::Mult;    }
    | "/"       { $$ = exprs::Op::Div;     }
    | "*."      { $$ = exprs::Op::MultFlt; }
    | "/."      { $$ = exprs::Op::DivFlt;  }
    | "mod"     { $$ = exprs::Op::Mod;     }
;

bar_clause_opt_list
    : %empty                            { $$ = std::vector<std::unique_ptr<match::Clause>>(); }
    | bar_clause_opt_list "|" clause    { $$ = $1; $$.push_back($3); }
;

clause
    : pattern "->" expr         { $$ = std::make_unique<match::Clause>($1, $3); }
;

pattern
    : "+" intconst          { $$ = match::Pattern::make<literals::Int>($2); }
    | "-" intconst          { $$ = match::Pattern::make<literals::Int>(std::string("-") + $2); }
    | intconst              { $$ = match::Pattern::make<literals::Int>($1); }
    | "+." floatconst       { $$ = match::Pattern::make<literals::Float>($2); }
    | "-." floatconst       { $$ = match::Pattern::make<literals::Float>(std::string("-") + $2); }
    | floatconst            { $$ = match::Pattern::make<literals::Float>($1); }
    | charconst             { $$ = match::Pattern::make<literals::Char>($1); }
    | "true"                { $$ = match::Pattern::make<literals::Bool>(true); }
    | "false"               { $$ = match::Pattern::make<literals::Bool>(false); }
    | idupper               { $$ = match::Pattern::make<match::ConstrPattern>($1); }
    | idlower               { $$ = match::Pattern::make<match::IdPattern>($1); }
    | "(" pattern ")"       { $$ = $2; }
    |  idupper pattern_list { $$ = match::Pattern::make<match::ConstrPattern>($1, $2); }
;
pattern_list
    : pattern {
        $$ = std::vector<std::unique_ptr<match::Pattern>>();
        $$.push_back($1);
    }
    | pattern_list pattern {
        auto vec = $1;
        vec.push_back($2);
        $$ = std::move(vec);
    }
;


%%
namespace ast {
void Parser::error (const std::string & msg) {
    gen.error(msg);
}
}
