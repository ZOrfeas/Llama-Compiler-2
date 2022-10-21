
#include <cstdint>
#include <compare>
#include <string>

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

    enum class lexeme : std::uint8_t {
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
} // namespace lla