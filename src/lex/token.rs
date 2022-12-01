pub struct Token {
    pub kind: TokenKind,
    pub original: Vec<u8>,
    pub pos: Position,
}
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenKind {
    EOF,
    UNMATCHED,
    COMMENT,

    // Keywords
    And,
    Array,
    Begin,
    Bool,
    Char,
    Delete,
    Dim,
    Do,
    Done,
    Downto,
    Else,
    End,
    False,
    Float,
    For,
    If,
    In,
    Int,
    Let,
    Match,
    Mod,
    Mutable,
    Not,
    Of,
    Rec,
    Ref,
    Then,
    To,
    True,
    Type,
    Unit,
    While,
    With,

    /// Identifiers
    IdUpper(String),
    IdLower(String),

    /// Literals
    IntLiteral(i32),
    FloatLiteral(f32),
    CharLiteral(u8),
    StringLiteral(String),

    /// Symbols
    Arrow,
    PlusDot,
    MinusDot,
    StarDot,
    SlashDot,
    DblStar,
    DlbAmpersand,
    DblBar,
    LtGt,
    LEq,
    GEq,
    DblEq,
    ExclamEq,
    ColonEq,
    Semicolon,
    Eq,
    Gt,
    Lt,
    Plus,
    Minus,
    Star,
    Slash,
    Colon,
    Comma,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Bar,
    Exclam,
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
            Self::DlbAmpersand => write!(f, "&&"),
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
