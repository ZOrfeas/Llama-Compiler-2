%{
// token definitions, possibly omissible later on
#include <string>
#include <memory>
#include <utility>
#include <fstream>
// #include <cstdlib>

#include "scanner.hpp"
#include "generator.hpp"
#include "parser.hpp"
// #include "location.hpp"
// possibly unnecessary// #include "../ast/ast.hpp"      // Doesn't include parser.hpp

// ast.cpp is linked and it brings parser.hpp and the functions that need yytokentype

#define yyterminate() ast::Parser::make_STOP()

// int yylineno = 1;
auto handle_single_char_token(char tok) -> ast::Parser::symbol_type;
int comment_cnt = 0;
%}
%option yylineno noyywrap nounput noinput

W [ \t\n\r]  
D  [0-9]     
Ll [a-z]     
Lu [A-Z]     
L  [A-Za-z]
Hex [0-9a-f] 

%x IN_COMMENT
%x INCLUDE_STMT

%option c++ yyclass="Scanner" prefix="AST_"

%%
%{  // this runs whenever yylex is called
%}
"#include"  { BEGIN(INCLUDE_STMT); }
<INCLUDE_STMT>{W}*  {/* eat whitespace */}
<INCLUDE_STMT>[^ \t\n\r]+ {
    include_stack.push(yytext);
    BEGIN(INITIAL);
}

"and"       { return ast::Parser::make_AND();     }
"array"     { return ast::Parser::make_ARRAY();   }
"begin"     { return ast::Parser::make_BEGIN();   }
"bool"      { return ast::Parser::make_BOOL();    }
"char"      { return ast::Parser::make_CHAR();    }
"delete"    { return ast::Parser::make_DELETE();  }
"dim"       { return ast::Parser::make_DIM();     }
"do"        { return ast::Parser::make_DO();      }
"done"      { return ast::Parser::make_DONE();    }
"downto"    { return ast::Parser::make_DOWNTO();  }
"else"      { return ast::Parser::make_ELSE();    }
"end"       { return ast::Parser::make_END();     }
"false"     { return ast::Parser::make_FALSE();   }
"float"     { return ast::Parser::make_FLOAT();   }
"for"       { return ast::Parser::make_FOR();     }
"if"        { return ast::Parser::make_IF();      }
"in"        { return ast::Parser::make_IN();      }
"int"       { return ast::Parser::make_INT();     }
"let"       { return ast::Parser::make_LET();     }
"match"     { return ast::Parser::make_MATCH();   }
"mod"       { return ast::Parser::make_MOD();     }
"mutable"   { return ast::Parser::make_MUTABLE(); }
"new"       { return ast::Parser::make_NEW();     }
"not"       { return ast::Parser::make_NOT();     }
"of"        { return ast::Parser::make_OF();      }
"rec"       { return ast::Parser::make_REC();     }
"ref"       { return ast::Parser::make_REF();     }
"then"      { return ast::Parser::make_THEN();    }
"to"        { return ast::Parser::make_TO();      }
"true"      { return ast::Parser::make_TRUE();    }
"type"      { return ast::Parser::make_TYPE();    }
"unit"      { return ast::Parser::make_UNIT();    }
"while"     { return ast::Parser::make_WHILE();   }
"with"      { return ast::Parser::make_WITH();    }

{Ll}({L}|{D}|_)*    { return ast::Parser::make_idlower(std::string(yytext)); }
{Lu}({L}|{D}|_)*    { return ast::Parser::make_idupper(std::string(yytext)); }

{D}+                        { 
    return ast::Parser::make_intconst(std::string(yytext));
}
({D}+\.{D}+(e[+\-]?{D}+)?)   { 
    return ast::Parser::make_floatconst(std::string(yytext));
}

\'([^'"\\\n\r\0]|(\\([ntr0\\'"]|(x{Hex}{Hex}))))\'    { 
    return ast::Parser::make_charconst(std::string(yytext));
}
\"((\\.|[^"\\\n])?)*\"    { 
    return ast::Parser::make_stringliteral(std::string(yytext));
    }

[=\|+\-\*/!;<>]     { return single_char_token_switch(yytext[0]); }
[\(\)\[\],:]        { return single_char_token_switch(yytext[0]); }

