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
    bool parse_type();
    bool parse_type_helper();
    
    std::vector<token> tokens;
    std::vector<token>::iterator it;
};