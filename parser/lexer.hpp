#ifndef PARSE_LEXER_HPP
#define PARSE_LEXER_HPP

#include "common.hpp"
#include <iterator>
#include <string_view>
#include <sys/_types/_ucontext.h>
#include <unordered_set>
#include <vector>

// error-handling notes:
//   exceptions will be thrown, and the top level of the Lexer will be
//   wrapped in a try-catch.
//   depending on Lexer paramereters, the Lexer can try to recover and store the
//   error or throw it higher up the stack.

namespace lla {

    class Lexer {
    public:
        Lexer(std::string_view); // this should run our barebones "preprocessor"
                                 // and load all source files into memory
        auto lex()
            -> int; // this should fill the token vector and handle errors
        [[nodiscard]] auto get_tokens() const -> std::vector<token> &;

    private:
    public:
        class Source {
        private:
            struct Source_file;

        public:
            Source(std::string_view);
            class const_iterator {
            public:
                using iterator_category = std::bidirectional_iterator_tag;
                using difference_type = std::ptrdiff_t; // not certainly correct
                using value_type = char;
                using pointer = char *;
                using reference = char &;

                [[nodiscard]] auto get_cur_filename() const -> std::string_view;
                [[nodiscard]] auto is_end() const -> bool;
                [[nodiscard]] auto is_start() const -> bool;
                auto operator*() -> char;
                auto operator++() -> const_iterator &;
                auto operator++(int) -> const_iterator;
                auto operator--() -> const_iterator &;
                auto operator--(int) -> const_iterator;
                friend auto operator==(const_iterator const &a,
                                       const_iterator const &b) -> bool {
                    return a.cur_file_it == b.cur_file_it && a.it == b.it;
                };
                friend auto operator!=(const_iterator const &a,
                                       const_iterator const &b) -> bool {
                    return !(a == b);
                };

            private:
                friend Source;
                const_iterator(Source const &);
                const_iterator(const_iterator const &) = default;

                const Source &src;
                std::vector<Source_file>::const_iterator cur_file_it;
                std::vector<char>::const_iterator it;
                auto advance_it() -> void;
                auto rewind_it() -> void;
            };
            [[nodiscard]] auto create_reader() const -> const_iterator;
            [[nodiscard]] auto begin() const -> const_iterator;
            [[nodiscard]] auto end() const -> const_iterator;

        private:
            struct Source_file {
                Source_file(std::string_view);
                std::string filename;
                std::vector<char> text;
                [[nodiscard]] auto cbegin() const
                    -> std::vector<char>::const_iterator;
                [[nodiscard]] auto cend() const
                    -> std::vector<char>::const_iterator;
                //! Note: Maybe you want const iterators as well, maybe not?
            };

            std::unordered_set<std::string_view> filenames; // for import-cycles
            std::vector<Source_file> text; // to be iterated over

            auto preprocess(Source_file) -> std::vector<Source_file>;
        };

        Source src;
        std::vector<token> tokens; // each token object is only a
                                   // string_view so no duplication happens
        std::vector<parse_error> errors;

        auto read_one_token() -> token;
    };
} // namespace lla

#endif // PARSE_LEXER_HPP