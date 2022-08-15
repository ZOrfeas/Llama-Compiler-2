#pragma once
#include <vector>

#include "common.hpp"

struct reserved {
    std::string name;
    token_kind t;
};

class Lexer {
public:
    Lexer(std::string &text);
    void lex();
    std::vector<token> get_tokens();
    void print_tokens();

private:
    /* Finds the next token in the sequence */
    token next_token();
    bool match_prefix_word_with(std::string s);
    void match_whitespace();
    bool match_id();
    bool match_Id();
    bool match_single_line_comment();
    bool match_multi_line_comment();
    bool match_literal_int();
    bool match_literal_float();
    bool match_literal_char();
    bool match_literal_string();
    bool match_end();
    void match_unmatched();

    // Input and output
    std::string text;
    std::vector<token> tokens;

    // Initialized by lex()
    std::string::iterator it;
    position pos;
    std::string cur_s;
};
