#include "lexer.hpp"
#include "common.hpp"
#include <algorithm>
#include <cctype>
#include <fstream>
#include <iterator>
#include <optional>

#include "fmt/core.h"
using namespace lla;
// // // // // // // //
static auto file_to_char_vec(std::string_view filename) -> std::vector<char> {
    // std::ios::ate places the file pointer at the end of the file
    std::ifstream file(filename, std::ios::binary | std::ios::ate);

    // Weird way to read file in one string, should be fast
    auto size = file.tellg();
    std::vector<char> text(size);
    file.seekg(0);
    file.read(text.data(), size);
    return text;
}
Source::Source(std::string_view filename)
    : is_preprocessed(false), text(file_to_char_vec(filename)),
      src_file_indexes({0}), filenames({std::string(filename)}) {}
auto Source::preprocess() -> void {
    if (this->is_preprocessed) {
        return;
    }
    // TODO: Implemnt preprocessor

    this->is_preprocessed = true;
}
auto Source::begin() const -> const_iterator { return this->text.begin(); }
auto Source::end() const -> const_iterator { return this->text.end(); }
auto Source::get_filename(const_iterator it) const -> std::string_view {
    // TODO: Implement
    return this->filenames[0];
}
auto Source::it_to_src_pos(const_iterator it) const -> source_position {
    source_position pos{1, 1, this->filenames[0]};
    auto src_it = this->text.begin();
    for (; src_it != this->text.end() && src_it != it; ++src_it) {
        if (*src_it == '\n') {
            ++pos.lineno;
            pos.colno = 1;
        } else {
            ++pos.colno;
        }
    }
    if (src_it == this->text.end() && it != this->text.end()) {
        throw parse_error{
            pos, "tried to get source position of out of bounds iterator",
            true};
    }
    pos.filename = this->get_filename(it);
    return pos;
}

// // // // // // // //
Lexer::Lexer(Source &src, bool crash_on_error)
    : src(src), crash_on_error(crash_on_error), tokens({}), errors({}) {}
auto Lexer::lex() -> void {
    this->src.preprocess();
    this->state.src_it = this->src.begin();
    // TODO: Implement filename tracking
    this->state.cur_pos =
        source_position{1, 1, this->src.get_filename(this->state.src_it)};
    // TODO: create two loop funcs to pass through the if (crash_on_error) only
    // once
    while (true) {
        try {
            auto token = this->match_token();
            this->tokens.push_back(token);
            if (token.tok_type == lexeme_t::end_of_file) {
                break;
            }
        } catch (parse_error &e) {
            if (crash_on_error) {
                throw e;
            } else {
                // TODO: Implement error recovery :)
                this->errors.push_back(e);
            }
        }
    }
}
auto Lexer::get_tokens() const -> const std::vector<token> & {
    return this->tokens;
}
auto Lexer::pretty_print_tokens() const -> void {
    // TODO: Improve by accepting different destination streams
    // TODO: Improve by counting max column widths
    const auto find_max_width = [this]<typename F>(F getter) {
        const auto max_it =
            std::max_element(this->tokens.begin(), this->tokens.end(),
                             [getter](const auto &a, const auto &b) {
                                 return getter(a) < getter(b);
                             });
        return getter(*max_it);
    };
    const auto max_tok_width =
        find_max_width(+[](const token &t) { return t.to_string().size(); });
    const auto max_src_file_width = find_max_width(
        +[](const token &t) { return t.src_start.to_string().size(); });

    fmt::print(stdout, "Found {} tokens\n", this->tokens.size());
    for (const auto &token : this->tokens) {
        if (token.tok_type == lexeme_t::end_of_file) {
            fmt::print("EOF at {}\n", token.src_start.to_string());
            continue;
        }
        fmt::print("{0:>{3}} | at {1:<{4}} -> {2:<{4}}\n", token.to_string(),
                   token.src_start.to_string(), token.src_end.to_string(),
                   max_tok_width, max_src_file_width);
    }
}

