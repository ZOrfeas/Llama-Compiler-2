%{
// token definitions, possibly omissible later on
#define T_eof 0
// #include "token-definitions.hpp"
#include <string>
// #include <cstdio>
#include <cstdlib>

#include "driver.hpp"
#include "parser.hpp"   
#include "../ast/ast.hpp"      // Doesn't include parser.hpp

// ast.cpp is linked and it brings parser.hpp and the functions that need yytokentype

// int yylineno = 1;
yy::parser::symbol_type handle_single_char_token(char tok);
int comment_cnt = 0;
// ast::core::Program dummy_program;
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

%%
%{  // this runs whenever yylex is called
    IncludeStack& include_stack = drv.include_stack;
%}
"#include"  { BEGIN(INCLUDE_STMT); }
<INCLUDE_STMT>{W}*  {/* eat whitespace */}
<INCLUDE_STMT>[^ \t\n\r]+ {
    include_stack.push(yytext);
    BEGIN(INITIAL);
}

"and"       { return yy::parser::make_AND();     }
"array"     { return yy::parser::make_ARRAY();   }
"begin"     { return yy::parser::make_BEGIN();   }
"bool"      { return yy::parser::make_BOOL();    }
"char"      { return yy::parser::make_CHAR();    }
"delete"    { return yy::parser::make_DELETE();  }
"dim"       { return yy::parser::make_DIM();     }
"do"        { return yy::parser::make_DO();      }
"done"      { return yy::parser::make_DONE();    }
"downto"    { return yy::parser::make_DOWNTO();  }
"else"      { return yy::parser::make_ELSE();    }
"end"       { return yy::parser::make_END();     }
"false"     { return yy::parser::make_FALSE();   }
"float"     { return yy::parser::make_FLOAT();   }
"for"       { return yy::parser::make_FOR();     }
"if"        { return yy::parser::make_IF();      }
"in"        { return yy::parser::make_IN();      }
"int"       { return yy::parser::make_INT();     }
"let"       { return yy::parser::make_LET();     }
"match"     { return yy::parser::make_MATCH();   }
"mod"       { return yy::parser::make_MOD();     }
"mutable"   { return yy::parser::make_MUTABLE(); }
"new"       { return yy::parser::make_NEW();     }
"not"       { return yy::parser::make_NOT();     }
"of"        { return yy::parser::make_OF();      }
"rec"       { return yy::parser::make_REC();     }
"ref"       { return yy::parser::make_REF();     }
"then"      { return yy::parser::make_THEN();    }
"to"        { return yy::parser::make_TO();      }
"true"      { return yy::parser::make_TRUE();    }
"type"      { return yy::parser::make_TYPE();    }
"unit"      { return yy::parser::make_UNIT();    }
"while"     { return yy::parser::make_WHILE();   }
"with"      { return yy::parser::make_WITH();    }

{Ll}({L}|{D}|_)*    { return yy::parser::make_idlower(std::string(yytext)); }
{Lu}({L}|{D}|_)*    { return yy::parser::make_idupper(std::string(yytext)); }

{D}+                        { 
    return yy::parser::make_intconst(std::string(yytext));
}
({D}+\.{D}+(e[+\-]?{D}+)?)   { 
    return yy::parser::make_floatconst(std::string(yytext));
}

\'([^'"\\\n\r\0]|(\\([ntr0\\'"]|(x{Hex}{Hex}))))\'    { 
    return yy::parser::make_charconst(std::string(yytext));
}
\"((\\.|[^"\\\n])?)*\"    { 
    return yy::parser::make_stringliteral(std::string(yytext));
    }

[=\|+\-\*/!;<>]    { return handle_single_char_token(yytext[0]); }
[\(\)\[\],:]        { return handle_single_char_token(yytext[0]); }

"->" { return yy::parser::make_DASHGREATER(); }
"+." { return yy::parser::make_PLUSDOT(); }
"-." { return yy::parser::make_MINUSDOT(); }
"*." { return yy::parser::make_STARDOT(); }
"/." { return yy::parser::make_SLASHDOT(); }
"**" { return yy::parser::make_DBLSTAR(); }
"&&" { return yy::parser::make_DBLAMPERSAND(); }
"||" { return yy::parser::make_DBLBAR(); }
"<>" { return yy::parser::make_LTGT(); }
"<=" { return yy::parser::make_LEQ(); }
">=" { return yy::parser::make_GEQ(); }
"==" { return yy::parser::make_DBLEQ(); }
"!=" { return yy::parser::make_EXCLAMEQ(); }
":=" { return yy::parser::make_COLONEQ(); }

--.*  { /* nothing */ }

"(*"                        { comment_cnt++; BEGIN(IN_COMMENT);}
<IN_COMMENT>"(*"            { comment_cnt++;}
<IN_COMMENT>"*)"            { comment_cnt--; if (comment_cnt==0) BEGIN(INITIAL);}
<IN_COMMENT>"*"             { /* nothing */ }
<IN_COMMENT>[^(*]+          { /* nothing */ }
<IN_COMMENT>"("             { /* nothing */ }
<IN_COMMENT><<EOF>>         { drv.error("EOF file in comment"); }

{W}+    { /* nothing */ } 
<<EOF>> { if (!include_stack.pop()) return yy::parser::make_STOP(); }
. { drv.error("Illegal character"); }

%%
IncludeStack::IncludeStack(ParseDriver& drv) : owning_driver(drv) {}
bool IncludeStack::is_empty() const {
    return this->filename_stack.empty();
}
bool IncludeStack::has(std::string_view name) const {
    return this->filename_set.find(std::string(name)) != this->filename_set.end();
}
void IncludeStack::push(std::string_view name) {
    if (this->has(name)) {
        std::string err = "Circular include: " + std::string(name);
        owning_driver.error(err);
    }
    yyin = std::fopen(std::string(name).c_str(), "r");
    if (!yyin) {
        std::string err = "Could not open file " + std::string(name);
        owning_driver.error(err);
    }
    yypush_buffer_state(yy_create_buffer(yyin, YY_BUF_SIZE));
    this->filename_stack.push_back(std::string(name));
    this->filename_set.insert(std::string(name));
}
bool IncludeStack::pop() {
    yypop_buffer_state();
    if (!this->is_empty()) {
        std::string current = this->filename_stack.back();
        this->filename_stack.pop_back();
        this->filename_set.erase(current);
    }
    return YY_CURRENT_BUFFER;

}
std::string_view IncludeStack::top() const {
    if (!this->is_empty()) {
        return this->filename_stack.back();
    }
    return "";
}
yy::parser::symbol_type handle_single_char_token(char tok) {
    switch (tok) {
        case '=': return yy::parser::make_EQ();
        case '|': return yy::parser::make_BAR();
        case '+': return yy::parser::make_PLUS();
        case '-': return yy::parser::make_MINUS();
        case '*': return yy::parser::make_STAR();
        case '/': return yy::parser::make_SLASH();
        case '!': return yy::parser::make_EXCLAM();
        case ';': return yy::parser::make_SEMICOLON();
        case '<': return yy::parser::make_LT();
        case '>': return yy::parser::make_GT();
        case '(': return yy::parser::make_LPAREN();
        case ')': return yy::parser::make_RPAREN();
        case '[': return yy::parser::make_LBRACKET();
        case ']': return yy::parser::make_RBRACKET();
        case ',': return yy::parser::make_COMMA();
        case ':': return yy::parser::make_COLON();
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
