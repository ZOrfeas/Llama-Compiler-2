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
        Ok(ast::Letdef {
            rec: self.accept(&TokenKind::Rec).is_some(),
            defs: self.match_at_least_one(Self::def, &TokenKind::And)?,
        })
    }
    fn def(&mut self) -> ParseResult<ast::Def> {
        self.expect_any_of(&[
            (TokenKind::IdLower, |p, id| {
                let id = id.extract_string_value();
                let pars =
                    p.match_zero_or_more_until(Self::par, &[TokenKind::Colon, TokenKind::Eq])?;
                let type_ = if p.accept(&TokenKind::Colon).is_some() {
                    Some(p.r#type()?)
                } else {
                    None
                };
                p.expect(TokenKind::Eq)?;
                let expr = p.expr()?;
                if pars.is_empty() {
                    Ok(ast::Def::Const(ast::ConstDef { id, type_, expr }))
                } else {
                    Ok(ast::Def::Function(ast::FunctionDef {
                        id,
                        pars,
                        type_,
                        expr,
                    }))
                }
            }),
            (TokenKind::Mutable, |p, _| {
                let id = p.expect(TokenKind::IdLower)?.extract_string_value();
                let dims = if p.accept(&TokenKind::LBracket).is_some() {
                    let dims = p.match_at_least_one(Self::expr, &TokenKind::Comma)?;
                    p.expect(TokenKind::RBracket)?;
                    dims
                } else {
                    Vec::new()
                };
                let type_ = if p.accept(&TokenKind::Colon).is_some() {
                    Some(p.r#type()?)
                } else {
                    None
                };
                if dims.is_empty() {
                    Ok(ast::Def::Variable(ast::VariableDef { id, type_ }))
                } else {
                    Ok(ast::Def::Array(ast::ArrayDef { id, type_, dims }))
                }
            }),
        ])
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
            id: id.extract_string_value(),
            constrs: self.match_at_least_one(Self::constr, &TokenKind::Bar)?,
        })
    }
    fn constr(&mut self) -> ParseResult<ast::Constr> {
        let id = self.expect(TokenKind::IdUpper)?;
        let types = if self.accept(&TokenKind::Of).is_some() {
            self.match_at_least_one_until(Self::r#type, &[TokenKind::Bar])?
        } else {
            Vec::new()
        };
        Ok(ast::Constr {
            id: id.extract_string_value(),
            types,
        })
    }
    fn par(&mut self) -> ParseResult<ast::Par> {
        expect_any_of!(self,
            (TokenKind::IdLower) -> |token: Token| {
                Ok(ast::Par {
                    id: token.extract_string_value(),
                    type_: None,
                })
            },
            (TokenKind::LParen) -> |_| {
                let id = self.expect(TokenKind::IdLower)?.extract_string_value();
                self.expect(TokenKind::Colon)?;
                let type_ = self.r#type()?;
                self.expect(TokenKind::RParen)?;
                Ok(ast::Par {
                    id,
                    type_: Some(type_),
                })
            }
        )
    }

    fn r#type(&mut self) -> ParseResult<ast::Type> {
        let t1 = self.type_precedence_helper()?;
        if self.accept(&TokenKind::Arrow).is_some() {
            let t2 = self.r#type()?;
            Ok(ast::Type::Func(Box::new(t1), Box::new(t2)))
        } else {
            Ok(t1)
        }
    }
    fn type_precedence_helper(&mut self) -> ParseResult<ast::Type> {
        let mut t = expect_any_of!(self,
            (TokenKind::Unit) -> |token: Token| Ok((&token).into()),
            (TokenKind::Int) -> |token: Token| Ok((&token).into()),
            (TokenKind::Char) -> |token: Token| Ok((&token).into()),
            (TokenKind::Bool) -> |token: Token| Ok((&token).into()),
            (TokenKind::Float) -> |token: Token| Ok((&token).into()),
            (TokenKind::LParen) -> |_| {
                let t = self
                    .match_at_least_one(Self::r#type, &TokenKind::Comma)
                    .map(ast::Type::maybe_tuple)?;
                self.expect(TokenKind::RParen)?;
                Ok(t)
            },
            (TokenKind::Array) -> |_| {
                let dimensions = if self.accept(&TokenKind::LBracket).is_some() {
                    self.expect(TokenKind::Star)?;
                    let mut dimensions = 1;
                    while self.accept(&TokenKind::Comma).is_some() {
                        self.expect(TokenKind::Star)?;
                        dimensions += 1;
                    }
                    self.expect(TokenKind::RBracket)?;
                    dimensions
                } else {
                    1
                };
                self.expect(TokenKind::Of)?;
                self.type_precedence_helper()
                    .map(|t| ast::Type::Array(Box::new(t), dimensions))
            },
            (TokenKind::IdLower) -> |token: Token| {
                Ok(ast::Type::Custom(token.extract_string_value()))
            }
        )?;
        // below loop handles type_recursion_helper non-terminal
        while self.accept(&TokenKind::Ref).is_some() {
            t = ast::Type::Ref(Box::new(t));
        }
        Ok(t)
    }
    fn expr(&mut self) -> ParseResult<ast::Expr> {
        todo!("expr")
    }
    fn clause(&mut self) -> ParseResult<ast::Clause> {
        todo!("clause")
    }
    fn pattern(&mut self) -> ParseResult<ast::Pattern> {
        todo!("pattern")
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
    fn match_zero_or_more_until<T>(
        &mut self,
        matcher: fn(&mut Self) -> ParseResult<T>,
        start_tokens: &[TokenKind],
        stop_tokens: &[TokenKind],
    ) -> ParseResult<Vec<T>> {
        if start_tokens
            .iter()
            .find(|&separator| Some(separator) == self.lexer.peek().map(|t| &t.kind))
            .is_none()
        {
            return Ok(vec![]);
        }
        let mut vec: Vec<T> = Vec::new();
        while stop_tokens
            .iter()
            .find(|&separator| Some(separator) == self.lexer.peek().map(|t| &t.kind))
            .is_none()
        {
            vec.push(matcher(self)?);
        }
        Ok(vec)
    }
    fn match_at_least_one_until<T>(
        &mut self,
        matcher: fn(&mut Self) -> ParseResult<T>,
        stop_tokens: &[TokenKind],
    ) -> ParseResult<Vec<T>> {
        let mut vec: Vec<T> = Vec::new();
        loop {
            vec.push(matcher(self)?);
            let peek_kind = self.lexer.peek().map(|t| &t.kind);
            if stop_tokens
                .iter()
                .find(|&separator| Some(separator) == peek_kind)
                .is_some()
            {
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