auto Lexer::match_token() -> token {
    // TODO: Think if first is_eof check of each matcher can be removed

    // order of matchers is the order in which they are run
    static const std::array match_funcs{&Lexer::match_eof,
                                        &Lexer::match_single_line_comment,
                                        &Lexer::match_multi_line_comment,
                                        &Lexer::match_reserved_word,
                                        &Lexer::match_lowercase_id,
                                        &Lexer::match_uppercase_id,
                                        &Lexer::match_float_literal,
                                        &Lexer::match_int_literal,
                                        &Lexer::match_char_literal,
                                        &Lexer::match_string_literal,
                                        &Lexer::match_multi_char_symop,
                                        &Lexer::match_single_char_sep_or_symop,
                                        &Lexer::match_unmatched};

    eat_whitespace();
    for (auto const match_func : match_funcs) {
        if (auto tok = (this->*match_func)()) {
            return *tok;
        }
    }
    throw parse_error{this->state.cur_pos, "Invalid parser state reached",
                      true};
}
auto Lexer::eat_whitespace() -> void {
    auto temp_src_it = this->state.src_it;
    for (; !is_eof(temp_src_it) && std::isspace(*temp_src_it); ++temp_src_it) {
        auto c = *temp_src_it;
        if (c == '\n') {
            ++this->state.cur_pos.lineno;
            this->state.cur_pos.colno = 1;
        } else {
            ++this->state.cur_pos.colno;
        }
    }
    this->state.src_it = temp_src_it;
    return;
}
auto Lexer::match_eof() -> std::optional<token> {
    if (is_eof(this->state.src_it)) {
        return token{
            lexeme_t::end_of_file,
            this->state.cur_pos,
            this->state.cur_pos,
        };
    }
    return std::nullopt;
}
auto Lexer::match_single_line_comment() -> std::optional<token> {
    if (is_eof(this->state.src_it) || is_eof(this->state.src_it + 1) ||
        *this->state.src_it != '-' || *(this->state.src_it + 1) != '-') {
        return std::nullopt;
    }
    auto tmp_src_it = this->state.src_it + 2;
    while (!is_eof(tmp_src_it) && *tmp_src_it != '\n') {
        // TODO: test on windows newlines
        ++tmp_src_it;
    }

    auto token = finalize_token(
        lexeme_t::COMMENT, tmp_src_it, [this, tmp_src_it](source_position pos) {
            pos.colno += std::distance(this->state.src_it, tmp_src_it);
            return pos;
        });
    ++this->state.cur_pos.lineno;
    this->state.cur_pos.colno = 1;
    return token;
}
auto Lexer::match_multi_line_comment() -> std::optional<token> {
    if (is_eof(this->state.src_it) || is_eof(this->state.src_it + 1) ||
        *this->state.src_it != '(' || *(this->state.src_it + 1) != '*') {
        return std::nullopt;
    }
    int nesting = 1;
    auto tmp_src_it = this->state.src_it + 2;
    auto tmp_cur_pos = this->state.cur_pos;
    tmp_cur_pos.colno += 2;
    while (!is_eof(tmp_src_it) && nesting > 0) {
        if (!is_eof(tmp_src_it + 1)) {
            // TODO: think of a way to avoid code duplication
            if (*tmp_src_it == '(' && *(tmp_src_it + 1) == '*') {
                ++nesting;
                tmp_src_it += 2;
                tmp_cur_pos.colno += 2;
                continue;
            } else if (*tmp_src_it == '*' && *(tmp_src_it + 1) == ')') {
                --nesting;
                tmp_src_it += 2;
                tmp_cur_pos.colno += 2;
                continue;
            }
        }
        if (*tmp_src_it == '\n') {
            // TODO: test on windows newlines
            ++tmp_src_it;
            ++tmp_cur_pos.lineno;
            tmp_cur_pos.colno = 1;
        } else {
            ++tmp_src_it;
            ++tmp_cur_pos.colno;
        }
    }
    if (nesting != 0) {
        throw parse_error{
            tmp_cur_pos,
            "unterminated multi-line comment",
        };
    }
    return finalize_token(
        lexeme_t::COMMENT, tmp_src_it,
        [&tmp_cur_pos](source_position pos) { return tmp_cur_pos; });
}
auto Lexer::match_reserved_word() -> std::optional<token> {
    static constexpr std::array lexemes{
        lexeme_t::AND,   lexeme_t::ARRAY,   lexeme_t::BEGIN, lexeme_t::BOOL,
        lexeme_t::CHAR,  lexeme_t::DELETE,  lexeme_t::DIM,   lexeme_t::DO,
        lexeme_t::DONE,  lexeme_t::DOWNTO,  lexeme_t::ELSE,  lexeme_t::END,
        lexeme_t::FALSE, lexeme_t::FLOAT,   lexeme_t::FOR,   lexeme_t::IF,
        lexeme_t::IN,    lexeme_t::INT,     lexeme_t::LET,   lexeme_t::MATCH,
        lexeme_t::MOD,   lexeme_t::MUTABLE, lexeme_t::NEW,   lexeme_t::NOT,
        lexeme_t::OF,    lexeme_t::REC,     lexeme_t::REF,   lexeme_t::THEN,
        lexeme_t::TO,    lexeme_t::TRUE,    lexeme_t::TYPE,  lexeme_t::UNIT,
        lexeme_t::WHILE, lexeme_t::WITH};
    for (const auto lexeme : lexemes) {
        if (auto tok = match_with_str(as<std::string_view>(lexeme), lexeme)) {
            return tok;
        }
    }
    return std::nullopt;
}
auto Lexer::match_lowercase_id() -> std::optional<token> {
    if (is_eof(this->state.src_it) || !std::islower(*this->state.src_it)) {
        return std::nullopt;
    }
    return match_any_id(lexeme_t::idlower);
}
auto Lexer::match_uppercase_id() -> std::optional<token> {
    if (is_eof(this->state.src_it) || !std::isupper(*this->state.src_it)) {
        return std::nullopt;
    }
    return match_any_id(lexeme_t::idupper);
}
auto Lexer::match_float_literal() -> std::optional<token> {
    // TODO: think of a way to avoid code duplication
    if (is_eof(this->state.src_it) || !std::isdigit(*this->state.src_it)) {
        return std::nullopt;
    }
    auto tmp_src_it = this->state.src_it;
    consume_digits(tmp_src_it);
    if (tmp_src_it != this->src.end() || *tmp_src_it != '.') {
        return std::nullopt;
    }
    digit_or_error(++tmp_src_it, "expected digit after decimal point");
    consume_digits(tmp_src_it);
    if (is_eof(tmp_src_it) || *tmp_src_it != 'e') {
        // valid float without exponent
        return finalize_token(lexeme_t::floatconst, tmp_src_it,
                              [this, tmp_src_it](source_position pos) {
                                  pos.colno += std::distance(this->state.src_it,
                                                             tmp_src_it);
                                  return pos;
                              });
    }
    if (!is_eof(++tmp_src_it) && (*tmp_src_it == '+' || *tmp_src_it == '-')) {
        // consume the sign
        ++tmp_src_it;
    }
    digit_or_error(tmp_src_it, "expected digit after exponent 'e' sign");
    consume_digits(tmp_src_it);
    return finalize_token(lexeme_t::floatconst, tmp_src_it,
                          [this, tmp_src_it](source_position pos) {
                              pos.colno +=
                                  std::distance(this->state.src_it, tmp_src_it);
                              return pos;
                          });
}
auto Lexer::match_int_literal() -> std::optional<token> {
    if (is_eof(this->state.src_it) || !std::isdigit(*this->state.src_it)) {
        return std::nullopt;
    }
    auto tmp_src_it = this->state.src_it;
    consume_digits(tmp_src_it);
    if (!is_eof(tmp_src_it) && *tmp_src_it == '.') {
        return std::nullopt;
    }
    return finalize_token(lexeme_t::intconst, tmp_src_it,
                          [this, tmp_src_it](source_position pos) {
                              pos.colno +=
                                  std::distance(this->state.src_it, tmp_src_it);
                              return pos;
                          });
}
auto Lexer::match_char_literal() -> std::optional<token> {
    if (is_eof(this->state.src_it) || *this->state.src_it != '\'') {
        return std::nullopt;
    }
    auto tmp_src_it = this->state.src_it;
    auto chars_to_eof = std::distance(tmp_src_it, this->src.end());
    const auto is_common = [this](char c) {
        static const std::array non_common_chars{'\n', '\r', '\t',
                                                 '\'', '\\', '"'};
        if (std::find(non_common_chars.begin(), non_common_chars.end(), c) ==
            non_common_chars.end()) {
            return true;
        }
        throw parse_error{this->state.cur_pos, "invalid character literal"};
    };
    if (chars_to_eof >= 3 && *(tmp_src_it + 2) == '\'' &&
        is_common(*(tmp_src_it + 1))) {
        return finalize_token(lexeme_t::charconst, tmp_src_it + 3,
                              [](source_position pos) {
                                  pos.colno += 3;
                                  return pos;
                              });
    }
    const auto can_be_escaped = [this](char c) {
        static const std::array escapable_chars{'n', 'r', 't', '\'', '\\', '"'};
        if (std::find(escapable_chars.begin(), escapable_chars.end(), c) !=
            escapable_chars.end()) {
            return true;
        }
        throw parse_error{
            this->state.cur_pos,
            "invalid escape sequence",
        };
    };
    if (chars_to_eof >= 4 && *(tmp_src_it + 3) == '\'' &&
        (*(tmp_src_it + 1) == '\\') &&
        can_be_escaped(*(tmp_src_it + 2))) { // single char escape sequence
        return finalize_token(lexeme_t::charconst, tmp_src_it + 4,
                              [](source_position pos) {
                                  pos.colno += 4;
                                  return pos;
                              });
    }
    const auto is_hex = [this](char c) {
        if (std::isdigit(c) || (c >= 'a' && c <= 'f')) {
            return true;
        }
        throw parse_error{this->state.cur_pos, "invalid hex escape sequence"};
    };
    if (chars_to_eof >= 6 && *(tmp_src_it + 5) == '\'' &&
        (*(tmp_src_it + 1) == '\\') && (*(tmp_src_it + 2) == 'x') &&
        is_hex(*(tmp_src_it + 3)) &&
        is_hex(*(tmp_src_it + 4))) { // hex escape sequence
        return finalize_token(lexeme_t::charconst, tmp_src_it + 6,
                              [](source_position pos) {
                                  pos.colno += 6;
                                  return pos;
                              });
    }
    throw parse_error{this->state.cur_pos, "invalid character literal"};
}
auto Lexer::match_string_literal() -> std::optional<token> {
    if (is_eof(this->state.src_it) || *this->state.src_it != '"') {
        return std::nullopt;
    }
    auto tmp_src_it = this->state.src_it;
    while (!is_eof(++tmp_src_it) && (*tmp_src_it != '"')) {
        if (*tmp_src_it == '\n') {
            auto err_pos = this->state.cur_pos;
            err_pos.colno += std::distance(this->state.src_it, tmp_src_it);
            throw parse_error{err_pos,
                              "string literal cannot span multiple lines"};
        }
        if (*tmp_src_it == '\\') {
            ++tmp_src_it; // consume escaped character
            if (!is_eof(tmp_src_it) && *tmp_src_it == '\n') {
                // new-line is not escaped
                --tmp_src_it;
            }
        }
    }
    if (is_eof(tmp_src_it)) {
        auto err_pos = this->state.cur_pos;
        err_pos.colno += std::distance(this->state.src_it, tmp_src_it);
        throw parse_error{err_pos, "eof in string literal"};
    }
    return finalize_token(lexeme_t::stringliteral, ++tmp_src_it,
                          [this, tmp_src_it](source_position pos) {
                              pos.colno +=
                                  std::distance(this->state.src_it, tmp_src_it);
                              return pos;
                          });
}
auto Lexer::match_multi_char_symop() -> std::optional<token> {
    static constexpr std::array lexemes{
        lexeme_t::DASHGREATER,  lexeme_t::PLUSDOT,  lexeme_t::MINUSDOT,
        lexeme_t::STARDOT,      lexeme_t::SLASHDOT, lexeme_t::DBLSTAR,
        lexeme_t::DBLAMPERSAND, lexeme_t::DBLBAR,   lexeme_t::LTGT,
        lexeme_t::LEQ,          lexeme_t::GEQ,      lexeme_t::DBLEQ,
        lexeme_t::EXCLAMEQ,     lexeme_t::COLONEQ};
    for (const auto lexeme : lexemes) {
        if (auto tok = match_with_str(as<std::string_view>(lexeme), lexeme)) {
            return tok;
        }
    }
    return std::nullopt;
}
auto Lexer::match_single_char_sep_or_symop() -> std::optional<token> {
    static constexpr std::array lexemes{
        lexeme_t::EQ,       lexeme_t::BAR,       lexeme_t::PLUS,
        lexeme_t::MINUS,    lexeme_t::STAR,      lexeme_t::SLASH,
        lexeme_t::EXCLAM,   lexeme_t::SEMICOLON, lexeme_t::LT,
        lexeme_t::GT,       lexeme_t::LPAREN,    lexeme_t::RPAREN,
        lexeme_t::LBRACKET, lexeme_t::RBRACKET,  lexeme_t::COMMA,
        lexeme_t::COLON};
    for (const auto lexeme : lexemes) {
        if (auto tok = match_with_str(as<std::string_view>(lexeme), lexeme)) {
            return tok;
        }
    }
    return std::nullopt;
}
auto Lexer::match_unmatched() -> std::optional<token> {
    // TODO: implement some kind of error tolerance e.g. eat up until whitespace
    return finalize_token(lexeme_t::UNMATCHED, this->src.end(),
                          [this](source_position pos) {
                              return this->src.it_to_src_pos(this->src.end());
                          });
}

