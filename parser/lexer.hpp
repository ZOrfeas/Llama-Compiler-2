#ifndef PARSE_LEXER_HPP
#define PARSE_LEXER_HPP

#include "common.hpp"
#include "source.hpp"
#include <iostream>
#include <optional>
#include <string_view>
#include <vector>

// error-handling notes:
//   exceptions will be thrown, and the top level of the Lexer will be
//   wrapped in a try-catch.
//   depending on Lexer paramereters, the Lexer can try to recover and store the
//   error or throw it higher up the stack.

namespace lla {
    class Lexer {
    public:
        Lexer(Source &, bool = true);
        Lexer(Source &&, bool = true);
        Lexer(std::string_view, bool = true);
        auto lex() -> Lexer const &;

        [[nodiscard]] auto get_tokens() const -> std::vector<token> const &;
        [[nodiscard]] auto get_errors() const
            -> std::vector<parse_error> const &;
        [[nodiscard]] auto extract_tokens_and_errors()
            && -> std::pair<std::vector<token>, std::vector<parse_error>>;
        auto pretty_print_tokens() const -> void;

    private:
        std::optional<Source> owned_src;
        Source &src;
        bool crash_on_error;
        std::vector<token> tokens; // each token object is only a
                                   // string_view so no duplication happens
        //! NOTE: I'll keep track of active filename by keeping the indexes and
        //! checking if we passed any after every token read
        std::vector<parse_error> errors;
        struct {
            source_position cur_pos;
            Source::const_iterator src_it;
        } state;
        auto match_token() -> token;

        auto eat_whitespace() -> void;
        auto match_eof() -> std::optional<token>;
        auto match_single_line_comment() -> std::optional<token>;
        auto match_multi_line_comment() -> std::optional<token>;
        auto match_reserved_word() -> std::optional<token>;
        auto match_lowercase_id() -> std::optional<token>;
        auto match_uppercase_id() -> std::optional<token>;
        auto match_float_literal() -> std::optional<token>;
        auto match_int_literal() -> std::optional<token>;
        auto match_char_literal() -> std::optional<token>;
        auto match_string_literal() -> std::optional<token>;
        auto match_multi_char_symop() -> std::optional<token>;
        auto match_single_char_sep_or_symop() -> std::optional<token>;
        auto match_unmatched() -> std::optional<token>;

        auto match_any_id(lexeme_t) -> std::optional<token>;
        auto match_with_str(std::string_view, lexeme_t) -> std::optional<token>;

        template <src_pos_advancer F>
        inline auto finalize_token(lexeme_t tok_type,
                                   Source::const_iterator tok_val_end_it, F f)
            -> token {
            source_position next_pos = f(this->state.cur_pos);
            token tok{tok_type,
                      this->state.cur_pos,
                      next_pos,
                      {this->state.src_it, tok_val_end_it}};
            this->state.cur_pos = next_pos;
            this->state.src_it = tok_val_end_it;
            return tok;
        }

        // helper/util funcs

        auto is_eof(Source::const_iterator) -> bool;
        /**
         * @brief Given iterator is advanced up until after the first non digit
         * character
         */
        auto consume_digits(Source::const_iterator &) -> void;
    };
} // namespace lla

#endif // PARSE_LEXER_HPP