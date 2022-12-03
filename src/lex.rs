mod token;

use log::error;

use crate::scan;

use self::token::{Position, Token};

pub struct Lexer<S: Iterator<Item = scan::Line>> {
    scanner: S,
    exit_on_error: bool,

    pos: Position,
    cur_line: Option<Vec<u8>>,
    cursor: usize,
    comment_nesting: usize,
}
// TODO: Consider a State-struct to hold the lexer's state grouped together.
// TODO:     cur_line, cursor, pos, comment_nesting, etc.
impl<S: Iterator<Item = scan::Line>> Lexer<S> {
    pub fn new(scanner: S) -> Self {
        Lexer {
            scanner,
            exit_on_error: true,
            pos: Position::new(0, 0, None),
            cur_line: None,
            cursor: 0,
            comment_nesting: 0,
        }
    }

    fn next_token(&mut self) -> LexResult<Option<Token>> {
        let matchers = [
            Self::match_eof,
            Self::match_single_line_comment,
            Self::match_multi_line_comment,
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
        todo!()
    }
    // !NOTE: All match functions return a LexResult even though some may be infallible for easier grouping afterwards.
    fn match_eof(&mut self) -> LexResult<Option<Token>> {
        todo!()
    }
    fn match_single_line_comment(&mut self) -> LexResult<Option<Token>> {
        todo!()
    }
    // !NOTE: Matches the start of a multi-line comment. Since wr are on a line by line basis, this needs care.
    fn match_multi_line_comment(&mut self) -> LexResult<Option<Token>> {
        todo!()
    }
    fn match_reserved_word(&mut self) -> LexResult<Option<Token>> {
        todo!()
    }
    fn match_lowercase_identifier(&mut self) -> LexResult<Option<Token>> {
        todo!()
    }
    fn match_uppercase_identifier(&mut self) -> LexResult<Option<Token>> {
        todo!()
    }
    fn match_float_literal(&mut self) -> LexResult<Option<Token>> {
        todo!()
    }
    fn match_integer_literal(&mut self) -> LexResult<Option<Token>> {
        todo!()
    }
    fn match_character_literal(&mut self) -> LexResult<Option<Token>> {
        todo!()
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
                todo!("set cur_line, reset cursor, set pos, maybe others")
            }
            Some(scan::Line::ChangeFile(filename)) => {
                todo!("set pos.filename, probably call self.next_line() again")
            }
            None => todo!("set cur_line to None, and this probably means lexing is done"),
        }
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

type LexResult<T> = Result<T, LexErr>;
pub enum LexErr {}

impl std::fmt::Display for LexErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
