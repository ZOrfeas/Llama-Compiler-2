#ifndef PARSE_COMMON_HPP
#define PARSE_COMMON_HPP

#include "fmt/core.h"
#include <concepts>
#include <cstdint>
#include <exception>
#include <string>
#include <string_view>
#include <utility>

namespace lla::parse {
    using lineno_t = std::uint32_t;
    using colno_t = std::uint32_t;
    using FilenamePtr = std::shared_ptr<std::string>;

    struct source_position {
        lineno_t lineno; // 1-indexed
        colno_t colno;   // 1-indexed
        FilenamePtr filename;
        [[nodiscard]] auto to_string() const -> std::string {
            return fmt::format("{}:{}:{}", *filename, lineno, colno);
        }
    };

    enum class lexeme_t : std::uint8_t {
        COMMENT,
        UNMATCHED,
        end_of_file,
        AND,
        ARRAY,
        BEGIN,
        BOOL,
        CHAR,
        DELETE,
        DIM,
        DO,
        DONE,
        DOWNTO,
        ELSE,
        END,
        FALSE,
        FLOAT,
        FOR,
        IF,
        IN,
        INT,
        LET,
        MATCH,
        MOD,
        MUTABLE,
        NEW,
        NOT,
        OF,
        REC,
        REF,
        THEN,
        TO,
        TRUE,
        TYPE,
        UNIT,
        WHILE,
        WITH,

        idlower,
        idupper,
        intconst,
        floatconst,
        charconst,
        stringliteral,

        DASHGREATER,
        PLUSDOT,
        MINUSDOT,
        STARDOT,
        SLASHDOT,
        DBLSTAR,
        DBLAMPERSAND,
        DBLBAR,
        LTGT,
        LEQ,
        GEQ,
        DBLEQ,
        EXCLAMEQ,
        COLONEQ,
        SEMICOLON,
        EQ,
        GT,
        LT,
        PLUS,
        MINUS,
        STAR,
        SLASH,
        COLON,
        COMMA,
        LBRACKET,
        RBRACKET,
        LPAREN,
        RPAREN,
        BAR,
        EXCLAM
    };

    template <typename T>
        requires std::is_same_v<T, std::string_view>
    inline static auto as(lexeme_t l) -> std::string_view {
        static std::array token_strings = {
            "comment",    "unmatched", "EOF",
            "and",        "array",     "begin",
            "bool",       "char",      "delete",
            "dim",        "do",        "done",
            "downto",     "else",      "end",
            "false",      "float",     "for",
            "if",         "in",        "int",
            "let",        "match",     "mod",
            "mutable",    "new",       "not",
            "of",         "rec",       "ref",
            "then",       "to",        "true",
            "type",       "unit",      "while",
            "with",

            "idlower",    "idupper",   "intconst",
            "floatconst", "charconst", "stringliteral",

            "->",         "+.",        "-.",
            "*.",         "/.",        "**",
            "&&",         "||",        "<>",
            "<=",         ">=",        "==",
            "!=",         ":=",        ";",
            "=",          ">",         "<",
            "+",          "-",         "*",
            "/",          ":",         ",",
            "[",          "]",         "(",
            ")",          "|",         "!"};
        return token_strings[static_cast<int>(l)];
    }
    template <typename T>
        requires std::is_same_v<T, std::string>
    inline static auto as(lexeme_t l) -> std::string {
        return std::string(as<std::string_view>(l));
    }

    struct token {
        lexeme_t type;
        source_position src_start, src_end;
        std::string value; // non-owning view to the source code location
        [[nodiscard]] auto to_string() const -> std::string {
            return fmt::format("({}: `{}`)", as<std::string_view>(type), value);
        }
    };
} // namespace lla::parse

#endif // PARSE_COMMON_HPP