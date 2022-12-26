use crate::{
    lex::token::{Token, TokenKind},
    long_peekable::{LongPeek, LongPeekableIterator},
};

pub mod ast;

pub struct Parser<L: Iterator<Item = Token>> {
    lexer: LongPeekableIterator<L>,
}

impl<L: Iterator<Item = Token>> Parser<L> {
    pub fn new(lexer: L) -> Self {
        Self {
            lexer: lexer.long_peekable(),
        }
    }
    // pub fn parse(&mut self) -> ast::Program {}

    fn program(&mut self) -> ParseResult<Vec<()>> {
        let mut defs: Vec<()> = Vec::new();
        while self.accept(&TokenKind::EOF).is_none() {
            let definition = self.expect_any_of(&[
                (TokenKind::Let, |p| p.letdef()),
                (TokenKind::Type, |p| p.typedef()),
            ])?;
            defs.push(definition);
        }
        Ok(defs)
    }
    fn letdef(&mut self) -> ParseResult<()> {
        todo!()
    }
    fn def(&mut self) -> ParseResult<()> {
        todo!()
    }
    fn typedef(&mut self) -> ParseResult<()> {}
    fn tdef(&mut self) -> ParseResult<()> {
        todo!()
    }
    fn constr(&mut self) -> ParseResult<()> {
        todo!()
    }
    fn par(&mut self) -> ParseResult<()> {
        todo!()
    }
    fn r#type(&mut self) -> ParseResult<()> {
        todo!()
    }
    fn expr(&mut self) -> ParseResult<()> {
        todo!()
    }

    fn expect_any_of<T>(
        &mut self,
        kinds_callbacks: &[(TokenKind, fn(&mut Self) -> ParseResult<T>)],
    ) -> ParseResult<T> {
        if let Some(pair) = kinds_callbacks
            .iter()
            .find(|(kind, _)| self.accept(kind).is_some())
        {
            (pair.1)(self)
        } else {
            Err(ParseErr::UnexpectedToken(
                self.lexer.peek().cloned(),
                kinds_callbacks
                    .iter()
                    .map(|(kind, _)| kind.clone())
                    .collect(),
            ))
        }
    }
    fn expect(&mut self, token_kind: TokenKind) -> ParseResult<Token> {
        self.accept(&token_kind).ok_or(ParseErr::UnexpectedToken(
            self.lexer.peek().cloned(),
            vec![token_kind],
        ))
    }
    fn accept(&mut self, token_kind: &TokenKind) -> Option<Token> {
        if let Some(true) = self.lexer.peek().map(|t| &t.kind == (token_kind)) {
            self.lexer.next()
        } else {
            None
        }
    }
}

pub trait IntoParser: Iterator<Item = Token> + Sized {
    fn into_parser(self) -> Parser<Self> {
        Parser::new(self)
    }
}
impl<I: Iterator<Item = Token>> IntoParser for I {}

type ParseResult<T> = Result<T, ParseErr>;
pub enum ParseErr {
    UnexpectedToken(Option<Token>, Vec<TokenKind>),
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
