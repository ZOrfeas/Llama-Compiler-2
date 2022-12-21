use std::rc::Rc;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub original: Vec<u8>,
    pub from: Position,
    pub to: Position,
}
impl Token {
    pub fn new(kind: TokenKind, original: Vec<u8>, from: Position, to: Position) -> Self {
        Token {
            kind,
            original,
            from,
            to,
        }
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` from {} to {}\n", self.kind, self.from, self.to)
    }
}
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub filename: Rc<String>,
}
impl Position {
    pub fn new(line: usize, column: usize, filename: Rc<String>) -> Self {
        Self {
            line,
            column,
            filename,
        }
    }
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{} in {})", self.line, self.column, self.filename)
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenKind {
    EOF, UNMATCHED, COMMENT,

    // Keywords
    And, Array, Begin, Bool, Char,
    Delete, Dim, Do, Done, Downto,
    Else, End, False, Float, For,
    If, In, Int, Let, Match, Mod,
    Mutable, New, Not, Of, Rec, Ref,
    Then, To, True, Type, Unit,
    While, With,

    // Identifiers
    IdUpper(String),
    IdLower(String),

    // Literals
    IntLiteral(i32),
    FloatLiteral(f64),
    CharLiteral(u8),
    StringLiteral(String),

    // Multi-char symbols
    Arrow, PlusDot, MinusDot, StarDot, SlashDot,
    DblStar, DblAmpersand, DblBar, LtGt, LEq,
    GEq, DblEq, ExclamEq, ColonEq,
    
    // Single-char symbols
    Semicolon, Eq, Gt, Lt, Plus, Minus, Star, Slash,
    Colon, Comma, LBracket, RBracket, LParen, RParen,
    Bar, Exclam,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            s if s >= &Self::And && s <= &Self::With => {
                write!(f, "{}", format!("{:?}", self).to_ascii_lowercase())
            }
            Self::Arrow => write!(f, "->"),
            Self::PlusDot => write!(f, "+."),
            Self::MinusDot => write!(f, "-."),
            Self::StarDot => write!(f, "*."),
            Self::SlashDot => write!(f, "/."),
            Self::DblStar => write!(f, "**"),
            Self::DblAmpersand => write!(f, "&&"),
            Self::DblBar => write!(f, "||"),
            Self::LtGt => write!(f, "<>"),
            Self::LEq => write!(f, "<="),
            Self::GEq => write!(f, ">="),
            Self::DblEq => write!(f, "=="),
            Self::ExclamEq => write!(f, "!="),
            Self::ColonEq => write!(f, ":="),
            Self::Semicolon => write!(f, ";"),
            Self::Eq => write!(f, "="),
            Self::Gt => write!(f, ">"),
            Self::Lt => write!(f, "<"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Colon => write!(f, ":"),
            Self::Comma => write!(f, ","),
            Self::LBracket => write!(f, "["),
            Self::RBracket => write!(f, "]"),
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::Bar => write!(f, "|"),
            Self::Exclam => write!(f, "!"),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialize_one_of_each() {
        assert_eq!(TokenKind::EOF.to_string(), "EOF");
        assert_eq!(TokenKind::Begin.to_string(), "begin");
        assert_eq!(TokenKind::IntLiteral(5i32).to_string(), "IntLiteral(5)");
        assert_eq!(
            TokenKind::StringLiteral("Kati".to_string()).to_string(),
            "StringLiteral(\"Kati\")"
        );
        assert_eq!(TokenKind::Arrow.to_string(), "->");
    }
}
