#ifndef LEXER_HPP
#define LEXER_HPP

#include "common.hpp"
#include <string_view>
#include <vector>

class Lexer {
public:
    Lexer(std::string_view);
    // void lex();
    // std::vector<token> get_tokens();
    auto get_next_token() -> token;
    auto lookahead() -> token;
    auto fast_forward_to_lookahead() -> void;

    auto flush_print_tokens() -> void;
    static auto print_token(token) -> void;

private:
    auto read_one_token() -> token;
    /* Finds the next token in the sequence */
    auto match_prefix_word_with(std::string) -> bool;
    auto match_whitespace() -> void;
    auto match_id() -> bool;
    auto match_Id() -> bool;
    auto match_single_line_comment() -> bool;
    auto match_multi_line_comment() -> bool;
    auto match_literal_int() -> bool;
    auto match_literal_float() -> bool;
    auto match_literal_char() -> bool;
    auto match_literal_string() -> bool;
    auto match_end() -> bool;
    auto match_unmatched() -> void;

    // Input and output
    std::string text;
    std::vector<token> token_buf;
    std::vector<token>::iterator cur_token_it;

    // Initialized by lex()
    std::string::iterator it;
    position pos;
    std::string cur_s;

    static auto read_file_to_string(std::string_view) -> std::string;
    struct reserved {
        std::string name;
        token_kind t;
    };
};

#endif // LEXER_HPP