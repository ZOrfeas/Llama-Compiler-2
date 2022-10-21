
#include <cstdint>
#include <compare>
#include <string>
#include <string_view>

namespace lla {
    using lineno_t = std::uint32_t;
    using colno_t = std::uint32_t;

    struct source_position {
        lineno_t lineno;
        colno_t colno;
        auto operator<=>(const source_position&) const = default;
        [[nodiscard]] auto to_sting() const -> std::string {
            return "(" + std::to_string(lineno) + "," + std::to_string(colno) + ")";
        }
    };

    struct parse_error {
        source_position pos;
        std::string msg;
        bool internal = false;
    };

    enum class lexeme_t : std::uint8_t {
        COMMENT, UNMATCHED, STOP, AND, ARRAY, BEGIN, BOOL,
        CHAR, DELETE, DIM, DO, DONE, DOWNTO, ELSE, END,
        FALSE, FLOAT, FOR, IF, IN, INT, LET, MATCH, MOD,
        MUTABLE, NEW, NOT, OF, REC, REF,THEN,TO,TRUE, TYPE,
        UNIT, WHILE, WITH, 
        
        idlower, idupper, intconst, floatconst, charconst, stringliteral,

        DASHGREATER, PLUSDOT, MINUSDOT, STARDOT, SLASHDOT, DBLSTAR,
        DBLAMPERSAND, DBLBAR, LTGT, LEQ, GEQ, DBLEQ, EXCLAMEQ,
        COLONEQ, SEMICOLON, EQ, GT, LT, PLUS, MINUS, STAR,
        SLASH, COLON, COMMA, LBRACKET, RBRACKET, LPAREN, RPAREN,
        BAR, EXCLAM
    };
    template<typename T> requires std::is_same_v<T, std::string>
    auto as(lexeme_t l) -> std::string {
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
        return token_strings[static_cast<int>(l)];
    }

    struct token {
        lexeme_t lexeme_type;
        source_position src_start, src_end;
        std::string_view value; // non-owning view to the source code location
        [[nodiscard]] auto to_string() const -> std::string {
            return "(" + as<std::string>(lexeme_type) +
                   ": " + std::string(value) + ")";
        }
    };
} // namespace lla