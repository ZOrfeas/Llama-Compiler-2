use std::fmt::format;

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

    pub fn program(&mut self) -> ParseResult<ast::Program> {
        let mut definitions: Vec<ast::Definition> = Vec::new();
        while self.accept(&TokenKind::EOF).is_none() {
            let definition = self.expect_any_of(&[
                (TokenKind::Let, |p, _| p.letdef().map(ast::Definition::Let)),
                (TokenKind::Type, |p, _| {
                    p.typedef().map(ast::Definition::Type)
                }),
            ])?;
            definitions.push(definition);
        }
        Ok(ast::Program { definitions })
    }
    fn letdef(&mut self) -> ParseResult<ast::Letdef> {
        todo!()
    }
    fn def(&mut self) -> ParseResult<()> {
        todo!()
    }
    fn typedef(&mut self) -> ParseResult<ast::Typedef> {
        Ok(ast::Typedef {
            tdefs: self.match_at_least_one(Self::tdef, &TokenKind::And)?,
        })
    }
    fn tdef(&mut self) -> ParseResult<ast::TDef> {
        let id = self.expect(TokenKind::IdLower)?;
        self.expect(TokenKind::Eq)?;
        Ok(ast::TDef {
            id: id.get_string_value(),
            constrs: self.match_at_least_one(Self::constr, &TokenKind::Bar)?,
        })
    }
    fn constr(&mut self) -> ParseResult<ast::Constr> {
        let id = self.expect(TokenKind::IdUpper)?;
        let types = if self.accept(&TokenKind::Of).is_some() {
            self.match_at_least_one_until(Self::r#type, &TokenKind::Bar)?
        } else {
            Vec::new()
        };
        Ok(ast::Constr {
            id: id.get_string_value(),
            types,
        })
    }
    fn par(&mut self) -> ParseResult<()> {
        todo!()
    }

    fn type_precedence_helper(&mut self) -> ParseResult<ast::Type> {
        let mut t = self.expect_any_of(&[
            (TokenKind::Unit, |_, _| Ok(ast::Type::Unit)),
            (TokenKind::Int, |_, _| Ok(ast::Type::Int)),
            (TokenKind::Char, |_, _| Ok(ast::Type::Char)),
            (TokenKind::Bool, |_, _| Ok(ast::Type::Bool)),
            (TokenKind::Float, |_, _| Ok(ast::Type::Float)),
            (TokenKind::LParen, |p, _| {
                let t = p
                    .match_at_least_one(Self::r#type, &TokenKind::Comma)
                    .map(ast::Type::maybe_tuple)?;
                p.expect(TokenKind::RParen)?;
                Ok(t)
            }),
            (TokenKind::Array, |p, _| {
                let dimensions = if p.accept(&TokenKind::LBracket).is_some() {
                    p.expect(TokenKind::Star)?;
                    let mut dimensions = 1;
                    while p.accept(&TokenKind::Comma).is_some() {
                        p.expect(TokenKind::Star)?;
                        dimensions += 1;
                    }
                    p.expect(TokenKind::RBracket)?;
                    dimensions
                } else {
                    1
                };
                p.expect(TokenKind::Of)?;
                p.type_precedence_helper()
                    .map(|t| ast::Type::Array(Box::new(t), dimensions))
            }),
            (TokenKind::IdLower, |_, t| {
                Ok(ast::Type::Custom(t.get_string_value()))
            }),
        ])?;
        // below loop handles type_recursion_helper non-terminal
        while self.accept(&TokenKind::Ref).is_some() {
            t = ast::Type::Ref(Box::new(t));
        }
        Ok(t)
    }
    // fn type_recursion_helper(&mut self) -> ParseResult<ast::Type> {
    //     // self.expect(TokenKind::Ref)?;
    // }
    fn r#type(&mut self) -> ParseResult<ast::Type> {
        let t1 = self.type_precedence_helper()?;
        if self.accept(&TokenKind::Arrow).is_some() {
            let t2 = self.r#type()?;
            Ok(ast::Type::Func(Box::new(t1), Box::new(t2)))
        } else {
            Ok(t1)
        }
    }
    fn expr(&mut self) -> ParseResult<()> {
        todo!()
    }

    fn expect_any_of<T>(
        &mut self,
        kinds_callbacks: &[(TokenKind, fn(&mut Self, Token) -> ParseResult<T>)],
    ) -> ParseResult<T> {
        if let Some((token, callback)) = kinds_callbacks
            .iter()
            .find_map(|(kind, callback)| self.accept(kind).map(|t| (t, callback)))
        {
            callback(self, token)
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

    fn match_zero_or_more<T>(
        &mut self,
        matcher: fn(&mut Self) -> ParseResult<T>,
        separator: &TokenKind,
    ) -> ParseResult<Vec<T>> {
        let mut vec: Vec<T> = Vec::new();
        while self.accept(separator).is_some() {
            vec.push(matcher(self)?);
        }
        Ok(vec)
    }

    fn match_at_least_one<T>(
        &mut self,
        matcher: fn(&mut Self) -> ParseResult<T>,
        separator: &TokenKind,
    ) -> ParseResult<Vec<T>> {
        let mut vec: Vec<T> = Vec::new();
        loop {
            vec.push(matcher(self)?);
            if self.accept(separator).is_none() {
                break;
            }
        }
        Ok(vec)
    }
    fn match_at_least_one_until<T>(
        &mut self,
        matcher: fn(&mut Self) -> ParseResult<T>,
        separator: &TokenKind,
    ) -> ParseResult<Vec<T>> {
        let mut vec: Vec<T> = Vec::new();
        loop {
            vec.push(matcher(self)?);
            if Some(separator) == self.lexer.peek().map(|t| &t.kind) {
                break;
            }
        }
        Ok(vec)
    }
}

pub trait IntoParser: Iterator<Item = Token> + Sized {
    fn into_parser(self) -> Parser<Self> {
        Parser::new(self)
    }
}
impl<I: Iterator<Item = Token>> IntoParser for I {}

type ParseResult<T> = Result<T, ParseErr>;
#[derive(Debug)]
pub enum ParseErr {
    UnexpectedToken(Option<Token>, Vec<TokenKind>),
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErr::UnexpectedToken(token, expected) => {
                let expected = expected
                    .iter()
                    .map(|t| format!("\"{}\"", t))
                    .collect::<Vec<_>>()
                    .join(", ");
                match token {
                    Some(token) => write!(
                        f,
                        "{}: Unexpected token \"{}\", expected any of {{{}}}",
                        token.from, token.kind, expected
                    ),
                    None => write!(f, "Unexpected end of file, expected any of {}", expected),
                }
            }
        }
    }
}
