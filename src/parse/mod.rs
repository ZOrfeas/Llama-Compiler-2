#![allow(unused_parens)]

use log::debug;

use crate::{
    lex::token::{Token, TokenKind},
    long_peekable::{LongPeek, LongPeekableIterator},
};

pub mod ast;
macro_rules! expect_any_of {
    (@as_expr $e:expr) => {$e};
    (@as_pat $p:pat) => {$p};
    ($self:ident, $($kind:path $(| $kinds:path)* => $expr:expr),+) => {
        match $self.peek_token().map(|t| &t.kind) {
            $(
                Some($kind) $(| Some($kinds))* => {
                    let token = $self.consume_token()
                        .expect("TokenKind matched but not accepted");
                    $expr(token)
                }
            )*
            _ => Err(ParseErr::UnexpectedToken(
                $self.peek_token().cloned(),
                vec![$($kind $(,$kinds)*),*]
            ))

        }
    }
}

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
            let definition = expect_any_of!(self,
                TokenKind::Let => |_| self.letdef().map(ast::Definition::Let),
                TokenKind::Type => |_| self.typedef().map(ast::Definition::Type)
            )?;
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
        expect_any_of!(self,
            TokenKind::IdLower => |token: Token| {
                let id = token.extract_string_value();
                let pars = self.match_zero_or_more_multiple(
                    Self::par,
                    &[TokenKind::IdLower, TokenKind::LParen],
                )?;
                let type_ = if self.accept(&TokenKind::Colon).is_some() {
                    Some(self.r#type()?)
                } else {
                    None
                };
                self.expect(TokenKind::Eq)?;
                let expr = self.expr()?;
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
            },
            TokenKind::Mutable => |_| {
                let id = self.expect(TokenKind::IdLower)?.extract_string_value();
                let dims = if self.accept(&TokenKind::LBracket).is_some() {
                    let dims = self.match_at_least_one(Self::expr, &TokenKind::Comma)?;
                    self.expect(TokenKind::RBracket)?;
                    dims
                } else {
                    Vec::new()
                };
                let type_ = if self.accept(&TokenKind::Colon).is_some() {
                    Some(self.r#type()?)
                } else {
                    None
                };
                if dims.is_empty() {
                    Ok(ast::Def::Variable(ast::VariableDef { id, type_ }))
                } else {
                    Ok(ast::Def::Array(ast::ArrayDef { id, type_, dims }))
                }
            }
        )
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
            TokenKind::IdLower => |token: Token| {
                Ok(ast::Par {
                    id: token.extract_string_value(),
                    type_: None,
                })
            },
            TokenKind::LParen => |_| {
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
            let t1 = Box::new(t1);
            let t2 = Box::new(self.r#type()?);
            Ok(ast::Type::Func(t1, t2))
        } else {
            Ok(t1)
        }
    }
    fn type_precedence_helper(&mut self) -> ParseResult<ast::Type> {
        let mut t = expect_any_of!(self,
            TokenKind::Unit | TokenKind::Int | TokenKind::Char
            | TokenKind::Bool | TokenKind::Float  => |token: Token| Ok((&token.kind).into()),
            TokenKind::LParen => |_| {
                let t = self
                    .match_at_least_one(Self::r#type, &TokenKind::Comma)
                    .map(ast::Type::maybe_tuple)?;
                self.expect(TokenKind::RParen)?;
                Ok(t)
            },
            TokenKind::Array => |_| {
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
            TokenKind::IdLower => |token: Token| {
                Ok(ast::Type::Custom(token.extract_string_value()))
            }
        )?;
        // below loop handles type_recursion_helper non-terminal
        while self.accept(&TokenKind::Ref).is_some() {
            t = ast::Type::Ref(Box::new(t));
        }
        Ok(t)
    }
    #[inline(always)]
    fn expr(&mut self) -> ParseResult<ast::Expr> {
        self.expr0()
    }
    fn expr0(&mut self) -> ParseResult<ast::Expr> {
        let letdefs = self.match_zero_or_more(Self::letdef, &TokenKind::Let)?;
        if letdefs.is_empty() {
            self.expr1()
        } else {
            self.expect(TokenKind::In)?;
            let expr = self.expr1()?;
            Ok(letdefs.into_iter().rfold(expr, |expr, letdef| {
                ast::Expr::LetIn(letdef, Box::new(expr))
            }))
        }
    }
    fn expr1(&mut self) -> ParseResult<ast::Expr> {
        let exprs = self.match_at_least_one(Self::expr2, &TokenKind::Semicolon)?;
        Ok(ast::Expr::left_assoc_from_vec(exprs, &TokenKind::Semicolon))
    }
    fn expr2(&mut self) -> ParseResult<ast::Expr> {
        if self.accept(&TokenKind::If).is_some() {
            let cond = Box::new(self.expr2()?);
            self.expect(TokenKind::Then)?;
            let then = Box::new(self.expr2()?);
            let r#else = if self.accept(&TokenKind::Else).is_some() {
                Some(Box::new(self.expr2()?))
            } else {
                None
            };
            Ok(ast::Expr::If(cond, then, r#else))
        } else {
            self.expr3()
        }
    }
    // TODO: Use structs on AST.
    // TODO: Improve printing where possible.
    // TODO: Think about error messages.
    fn expr3(&mut self) -> ParseResult<ast::Expr> {
        todo!()
        // self.expr9()
    }
    fn expr9(&mut self) -> ParseResult<ast::Expr> {
        const OPS: [TokenKind; 6] = [
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::PlusDot,
            TokenKind::MinusDot,
            TokenKind::Not,
            TokenKind::Delete,
        ];
        let mut unops = Vec::new();
        while self
            .peek_token()
            .map(|token| OPS.contains(&token.kind))
            .unwrap_or(false)
        {
            unops.push(
                self.consume_token()
                    .expect("peeeked token should be present"),
            );
        }
        let expr = self.expr10()?;
        Ok(unops.into_iter().rfold(expr, |expr, token| {
            ast::Expr::Unop((&token.kind).into(), Box::new(expr))
        }))
    }
    fn expr10(&mut self) -> ParseResult<ast::Expr> {
        let match_array_access = |s: &mut Self, id| {
            let indexes = s.match_at_least_one(Self::expr, &TokenKind::Comma)?;
            s.expect(TokenKind::RBracket)?;
            Ok(ast::Expr::ArrayAccess(id, indexes))
        };
        let match_call = |s: &mut Self, id, make: fn(String, Vec<ast::Expr>) -> ast::Expr| {
            let args = s.match_zero_or_more_multiple(
                Self::expr10,
                &[
                    TokenKind::IdLower,
                    TokenKind::IdUpper,
                    TokenKind::Exclam,
                    TokenKind::IntLiteral,
                    TokenKind::FloatLiteral,
                    TokenKind::CharLiteral,
                    TokenKind::StringLiteral,
                    TokenKind::True,
                    TokenKind::False,
                    TokenKind::LParen,
                    TokenKind::Dim,
                    TokenKind::New,
                    TokenKind::Begin,
                    TokenKind::While,
                    TokenKind::For,
                    TokenKind::Match,
                ],
            )?;
            Ok(make(id, args))
        };
        let deref_cnt = self.accept_many_and_count(&TokenKind::Exclam);
        if deref_cnt > 0 {
            let inner_expr = if let Some(token) = self.accept(&TokenKind::IdLower) {
                let id = token.extract_string_value();
                self.expect(TokenKind::LBracket)?;
                match_array_access(self, id)?
            } else {
                self.expr_primary()?
            };
            Ok((0..deref_cnt).rfold(inner_expr, |expr, _| {
                ast::Expr::Unop((&TokenKind::Exclam).into(), Box::new(expr))
            }))
        } else {
            if let Some(token) = self.accept(&TokenKind::IdLower) {
                if self.accept(&TokenKind::LBracket).is_some() {
                    match_array_access(self, token.extract_string_value())
                } else {
                    match_call(self, token.extract_string_value(), ast::Expr::Call)
                }
            } else if let Some(token) = self.accept(&TokenKind::IdUpper) {
                match_call(self, token.extract_string_value(), ast::Expr::ConstrCall)
            } else {
                self.expr_primary()
            }
        }
    }
    fn expr_primary(&mut self) -> ParseResult<ast::Expr> {
        expect_any_of!(self,
            TokenKind::IntLiteral => |token: Token| Ok(ast::Expr::IntLiteral(token.extract_int_value())),
            TokenKind::FloatLiteral => |token: Token| Ok(ast::Expr::FloatLiteral(token.extract_float_value())),
            TokenKind::CharLiteral => |token: Token| Ok(ast::Expr::CharLiteral(token.extract_char_value())),
            TokenKind::StringLiteral => |token: Token| Ok(ast::Expr::StringLiteral(token.extract_string_value())),
            TokenKind::True => |_| Ok(ast::Expr::BoolLiteral(true)),
            TokenKind::False => |_| Ok(ast::Expr::BoolLiteral(false)),
            TokenKind::LParen => |_| {
                if self.accept(&TokenKind::RParen).is_some() {
                    Ok(ast::Expr::UnitLiteral)
                } else {
                    Ok(ast::Expr::maybe_tuple(
                        self.match_at_least_one(Self::expr, &TokenKind::Comma)?
                    ))
                }
            },
            TokenKind::Dim => |_| {
                let dim = self.accept(&TokenKind::IntLiteral).map_or(1, |token| token.extract_int_value());
                let id = self.expect(TokenKind::IdLower)?.extract_string_value();
                Ok(ast::Expr::Dim(id, dim))
            },
            TokenKind::New => |_| Ok(ast::Expr::New(self.r#type()?)),
            TokenKind::Begin => |_| {
                let expr = self.expr()?;
                self.expect(TokenKind::End)?;
                Ok(expr)
            },
            TokenKind::While => |_| {
                let cond = Box::new(self.expr()?);
                self.expect(TokenKind::Do)?;
                let body = Box::new(self.expr()?);
                Ok(ast::Expr::While(cond, body))
            },
            TokenKind::For => |_| {
                let id = self.expect(TokenKind::IdLower)?.extract_string_value();
                self.expect(TokenKind::Eq)?;
                let init = Box::new(self.expr()?);
                let to_downto = expect_any_of!(self,
                    TokenKind::To => |_| Ok(true),
                    TokenKind::Downto => |_| Ok(false)
                )?;
                let end = Box::new(self.expr()?);
                self.expect(TokenKind::Do)?;
                let body = Box::new(self.expr()?);
                self.expect(TokenKind::Done)?;
                Ok(ast::Expr::For(id, init, to_downto, end, body))
            },
            TokenKind::Match => |_| {
                let expr = Box::new(self.expr()?);
                self.expect(TokenKind::With)?;
                let clauses = self.match_at_least_one(Self::clause, &TokenKind::Bar)?;
                Ok(ast::Expr::Match(expr, clauses))
            }
        )
    }

    fn clause(&mut self) -> ParseResult<ast::Clause> {
        let pattern = self.pattern()?;
        self.expect(TokenKind::Arrow)?;
        let expr = self.expr()?;
        Ok(ast::Clause { pattern, expr })
    }
    fn pattern(&mut self) -> ParseResult<ast::Pattern> {
        expect_any_of!(self,
            TokenKind::Plus | TokenKind::Minus => |token: Token| {
                let mut integer = self.expect(TokenKind::IntLiteral)?.extract_int_value();
                if token.kind == TokenKind::Minus {
                    integer = -integer;
                }
                Ok(ast::Pattern::IntLiteral(integer))
            },
            TokenKind::PlusDot | TokenKind::MinusDot => |token: Token| {
                let mut float = self.expect(TokenKind::FloatLiteral)?.extract_float_value();
                if token.kind == TokenKind::MinusDot {
                    float = -float;
                }
                Ok(ast::Pattern::FloatLiteral(float))
            },
            TokenKind::CharLiteral => |_| {
                let char = self.expect(TokenKind::CharLiteral)?.extract_char_value();
                Ok(ast::Pattern::CharLiteral(char))
            },
            TokenKind::StringLiteral => |_| {
                let string = self.expect(TokenKind::StringLiteral)?.extract_string_value();
                Ok(ast::Pattern::StringLiteral(string))
            },
            TokenKind::IntLiteral => |_| {
                let integer = self.expect(TokenKind::IntLiteral)?.extract_int_value();
                Ok(ast::Pattern::IntLiteral(integer))
            },
            TokenKind::FloatLiteral => |_| {
                let float = self.expect(TokenKind::FloatLiteral)?.extract_float_value();
                Ok(ast::Pattern::FloatLiteral(float))
            },
            TokenKind::False => |_| {Ok(ast::Pattern::BoolLiteral(false))},
            TokenKind::True => |_| {Ok(ast::Pattern::BoolLiteral(true))},
            TokenKind::IdLower => |token: Token| {
                Ok(ast::Pattern::IdLower(token.extract_string_value()))
            },
            TokenKind::LParen => |_| {
                let patterns = self
                    .match_at_least_one(Self::pattern, &TokenKind::Comma)
                    .map(ast::Pattern::maybe_tuple)?;
                self.expect(TokenKind::RParen)?;
                Ok(patterns)
            },
            TokenKind::IdUpper => |token: Token| {
                let id = token.extract_string_value();
                let patterns = self.match_zero_or_more_multiple(Self::pattern, &[
                    TokenKind::Plus, TokenKind::Minus, TokenKind::PlusDot, TokenKind::MinusDot,
                    TokenKind::CharLiteral, TokenKind::StringLiteral, TokenKind::IntLiteral,
                    TokenKind::FloatLiteral, TokenKind::False, TokenKind::True, TokenKind::IdLower,
                    TokenKind::LParen, TokenKind::IdUpper
                ])?;
                Ok(ast::Pattern::IdUpper(id, patterns))
            }
        )
    }

    fn expect(&mut self, token_kind: TokenKind) -> ParseResult<Token> {
        self.accept(&token_kind).ok_or(ParseErr::UnexpectedToken(
            self.peek_token().cloned(),
            vec![token_kind],
        ))
    }
    fn accept_many_and_count(&mut self, token_kind: &TokenKind) -> usize {
        let mut cnt = 0;
        while self.accept(token_kind).is_some() {
            cnt += 1;
        }
        cnt
    }
    fn accept(&mut self, token_kind: &TokenKind) -> Option<Token> {
        if let Some(true) = self.peek_token().map(|t| &t.kind == (token_kind)) {
            self.consume_token()
        } else {
            None
        }
    }
    fn consume_token(&mut self) -> Option<Token> {
        while self.lexer.peek().map(|t| &t.kind) == Some(&TokenKind::COMMENT) {
            self.lexer.next();
        }
        self.lexer.next()
    }
    fn peek_token(&mut self) -> Option<&Token> {
        while self.lexer.peek().map(|t| &t.kind) == Some(&TokenKind::COMMENT) {
            self.lexer.next();
        }
        self.lexer.peek()
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
    fn match_zero_or_more_multiple<T>(
        &mut self,
        matcher: fn(&mut Self) -> ParseResult<T>,
        first_tokens: &[TokenKind],
    ) -> ParseResult<Vec<T>> {
        let mut vec: Vec<T> = Vec::new();
        while first_tokens
            .iter()
            .find(|&separator| Some(separator) == self.peek_token().map(|t| &t.kind))
            .is_some()
        {
            vec.push(matcher(self)?);
        }
        Ok(vec)
    }
    fn match_at_least_one_until<T>(
        &mut self,
        matcher: fn(&mut Self) -> ParseResult<T>,
        follow_tokens: &[TokenKind],
    ) -> ParseResult<Vec<T>> {
        let mut vec: Vec<T> = Vec::new();
        loop {
            vec.push(matcher(self)?);
            let peek_kind = self.peek_token().map(|t| &t.kind);
            if follow_tokens
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
