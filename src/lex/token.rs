use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub original: Vec<u8>,
    pub value: TokenValue,
    pub from: Position,
    pub to: Position,
}
impl Token {
    pub fn new(kind: TokenKind, original: Vec<u8>, from: Position, to: Position) -> Self {
        Token {
            kind,
            original,
            value: TokenValue::None,
            from,
            to,
        }
    }
    pub fn new_with_value(
        kind: TokenKind,
        original: Vec<u8>,
        value: TokenValue,
        from: Position,
        to: Position,
    ) -> Self {
        Token {
            kind,
            original,
            value,
            from,
            to,
        }
    }
    pub fn extract_string_value(self) -> String {
        match self.value {
            TokenValue::String(s) => s,
            _ => panic!("TokenValue is not a string"),
        }
    }
    pub fn extract_int_value(self) -> i32 {
        match self.value {
            TokenValue::Int(i) => i,
            _ => panic!("TokenValue is not an int"),
        }
    }
    pub fn extract_float_value(self) -> f64 {
        match self.value {
            TokenValue::Float(f) => f,
            _ => panic!("TokenValue is not a float"),
        }
    }
    pub fn extract_char_value(self) -> u8 {
        match self.value {
            TokenValue::Char(c) => c,
            _ => panic!("TokenValue is not a char"),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TokenValue {
    Int(i32),
    Float(f64),
    Char(u8),
    String(String),
    None,
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
    IdUpper,
    IdLower,

    // Literals
    IntLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,

    // Multi-char symbols
    Arrow, PlusDot, MinusDot, StarDot, SlashDot,
    DblStar, DblAmpersand, DblBar, LtGt, LEq,
    GEq, DblEq, ExclamEq, ColonEq,
    
    // Single-char symbols
    Semicolon, Eq, Gt, Lt, Plus, Minus, Star, Slash,
    Colon, Comma, LBracket, RBracket, LParen, RParen,
    Bar, Exclam,
}
#[rustfmt::skip]
pub const KEYWORDS: [TokenKind; 34] = [
    TokenKind::And, TokenKind::Array, TokenKind::Begin, TokenKind::Bool,
    TokenKind::Char, TokenKind::Delete, TokenKind::Dim, TokenKind::Do,
    TokenKind::Done, TokenKind::Downto, TokenKind::Else, TokenKind::End,
    TokenKind::False, TokenKind::Float, TokenKind::For, TokenKind::If,
    TokenKind::Int, TokenKind::In, TokenKind::Let, TokenKind::Match,
    TokenKind::Mod, TokenKind::Mutable, TokenKind::New, TokenKind::Not,
    TokenKind::Of, TokenKind::Rec, TokenKind::Ref, TokenKind::Then,
    TokenKind::To, TokenKind::True, TokenKind::Type, TokenKind::Unit,
    TokenKind::While, TokenKind::With,
];
#[rustfmt::skip]
pub const MULTI_CHAR_SYMBOLS: [TokenKind; 14] = [
    TokenKind::Arrow, TokenKind::PlusDot, TokenKind::MinusDot, 
    TokenKind::StarDot, TokenKind::SlashDot, TokenKind::DblStar,
    TokenKind::DblAmpersand, TokenKind::DblBar, TokenKind::LtGt,
    TokenKind::LEq, TokenKind::GEq, TokenKind::DblEq, TokenKind::ExclamEq,
    TokenKind::ColonEq,
];
#[rustfmt::skip]
pub const SINGLE_CHAR_SYMBOLS: [TokenKind; 16]= [
    TokenKind::Semicolon, TokenKind::Eq, TokenKind::Gt, TokenKind::Lt,
    TokenKind::Plus, TokenKind::Minus, TokenKind::Star, TokenKind::Slash,
    TokenKind::Colon, TokenKind::Comma, TokenKind::LBracket, TokenKind::RBracket,
    TokenKind::LParen, TokenKind::RParen, TokenKind::Bar, TokenKind::Exclam,
];

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
        assert_eq!(TokenKind::IntLiteral.to_string(), "IntLiteral");
        assert_eq!(TokenKind::StringLiteral.to_string(), "StringLiteral");
        assert_eq!(TokenKind::Arrow.to_string(), "->");
    }
}
