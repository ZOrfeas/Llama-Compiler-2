#pragma once
#include "common.hpp"
#include "lexer.hpp"

class Parser {
public:
    Parser(Lexer& lexer);
    void parse();

private:
    token peek_token();
    void consume_token();
    token peek_consume_token();
    bool parse_program();
    bool parse_letdef();
    bool parse_def();
    bool parse_par();
    bool parse_expr();
    bool parse_typedef();
    bool parse_tdef();
    bool parse_constr();
    bool parse_type();
    bool parse_type_helper();
    
    std::vector<token> tokens;
    std::vector<token>::iterator it;
};