"->" { return ast::Parser::make_DASHGREATER(); }
"+." { return ast::Parser::make_PLUSDOT(); }
"-." { return ast::Parser::make_MINUSDOT(); }
"*." { return ast::Parser::make_STARDOT(); }
"/." { return ast::Parser::make_SLASHDOT(); }
"**" { return ast::Parser::make_DBLSTAR(); }
"&&" { return ast::Parser::make_DBLAMPERSAND(); }
"||" { return ast::Parser::make_DBLBAR(); }
"<>" { return ast::Parser::make_LTGT(); }
"<=" { return ast::Parser::make_LEQ(); }
">=" { return ast::Parser::make_GEQ(); }
"==" { return ast::Parser::make_DBLEQ(); }
"!=" { return ast::Parser::make_EXCLAMEQ(); }
":=" { return ast::Parser::make_COLONEQ(); }

--.*  { /* nothing */ }

"(*"                        { comment_cnt++; BEGIN(IN_COMMENT); }
<IN_COMMENT>"(*"            { comment_cnt++; }
<IN_COMMENT>"*)"            { comment_cnt--; if (comment_cnt==0) BEGIN(INITIAL); }
<IN_COMMENT>"*"             { /* nothing */ }
<IN_COMMENT>[^(*]+          { /* nothing */ }
<IN_COMMENT>"("             { /* nothing */ }
<IN_COMMENT><<EOF>>         { gen.error("EOF file in comment"); }

{W}+    { /* nothing */ } 
<<EOF>> { if (include_stack.pop()) return yyterminate(); }
. { gen.error("Illegal character"); }

%%
ast::Scanner::IncludeStack::IncludeStack(ast::Scanner& scanner) : scanner(scanner) {}
auto ast::Scanner::IncludeStack::is_empty() const -> bool {
    return this->filename_stack.empty();
}
auto ast::Scanner::IncludeStack::has(std::string_view name) const -> bool {
    return this->filename_set.find(std::string(name)) != this->filename_set.end();
}
auto ast::Scanner::IncludeStack::push(std::string_view name) -> void {
    if (this->has(name)) {
        std::string err = "Circular include: " + std::string(name);
        scanner.gen.error(err);
    }
    auto newfilestream = std::make_unique<std::ifstream>(std::string(name));
    if (!newfilestream->is_open()) {
        std::string err = "Could not open file " + std::string(name);
        scanner.gen.error(err);
    }
    scanner.yypush_buffer_state(
        scanner.yy_create_buffer(newfilestream.get(), YY_BUF_SIZE));
    this->filename_stack.push_back(std::make_pair(std::string(name), std::move(newfilestream)));
    this->filename_set.insert(std::string(name));
}
auto ast::Scanner::IncludeStack::pop() -> bool {
    scanner.yypop_buffer_state();
    if (!this->is_empty()) {
        std::string current = this->filename_stack.back().first;
        this->filename_stack.pop_back();
        this->filename_set.erase(current);
    }
    return this->is_empty();

}
auto ast::Scanner::IncludeStack::top() const -> std::string_view {
    if (!this->is_empty()) {
        return this->filename_stack.back().first;
    }
    return "";
}
auto ast::Scanner::single_char_token_switch(char tok) 
    -> ast::Parser::symbol_type {
    switch (tok) {
        case '=': return ast::Parser::make_EQ();
        case '|': return ast::Parser::make_BAR();
        case '+': return ast::Parser::make_PLUS();
        case '-': return ast::Parser::make_MINUS();
        case '*': return ast::Parser::make_STAR();
        case '/': return ast::Parser::make_SLASH();
        case '!': return ast::Parser::make_EXCLAM();
        case ';': return ast::Parser::make_SEMICOLON();
        case '<': return ast::Parser::make_LT();
        case '>': return ast::Parser::make_GT();
        case '(': return ast::Parser::make_LPAREN();
        case ')': return ast::Parser::make_RPAREN();
        case '[': return ast::Parser::make_LBRACKET();
        case ']': return ast::Parser::make_RBRACKET();
        case ',': return ast::Parser::make_COMMA();
        case ':': return ast::Parser::make_COLON();
        default: std::exit(1);
    }
}
/* void lex_error(const char *msg) {
    fprintf(stderr, "Error at line %d, token %c (ASCII: %d): %s  \n",
        yylineno, yytext[0], yytext[0], msg);
    exit(1);
} */

/* int main() {
    int token;
    do {
        token = yylex();
        printf("token=%d, lexeme=\"%s\"\n", token, yytext);
    } while (token != T_eof);
} */