auto Lexer::match_any_id(lexeme_t tok_type) -> std::optional<token> {
    auto tmp_src_it = this->state.src_it;
    while (!is_eof(tmp_src_it) &&
           (std::isalnum(*tmp_src_it) || *tmp_src_it == '_')) {
        ++tmp_src_it;
    }
    if (tmp_src_it == this->state.src_it) { // means it found no alnums
        return std::nullopt;
    }
    // fmt::print("std::distance(this->src.begin(), tmp_src_it) = {}\n",
    //            std::distance(this->src.begin(), tmp_src_it));
    return finalize_token(
        tok_type, tmp_src_it, [this, tmp_src_it](source_position pos) {
            pos.colno += std::distance(this->state.src_it, tmp_src_it);
            return pos;
        });
}
auto Lexer::match_with_str(std::string_view to_match, lexeme_t tok_type)
    -> std::optional<token> {
    auto temp_src_it = this->state.src_it;
    auto to_match_it = to_match.begin();

    for (; !is_eof(temp_src_it) && to_match_it != to_match.end();
         ++temp_src_it, ++to_match_it) {
        if (*temp_src_it != *to_match_it) {
            return std::nullopt;
        }
    }
    if (auto char_after_match = *temp_src_it;
        to_match_it != to_match.end() &&
        (std::isalnum(char_after_match) || char_after_match == '_')) {
        return std::nullopt;
    }
    // auto tok_end_pos =
    // this->state.cur_pos.pos_increment_col(to_match.size());
    return finalize_token(tok_type, temp_src_it,
                          [&to_match](source_position pos) {
                              pos.colno += to_match.size();
                              return pos;
                          });
}

auto Lexer::is_eof(Source::const_iterator it) -> bool {
    return it == this->src.end();
}
// TODO: this can be a lambda and not a member function
auto Lexer::digit_or_error(Source::const_iterator it, std::string_view err_msg)
    -> void {
    if (it == this->src.end() || !std::isdigit(*it)) {
        auto err_pos = this->state.cur_pos;
        err_pos.colno += std::distance(this->state.src_it, it);
        throw parse_error{err_pos, err_msg};
    }
}
auto Lexer::consume_digits(Source::const_iterator &it) -> void {
    while (!is_eof(++it) && std::isdigit(*it)) {
    }
}
