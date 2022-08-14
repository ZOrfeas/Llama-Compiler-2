#pragma once

#include <string>
#include <vector>
#include <fstream>
#include <iostream>
#include <regex>


struct position {
    int line; 
    int column;
};

enum token_kind 
{
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

struct token 
{
    token_kind t;
    std::string name;
    position start, end;
};
