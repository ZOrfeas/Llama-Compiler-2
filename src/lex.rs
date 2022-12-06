mod token;

use std::{
    num::{ParseFloatError, ParseIntError},
    ops::Not,
    rc::Rc,
    string::FromUtf8Error,
};

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
        }
    }

    fn next_token(&mut self) -> LexResult<Option<Token>> {
        let matchers = [
            Self::match_eof,
            Self::match_multi_line_comment,
            Self::match_single_line_comment,
            Self::match_reserved_word,
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
                return Ok(Some(token));
            }
        }
        unreachable!("unmatched cases handled above, this should be unreachable")
    }
    fn eat_whitespace(&mut self) {
        if self.cursor >= self.cur_line.as_ref().unwrap().len() {
            self.next_line();
        }
        while let Some(line) = &self.cur_line {
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
        if !self.cur_line.as_ref().unwrap()[self.cursor..].starts_with(b"(*") {
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
                            self.make_position(self.cursor - 1),
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
        let line = self.cur_line.as_ref().unwrap();
        if line[self.cursor..].starts_with(b"--") {
            let from = self.make_position(self.cursor);
            // self.cursor = line.len() - 1;
            let to = self.make_position(line.len() - 1);
            let retval = Token::new(TokenKind::COMMENT, line[self.cursor..].to_vec(), from, to);
            self.next_line();
            Ok(Some(retval))
        } else {
            Ok(None)
        }
    }
    fn match_reserved_word(&mut self) -> LexResult<Option<Token>> {
        const LEXEMES: [TokenKind; 34] = [
            TokenKind::And,
            TokenKind::Array,
            TokenKind::Begin,
            TokenKind::Bool,
            TokenKind::Char,
            TokenKind::Delete,
            TokenKind::Dim,
            TokenKind::Do,
            TokenKind::Done,
            TokenKind::Downto,
            TokenKind::Else,
            TokenKind::End,
            TokenKind::False,
            TokenKind::Float,
            TokenKind::For,
            TokenKind::If,
            TokenKind::In,
            TokenKind::Int,
            TokenKind::Let,
            TokenKind::Match,
            TokenKind::Mod,
            TokenKind::Mutable,
            TokenKind::New,
            TokenKind::Not,
            TokenKind::Of,
            TokenKind::Rec,
            TokenKind::Ref,
            TokenKind::Then,
            TokenKind::To,
            TokenKind::True,
            TokenKind::Type,
            TokenKind::Unit,
            TokenKind::While,
            TokenKind::With,
        ];
        let line = self.cur_line.as_ref().unwrap();
        for lexeme in LEXEMES.iter() {
            let lexeme_str = lexeme.to_string();
            if line[self.cursor..].starts_with(lexeme_str.as_bytes()) {
                let from = self.make_position(self.cursor);
                self.cursor += lexeme_str.len();
                return Ok(Some(Token::new(
                    lexeme.clone(),
                    lexeme_str.as_bytes().to_vec(),
                    from,
                    self.make_position(self.cursor - 1),
                )));
            }
        }
        Ok(None)
    }
    fn match_lowercase_identifier(&mut self) -> LexResult<Option<Token>> {
        self.match_any_identifier(|c| c.is_ascii_lowercase())
    }
    fn match_uppercase_identifier(&mut self) -> LexResult<Option<Token>> {
        self.match_any_identifier(|c| c.is_ascii_uppercase())
    }
    fn match_float_literal(&mut self) -> LexResult<Option<Token>> {
        let line = self.cur_line.as_ref().unwrap();
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
            .map_err(|e| LexErr::FromUtf8Error(from.clone(), e))?
            .parse::<f64>()
            .map_err(|e| LexErr::ParseFloatError(from.clone(), e))?;
        Ok(Some(Token::new(
            TokenKind::FloatLiteral(number),
            contents,
            from,
            self.make_position(self.cursor - 1),
        )))
    }
    fn match_integer_literal(&mut self) -> LexResult<Option<Token>> {
        let line = self.cur_line.as_ref().unwrap();
        let digits = self.count_digits(line, self.cursor);
        if digits == 0 {
            return Ok(None);
        }
        let from = self.make_position(self.cursor);
        let contents = line[self.cursor..self.cursor + digits].to_vec();
        self.cursor += digits;
        let number = String::from_utf8(contents.clone())
            .map_err(|e| LexErr::FromUtf8Error(from.clone(), e))?
            .parse::<i32>()
            .map_err(|e| LexErr::ParseIntError(from.clone(), e))?;
        Ok(Some(Token::new(
            TokenKind::IntLiteral(number),
            contents,
            from,
            self.make_position(self.cursor - 1),
        )))
    }
    fn match_character_literal(&mut self) -> LexResult<Option<Token>> {
        let line = self.cur_line.as_ref().unwrap();
        if line.get(self.cursor) != Some(&b'\'') {
            return Ok(None);
        }
        let from = self.make_position(self.cursor);
        if line.get(self.cursor + 2) == Some(&b'\'') && line.get(self.cursor + 1) != Some(&b'\\') {
            let c = line[self.cursor + 1];
            const NON_COMMON_CHARS: [u8; 5] = [b'\n', b'\r', b'\t', b'\'', b'"'];
            if !NON_COMMON_CHARS.contains(&c) {
                let original = line[self.cursor..=self.cursor + 2].to_vec();
                self.cursor += 3;
                return Ok(Some(Token::new(
                    TokenKind::CharLiteral(c),
                    original,
                    from,
                    self.make_position(self.cursor - 1),
                )));
            } else {
                return Err(LexErr::InvalidCharLiteral(
                    from,
                    "Only common and printable characters are allowed as character literals",
                ));
            }
        }
        if line.get(self.cursor + 3) == Some(&b'\'') && line.get(self.cursor + 1) == Some(&b'\\') {
            let c = line[self.cursor + 2];
            if let Some(real_char) = match c {
                b'n' => Some(b'\n'),
                b'r' => Some(b'\r'),
                b't' => Some(b'\t'),
                b'\'' => Some(b'\''),
                b'"' => Some(b'"'),
                b'\\' => Some(b'\\'),
                _ => None,
            } {
                let original = line[self.cursor..=self.cursor + 3].to_vec();
                self.cursor += 4;
                return Ok(Some(Token::new(
                    TokenKind::CharLiteral(real_char),
                    original,
                    from,
                    self.make_position(self.cursor - 1),
                )));
            } else {
                return Err(LexErr::InvalidCharLiteral(from, "Invalid escape sequence"));
            }
        }
        if line.get(self.cursor + 5) == Some(&b'\'')
            && line.get(self.cursor + 1) == Some(&b'\\')
            && line.get(self.cursor + 2) == Some(&b'x')
        {
            let c1 = line[self.cursor + 3];
            let c2 = line[self.cursor + 4];
            if (c1 as char).is_ascii_hexdigit() && (c2 as char).is_ascii_hexdigit() {
                let c = u8::from_str_radix(
                    &String::from_utf8(line[self.cursor + 3..=self.cursor + 4].to_vec()).map_err(
                        |e| LexErr::FromUtf8Error(self.make_position(self.cursor + 3), e),
                    )?,
                    16,
                )
                .map_err(|e| LexErr::ParseIntError(self.make_position(self.cursor + 3), e))?;
                let original = line[self.cursor..=self.cursor + 5].to_vec();
                self.cursor += 6;
                return Ok(Some(Token::new(
                    TokenKind::CharLiteral(c),
                    original,
                    from,
                    self.make_position(self.cursor - 1),
                )));
            } else {
                return Err(LexErr::InvalidCharLiteral(
                    from,
                    "Invalid hex code escape sequence",
                ));
            }
        }
        Ok(None)
    }
    fn match_string_literal(&mut self) -> LexResult<Option<Token>> {
        todo!()
    }
    fn match_multi_char_symop(&mut self) -> LexResult<Option<Token>> {
        todo!()
    }
    fn match_single_char_symop_or_sep(&mut self) -> LexResult<Option<Token>> {
        todo!()
    }
    fn match_unmatched(&mut self) -> LexResult<Option<Token>> {
        todo!()
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

    fn match_any_identifier(&mut self, f: fn(&u8) -> bool) -> LexResult<Option<Token>> {
        let line = self.cur_line.as_ref().unwrap();
        if f(&line[self.cursor]) {
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
            let to = self.make_position(self.cursor + i);
            let identifier = line[self.cursor..=i].to_vec();
            let retval = Token::new(
                TokenKind::IdUpper(String::from_utf8(identifier.clone()).unwrap()),
                identifier,
                from,
                to,
            );
            self.cursor = i + 1;
            Ok(Some(retval))
        } else {
            Ok(None)
        }
    }
    fn count_digits(&self, line: &[u8], start: usize) -> usize {
        line[start..]
            .iter()
            .take_while(|&&c| c.is_ascii_digit())
            .count()
    }

    fn make_position(&self, cursor: usize) -> Position {
        Position::new(self.cur_lineno, cursor + 1, Rc::clone(&self.cur_filename))
    }
}

// impl<S: Iterator<Item = scan::Line>> Iterator for Lexer<S> {
//     type Item = Token;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.next_token() {
//             Ok(Some(token)) => Some(token),
//             Ok(None) => None,
//             Err(err) => {
//                 if self.exit_on_error {
//                     error!("{}", err);
//                     std::process::exit(1);
//                 } else {
//                     todo!("handle error (possibly by calling match_unmatched");
//                 }
//             }
//         }
//     }
// }

type LexResult<T> = Result<T, LexErr>;
pub enum LexErr {
    UnterminatedComment(Position),
    InvalidIntLiteral(Position),
    InvalidCharLiteral(Position, &'static str),
    ParseFloatError(Position, ParseFloatError),
    ParseIntError(Position, ParseIntError),
    FromUtf8Error(Position, FromUtf8Error),
}

impl std::fmt::Display for LexErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
