pub mod token;

use std::{
    rc::Rc,
};

use log::error;

use crate::scan;

use self::token::{Position, Token, TokenKind};

pub struct Lexer<S: Iterator<Item = scan::Line>> {
    scanner: S,
    exit_on_error: bool,

    cursor: usize, // cur_colno - 1
    cur_lineno: usize,
    cur_filename: Rc<String>,
    cur_line: Option<Vec<u8>>,
    comment_nesting: usize,

    is_done: bool,
}
// TODO: Consider a State-struct to hold the lexer's state grouped together.
// TODO:     cur_line, cursor, pos, comment_nesting, etc.
impl<S: Iterator<Item = scan::Line>> Lexer<S> {
    pub fn new(scanner: S) -> Self {
        Lexer {
            scanner,
            exit_on_error: true,

            cursor: 0,
            cur_lineno: 0,
            cur_filename: Rc::new(String::new()), 
            cur_line: None,
            comment_nesting: 0,

            is_done: false,
        }
    }
    fn get_cur_line(&self, caller_msg: &'static str) -> &Vec<u8> {
        self.cur_line
            .as_ref()
            .expect(caller_msg)
    }

    fn next_token(&mut self) -> LexResult<Option<Token>> {
        if self.is_done {
            return Ok(None);
        };
        if self.cur_line.is_none() {
            self.next_line();
        }
        let matchers = [
            Self::match_eof,
            Self::match_multi_line_comment,
            Self::match_single_line_comment,
            Self::match_lowercase_identifier,
            Self::match_uppercase_identifier,
            Self::match_float_literal,
            Self::match_integer_literal,
            Self::match_character_literal,
            Self::match_string_literal,
            Self::match_multi_char_symop,
            Self::match_single_char_symop_or_sep,
            Self::match_unmatched,
        ];
        self.eat_whitespace();
        for matcher in matchers.iter() {
            if let Some(token) = matcher(self)? {
                match &token.kind {
                    TokenKind::EOF => self.is_done = true,
                    TokenKind::IdLower(id) => if let Some(reserved) = Self::reserved_word_from_id(id) {
                        return Ok(Some(Token::new(
                            reserved, token.original, token.from, token.to
                        )))
                    }
                    _ => {}
                }
                return Ok(Some(token));
            }
        }
        unreachable!("unmatched cases handled above, this should be unreachable")
    }
    fn eat_whitespace(&mut self) {
        while let Some(line) = self.cur_line.as_ref() {
            if self.cursor >= line.len() {
                self.next_line();
                continue;
            }
            let first_non_whitespace = line[self.cursor..]
                .iter()
                .enumerate()
                .find(|(_, &c)| !(c as char).is_ascii_whitespace());
            if let Some((i, _)) = first_non_whitespace {
                self.cursor += i;
                return;
            }
            self.next_line();
        }
    }
    fn match_eof(&mut self) -> LexResult<Option<Token>> {
        if self.cur_line.is_none() {
            Ok(Some(Token::new(
                TokenKind::EOF,
                vec![],
                self.make_position(self.cursor),
                self.make_position(self.cursor),
            )))
        } else {
            Ok(None)
        }
    }
    fn match_multi_line_comment(&mut self) -> LexResult<Option<Token>> {
        if !self.get_cur_line("match_multi_line_comment")[self.cursor..].starts_with(b"(*") {
            return Ok(None);
        }
        self.comment_nesting += 1;
        let from = self.make_position(self.cursor);
        self.cursor += 2;
        let mut comment_contents = vec![b'(', b'*'];
        let mut save_contents = |line: &Vec<u8>, from: usize, i: usize| {
            comment_contents.extend_from_slice(&line[from..from + i])
        };
        while let Some(line) = self.cur_line.as_ref() {
            if self.cursor >= line.len() {
                self.next_line();
                continue;
            }
            let next_comment_open_or_close =
                line[self.cursor..].iter().enumerate().find(|(i, &c)| {
                    (c == b'*' && line.get(self.cursor + i + 1) == Some(&b')'))
                        || (c == b'(' && line.get(self.cursor + i + 1) == Some(&b'*'))
                });
            match next_comment_open_or_close {
                Some((i, &b'*')) => {
                    // comment close
                    self.comment_nesting -= 1;
                    save_contents(line, self.cursor, i + 2);
                    self.cursor += i + 2;
                    if self.comment_nesting == 0 {
                        return Ok(Some(Token::new(
                            TokenKind::COMMENT,
                            comment_contents,
                            from,
                            self.make_position(self.cursor),
                        )));
                    }
                }
                Some((i, &b'(')) => {
                    // comment open
                    self.comment_nesting += 1;
                    save_contents(line, self.cursor, i + 2);
                    self.cursor += i + 2;
                }
                None => {
                    // end of line
                    save_contents(line, self.cursor, line.len() - self.cursor);
                    self.next_line();
                }
                _ => unreachable!("unreachable because of the find above"),
            }
        }
        Err(LexErr::UnterminatedComment(from))
    }
    fn match_single_line_comment(&mut self) -> LexResult<Option<Token>> {
        let line = self.get_cur_line("match_single_line_comment");
        if line[self.cursor..].starts_with(b"--") {
            let from = self.make_position(self.cursor);
            // self.cursor = line.len() - 1;
            let to = self.make_position(line.len());
            let retval = Token::new(TokenKind::COMMENT, line[self.cursor..].to_vec(), from, to);
            self.next_line();
            Ok(Some(retval))
        } else {
            Ok(None)
        }
    }
    #[rustfmt::skip]
    fn reserved_word_from_id(id: &String) -> Option<TokenKind> {
        const LEXEMES: [TokenKind; 34] = [
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
        LEXEMES.iter().find(|lexeme| lexeme.to_string() == *id).cloned()
    }
    fn match_lowercase_identifier(&mut self) -> LexResult<Option<Token>> {
        self.match_any_identifier(|c| c.is_ascii_lowercase(), TokenKind::IdLower)
    }
    fn match_uppercase_identifier(&mut self) -> LexResult<Option<Token>> {
        self.match_any_identifier(|c| c.is_ascii_uppercase(), TokenKind::IdUpper)
    }
    fn match_float_literal(&mut self) -> LexResult<Option<Token>> {
        let line = self.get_cur_line("match_float_literal");
        let integral_part_digits = self.count_digits(line, self.cursor);

        let mut tmp_cursor = self.cursor + integral_part_digits;
        if integral_part_digits == 0 || line.get(tmp_cursor) != Some(&b'.') {
            return Ok(None);
        }
        tmp_cursor += 1;
        let fractional_part_digits = self.count_digits(line, tmp_cursor);
        if fractional_part_digits == 0 {
            return Ok(None);
        }
        let from = self.make_position(self.cursor);
        let contents = line[self.cursor..tmp_cursor + fractional_part_digits].to_vec();
        self.cursor = tmp_cursor + fractional_part_digits;
        let number = String::from_utf8(contents.clone())
            .map_err(|e| LexErr::FromUtf8Error(from.clone(), e.to_string()))?
            .parse::<f64>()
            .map_err(|e| LexErr::ParseFloatError(from.clone(), e.to_string()))?;
        Ok(Some(Token::new(
            TokenKind::FloatLiteral(number),
            contents,
            from,
            self.make_position(self.cursor),
        )))
    }
    fn match_integer_literal(&mut self) -> LexResult<Option<Token>> {
        let line = self.get_cur_line("match_integer_literal");
        let digits = self.count_digits(line, self.cursor);
        if digits == 0 {
            return Ok(None);
        }
        let from = self.make_position(self.cursor);
        let contents = line[self.cursor..self.cursor + digits].to_vec();
        self.cursor += digits;
        let number = String::from_utf8(contents.clone())
            .map_err(|e| LexErr::FromUtf8Error(from.clone(), e.to_string()))?
            .parse::<i32>()
            .map_err(|e| LexErr::ParseIntError(from.clone(), e.to_string()))?;
        Ok(Some(Token::new(
            TokenKind::IntLiteral(number),
            contents,
            from,
            self.make_position(self.cursor),
        )))
    }
    fn match_character_literal(&mut self) -> LexResult<Option<Token>> {
        let line = self.get_cur_line("match_character_literal");
        if line.get(self.cursor) != Some(&b'\'') {
            return Ok(None);
        }
        let from = self.make_position(self.cursor);
        let finalize_literal = |c: u8, cnt: usize| {
            let original = line[self.cursor..=self.cursor + cnt].to_vec();
            (
                self.cursor + cnt + 1,
                Ok(Some(Token::new(
                    TokenKind::CharLiteral(c),
                    original,
                    from.clone(),
                    self.make_position(self.cursor),
                ))),
            )
        };
        match line[self.cursor + 1..]
            .iter()
            .enumerate()
            .find(|(i, &c)| i > &4 || (c == b'\'' && line[self.cursor + i] != b'\\'))
        {
            Some((0, _)) => Err(LexErr::InvalidCharLiteral(from, "empty character literal")),
            Some((1, _)) => {
                let c = line[self.cursor + 1];
                if ![b'\n', b'\r', b'\t', b'\'', b'"'].contains(&c) {
                    let (new_cursor, retval) = finalize_literal(c, 2);
                    self.cursor = new_cursor;
                    retval
                } else {
                    Err(LexErr::InvalidCharLiteral(
                        from,
                        "Only common and printable characters are allowed as character literals",
                    ))
                }
            }
            Some((2, _)) if line[self.cursor + 1] == b'\\' => {
                let c = line[self.cursor + 2];
                if let Some(escaped) = Self::parse_single_char_escape_seq(c) {
                    let (new_cursor, retval) = finalize_literal(escaped, 3);
                    self.cursor = new_cursor;
                    retval
                } else {
                    Err(LexErr::InvalidCharLiteral(from, "invalid escape sequence"))
                }
            }
            Some((4, _)) if line[self.cursor + 1] == b'\\' && line[self.cursor + 2] == b'x' => {
                let c1 = line[self.cursor + 3];
                let c2 = line[self.cursor + 4];
                if let Some(escaped) = Self::parse_hex_escape_seq(c1, c2) {
                    let (new_cursor, retval) = finalize_literal(escaped, 5);
                    self.cursor = new_cursor;
                    retval
                } else {
                    Err(LexErr::InvalidCharLiteral(from, "invalid escape sequence"))
                }
            }
            Some((_, _)) | None => Err(LexErr::InvalidCharLiteral(
                from,
                "invalid character literal",
            )),
        }
    }
    fn match_string_literal(&mut self) -> LexResult<Option<Token>> {
        let line = self.get_cur_line("match_string_literal");
        if line.get(self.cursor) != Some(&b'"') {
            return Ok(None);
        }
        let from = self.make_position(self.cursor);
        match line[self.cursor + 1..]
            .iter()
            .enumerate()
            .find(|(i, &c)| c == b'"' && line[self.cursor + i] != b'\\')
        {
            Some((0, _)) => Err(LexErr::InvalidStringLiteral(from, "empty string literal")),
            Some((i, _)) => {
                let orig_contents = line[self.cursor + 1..self.cursor + i + 1].to_vec();
                let contents = Self::parse_string_escape_seqs(&orig_contents).map_err(|e| {
                    LexErr::InvalidStringLiteral(
                        self.make_position(self.cursor + e + 1),
                        "invalid escape sequence",
                    )
                })?;
                self.cursor += i + 2;
                Ok(Some(Token::new(
                    TokenKind::StringLiteral(contents),
                    orig_contents,
                    from,
                    self.make_position(self.cursor),
                )))
            }
            None => Err(LexErr::InvalidStringLiteral(
                from,
                "unterminated string literal",
            )),
        }
    }
    #[rustfmt::skip]
    fn match_multi_char_symop(&mut self) -> LexResult<Option<Token>> {
        const LEXEMES: [TokenKind; 14] = [
            TokenKind::Arrow, TokenKind::PlusDot, TokenKind::MinusDot, 
            TokenKind::StarDot, TokenKind::SlashDot, TokenKind::DblStar,
            TokenKind::DblAmpersand, TokenKind::DblBar, TokenKind::LtGt,
            TokenKind::LEq, TokenKind::GEq, TokenKind::DblEq, TokenKind::ExclamEq,
            TokenKind::ColonEq,
        ];
        self.match_lexemes(&LEXEMES)
    }
    #[rustfmt::skip]
    fn match_single_char_symop_or_sep(&mut self) -> LexResult<Option<Token>> {
        const LEXEMES: [TokenKind; 16]= [
            TokenKind::Semicolon, TokenKind::Eq, TokenKind::Gt, TokenKind::Lt,
            TokenKind::Plus, TokenKind::Minus, TokenKind::Star, TokenKind::Slash,
            TokenKind::Colon, TokenKind::Comma, TokenKind::LBracket, TokenKind::RBracket,
            TokenKind::LParen, TokenKind::RParen, TokenKind::Bar, TokenKind::Exclam,
        ];
        self.match_lexemes(&LEXEMES)
    }
    fn match_unmatched(&mut self) -> LexResult<Option<Token>> {
        let mut original: Vec<u8> = Vec::new();
        let from = self.make_position(self.cursor);
        let to = loop {
            let line = self.get_cur_line("match_unmatched");
            match line[self.cursor..].iter().enumerate().find(|(_, c)| {
                c.is_ascii_whitespace()
            }) {
                Some((i, _)) => {
                    original.extend_from_slice(&line[self.cursor..self.cursor + i]);
                    self.cursor += i;
                    break self.make_position(self.cursor);
                },
                None => {
                    original.extend_from_slice(&line[self.cursor..]);
                    let line_len = line.len(); // needed to mutably borrow self below
                    self.next_line();
                    if self.cur_line.is_none() {
                        break Position::new(self.cur_lineno, line_len, Rc::clone(&self.cur_filename));
                    }
                }
            }
        };
        Ok(Some(Token::new(
            TokenKind::UNMATCHED,
            original,
            from,
            to,
        )))
    }
    fn next_line(&mut self) {
        match self.scanner.next() {
            Some(scan::Line::Line { text, lineno }) => {
                self.cur_line = Some(text);
                self.cur_lineno = lineno;
                self.cursor = 0;
            }
            Some(scan::Line::ChangeFile(filename)) => {
                self.cur_filename = filename;
                self.next_line();
            }
            None => self.cur_line = None,
        }
    }

    fn match_any_identifier(
        &mut self,
        check_first_char: fn(u8) -> bool,
        make_token_kind: fn(String) -> TokenKind,
    ) -> LexResult<Option<Token>> {
        let line = self.get_cur_line("match_any_identifier");
        if check_first_char(line[self.cursor]) {
            let from = self.make_position(self.cursor);
            let i = if let Some((i, _)) = line[self.cursor + 1..]
                .iter()
                .enumerate()
                .take_while(|(_, &c)| c.is_ascii_alphanumeric() || c == b'_')
                .last()
            {
                i + 1
            } else {
                0
            };
            // let to = self.make_position(self.cursor + i);
            let identifier = line[self.cursor..=self.cursor + i].to_vec();
            self.cursor += i + 1;
            let retval = Token::new(
                make_token_kind(String::from_utf8(identifier.clone()).expect("should be alphanumeric")),
                identifier,
                from,
                self.make_position(self.cursor),
            );
            Ok(Some(retval))
        } else {
            Ok(None)
        }
    }
    fn match_lexemes(&mut self, lexemes: &[TokenKind]) -> LexResult<Option<Token>> {
        let line = self.get_cur_line("match_lexemes");
        for lexeme in lexemes.iter() {
            let lexeme_str = lexeme.to_string();
            if line[self.cursor..].starts_with(lexeme_str.as_bytes()) {
                let from = self.make_position(self.cursor);
                self.cursor += lexeme_str.len();
                return Ok(Some(Token::new(
                    lexeme.clone(),
                    lexeme_str.as_bytes().to_vec(),
                    from,
                    self.make_position(self.cursor ),
                )));
            }
        }
        Ok(None)

    }
    fn count_digits(&self, line: &[u8], start: usize) -> usize {
        line[start..]
            .iter()
            .take_while(|&&c| c.is_ascii_digit())
            .count()
    }
    fn parse_string_escape_seqs(s: &[u8]) -> Result<String, usize> {
        let mut idx = 0;
        let mut retval = String::new();
        while let Some((i, _)) = s[idx..].iter().enumerate().find(|(_, &c)| c == b'\\') {
            let slash_idx = idx + i;
            retval.push_str(
                &String::from_utf8(s[idx..slash_idx].to_vec()).expect("should be valid utf8"),
            );
            let chars_cnt_after_slash = s.len() - slash_idx - 1;
            if chars_cnt_after_slash >= 1 && s[slash_idx + 1] != b'x' {
                if let Some(escaped) = Self::parse_single_char_escape_seq(s[slash_idx + 1]) {
                    retval.push(escaped as char);
                    idx = slash_idx + 2;
                } else {
                    return Err(slash_idx);
                }
            } else if chars_cnt_after_slash >= 3 && s[slash_idx + 1] == b'x' {
                if let Some(escaped) =
                    Self::parse_hex_escape_seq(s[slash_idx + 2], s[slash_idx + 3])
                {
                    retval.push(escaped as char);
                    idx = slash_idx + 4;
                } else {
                    return Err(slash_idx);
                }
            } else {
                return Err(slash_idx);
            }
        }
        retval.push_str(&String::from_utf8(s[idx..].to_vec()).expect("should be valid utf8"));
        Ok(retval)
    }
    fn parse_single_char_escape_seq(c: u8) -> Option<u8> {
        match c {
            b'n' => Some(b'\n'),
            b'r' => Some(b'\r'),
            b't' => Some(b'\t'),
            b'\'' => Some(b'\''),
            b'"' => Some(b'"'),
            b'\\' => Some(b'\\'),
            _ => None,
        }
    }
    fn parse_hex_escape_seq(c1: u8, c2: u8) -> Option<u8> {
        if !((c1 as char).is_ascii_hexdigit() && (c2 as char).is_ascii_hexdigit()) {
            return None;
        }
        String::from_utf8(vec![c1, c2]).expect("should be ascii hexdigits").parse().ok()
    }

    fn make_position(&self, cursor: usize) -> Position {
        Position::new(self.cur_lineno, cursor + 1, Rc::clone(&self.cur_filename))
    }
}

impl<S: Iterator<Item = scan::Line>> Iterator for Lexer<S> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Some(token)) => Some(token),
            Ok(None) => None,
            Err(err) => {
                if self.exit_on_error {
                    error!("{}", err);
                    std::process::exit(1);
                } else {
                    todo!("handle error (possibly by calling match_unmatched");
                }
            }
        }
    }
}
pub trait IntoLexer: Iterator<Item = scan::Line> + Sized {
    fn into_lexer(self, exit_on_error: bool) -> Lexer<Self> {
        let mut lexer = Lexer::new(self);
        lexer.exit_on_error = exit_on_error;
        lexer
    }
}
impl<S: Iterator<Item = scan::Line>> IntoLexer for S {}

type LexResult<T> = Result<T, LexErr>;
pub enum LexErr {
    UnterminatedComment(Position),
    InvalidCharLiteral(Position, &'static str),
    InvalidStringLiteral(Position, &'static str),
    ParseFloatError(Position, String),
    ParseIntError(Position, String),
    FromUtf8Error(Position, String),
}

impl std::fmt::Display for LexErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnterminatedComment(pos) => write!(f, "unterminated comment at {}", pos),
            Self::InvalidCharLiteral(pos, msg) |
            Self::InvalidStringLiteral(pos, msg) => {
                write!(f, "{} at {}", msg, pos)
            }
            Self::ParseFloatError(pos, msg) |
            Self::ParseIntError(pos, msg) |
            Self::FromUtf8Error(pos, msg) => {
                write!(f, "{} at {}", msg, pos)
            }
        }
    }
}
