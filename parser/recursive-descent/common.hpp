#ifndef PARSING_COMMON_HPP
#define PARSING_COMMON_HPP

#include <array>
#include <fstream>
#include <iostream>
#include <string>
#include <array>
#include <vector>

#define TAB_SIZE 8

struct position {
    unsigned long line;
    unsigned long column;
};

enum class token_kind {
    COMMENT,
    UNMATCHED,
    STOP,
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

struct token {
    token_kind t;
    std::string value;
    position start, end;
};

// inlined functions with static variables
// usually use the same storage for all instances after linking
inline auto token_kind_string(token_kind t) -> std::string {
    static const std::array token_strings = {
        "COMMENT",      "UNMATCHED", "STOP",
        "AND",          "ARRAY",     "BEGIN",
        "BOOL",         "CHAR",      "DELETE",
        "DIM",          "DO",        "DONE",
        "DOWNTO",       "ELSE",      "END",
        "FALSE",        "FLOAT",     "FOR",
        "IF",           "IN",        "INT",
        "LET",          "MATCH",     "MOD",
        "MUTABLE",      "NEW",       "NOT",
        "OF",           "REC",       "REF",
        "THEN",         "TO",        "TRUE",
        "TYPE",         "UNIT",      "WHILE",
        "WITH",

        "idlower",      "idupper",   "intconst",
        "floatconst",   "charconst", "stringliteral",

        "DASHGREATER",  "PLUSDOT",   "MINUSDOT",
        "STARDOT",      "SLASHDOT",  "DBLSTAR",
        "DBLAMPERSAND", "DBLBAR",    "LTGT",
        "LEQ",          "GEQ",       "DBLEQ",
        "EXCLAMEQ",     "COLONEQ",   "SEMICOLON",
        "EQ",           "GT",        "LT",
        "PLUS",         "MINUS",     "STAR",
        "SLASH",        "COLON",     "COMMA",
        "LBRACKET",     "RBRACKET",  "LPAREN",
        "RPAREN",       "BAR",       "EXCLAM"};
    return token_strings[static_cast<int>(t)];
}

#endif // PARSING_COMMON_HPP