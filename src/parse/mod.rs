#![allow(unused_parens)]

use log::debug;

use crate::{
    lex::token::{Token, TokenKind},
    long_peekable::{LongPeek, LongPeekableIterator},
};

pub mod ast;
macro_rules! expect_any_of {
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

    pub fn program(&mut self) -> ParseResult<ast::Program> {
        let mut definitions: Vec<ast::def::Definition> = Vec::new();
        while self.accept(&TokenKind::EOF).is_none() {
            let definition = expect_any_of!(self,
                TokenKind::Let => |_| self.letdef().map(ast::def::Definition::Let),
                TokenKind::Type => |_| self.typedef().map(ast::def::Definition::Type)
            )?;
            definitions.push(definition);
        }
        Ok(ast::Program { definitions })
    }
    fn letdef(&mut self) -> ParseResult<ast::def::Letdef> {
        Ok(ast::def::Letdef {
            rec: self.accept(&TokenKind::Rec).is_some(),
            defs: self.match_at_least_one(Self::def, &TokenKind::And)?,
        })
    }
    fn def(&mut self) -> ParseResult<ast::def::Def> {
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
                    Ok(ast::def::Def::Const(ast::def::ConstDef { id, type_, expr }))
                } else {
                    Ok(ast::def::Def::Function(ast::def::FunctionDef {
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
                    Ok(ast::def::Def::Variable(ast::def::VariableDef { id, type_ }))
                } else {
                    Ok(ast::def::Def::Array(ast::def::ArrayDef { id, type_, dims }))
                }
            }
        )
    }
    fn typedef(&mut self) -> ParseResult<ast::def::Typedef> {
        Ok(ast::def::Typedef {
            tdefs: self.match_at_least_one(Self::tdef, &TokenKind::And)?,
        })
    }
    fn tdef(&mut self) -> ParseResult<ast::def::TDef> {
        let id = self.expect(TokenKind::IdLower)?;
        self.expect(TokenKind::Eq)?;
        Ok(ast::def::TDef {
            id: id.extract_string_value(),
            constrs: self.match_at_least_one(Self::constr, &TokenKind::Bar)?,
        })
    }
    fn constr(&mut self) -> ParseResult<ast::def::Constr> {
        let id = self.expect(TokenKind::IdUpper)?.extract_string_value();
        let types = if self.accept(&TokenKind::Of).is_some() {
            self.match_at_least_one_until(
                Self::r#type,
                &[
                    TokenKind::Bar,
                    TokenKind::And,
                    TokenKind::Let,
                    TokenKind::Type,
                    TokenKind::EOF,
                ],
            )?
        } else {
            Vec::new()
        };
        Ok(ast::def::Constr { id: id, types })
    }
    fn par(&mut self) -> ParseResult<ast::def::Par> {
        expect_any_of!(self,
            TokenKind::IdLower => |token: Token| {
                Ok(ast::def::Par {
                    id: token.extract_string_value(),
                    type_: None,
                })
            },
            TokenKind::LParen => |_| {
                let id = self.expect(TokenKind::IdLower)?.extract_string_value();
                self.expect(TokenKind::Colon)?;
                let type_ = self.r#type()?;
                self.expect(TokenKind::RParen)?;
                Ok(ast::def::Par {
                    id,
                    type_: Some(type_),
                })
            }
        )
    }

    fn r#type(&mut self) -> ParseResult<ast::annotation::Type> {
        let t1 = self.type_precedence_helper()?;
        if self.accept(&TokenKind::Arrow).is_some() {
            let lhs = Box::new(t1);
            let rhs = Box::new(self.r#type()?);
            Ok(ast::annotation::Type::Func { lhs, rhs })
        } else {
            Ok(t1)
        }
    }
    fn type_precedence_helper(&mut self) -> ParseResult<ast::annotation::Type> {
        let mut t = expect_any_of!(self,
            TokenKind::Unit | TokenKind::Int | TokenKind::Char
            | TokenKind::Bool | TokenKind::Float  => |token: Token| Ok((&token.kind).into()),
            TokenKind::LParen => |_| {
                let t = self
                    .match_at_least_one(Self::r#type, &TokenKind::Comma)
                    .map(ast::annotation::Type::maybe_tuple)?;
                self.expect(TokenKind::RParen)?;
                Ok(t)
            },
            TokenKind::Array => |_| {
                let dim_cnt = if self.accept(&TokenKind::LBracket).is_some() {
                    self.expect(TokenKind::Star)?;
                    let mut dim_cnt = 1;
                    while self.accept(&TokenKind::Comma).is_some() {
                        self.expect(TokenKind::Star)?;
                        dim_cnt += 1;
                    }
                    self.expect(TokenKind::RBracket)?;
                    dim_cnt
                } else {
                    1
                };
                self.expect(TokenKind::Of)?;
                self.type_precedence_helper()
                    .map(|t| ast::annotation::Type::Array{ inner: Box::new(t), dim_cnt })
            },
            TokenKind::IdLower => |token: Token| {
                Ok(ast::annotation::Type::Custom{ id: token.extract_string_value() })
            }
        )?;
        // below loop handles type_recursion_helper non-terminal
        while self.accept(&TokenKind::Ref).is_some() {
            t = ast::annotation::Type::Ref(Box::new(t));
        }
        Ok(t)
    }
    #[inline(always)]
    fn expr(&mut self) -> ParseResult<ast::expr::Expr> {
        self.expr0()
    }
    fn expr0(&mut self) -> ParseResult<ast::expr::Expr> {
        let mut letdefs = Vec::new();
        while self.accept(&TokenKind::Let).is_some() {
            letdefs.push(self.letdef()?);
            self.expect(TokenKind::In)?;
        }
        if letdefs.is_empty() {
            self.expr1()
        } else {
            let expr = self.expr1()?;
            Ok(letdefs.into_iter().rfold(expr, |expr, letdef| {
                let expr = Box::new(expr);
                ast::expr::Expr::LetIn { letdef, expr }
            }))
        }
    }
    fn expr1(&mut self) -> ParseResult<ast::expr::Expr> {
        let lhs = self.expr2()?;
        let exprs = self.match_zero_or_more(Self::expr, &TokenKind::Semicolon)?;
        Ok(exprs
            .into_iter()
            .fold(lhs, |lhs, rhs| ast::expr::Expr::Binop {
                lhs: Box::new(lhs),
                op: (&TokenKind::Semicolon).into(),
                rhs: Box::new(rhs),
            }))
    }
    fn expr2(&mut self) -> ParseResult<ast::expr::Expr> {
        if self.accept(&TokenKind::If).is_some() {
            let cond = Box::new(self.expr()?);
            self.expect(TokenKind::Then)?;
            let then_body = Box::new(self.expr()?);
            let else_body = if self.accept(&TokenKind::Else).is_some() {
                Some(Box::new(self.expr()?))
            } else {
                None
            };
            Ok(ast::expr::Expr::If {
                cond,
                then_body,
                else_body,
            })
        } else {
            self.expr3()
        }
    }
    // TODO: Improve printing where possible.
    // TODO: Think about error messages.
    fn expr3(&mut self) -> ParseResult<ast::expr::Expr> {
        let lhs = self.expr4()?;
        if let Some(token) = self.accept(&TokenKind::ColonEq) {
            let rhs = Box::new(self.expr3()?);
            Ok(ast::expr::Expr::Binop {
                lhs: Box::new(lhs),
                op: (&token.kind).into(),
                rhs,
            })
        } else {
            Ok(lhs)
        }
    }
    fn expr4(&mut self) -> ParseResult<ast::expr::Expr> {
        self.match_at_least_one(Self::expr5, &TokenKind::DblBar)
            .map(|v| ast::expr::Expr::left_assoc_from_vec(v, &TokenKind::DblBar))
    }
    fn expr5(&mut self) -> ParseResult<ast::expr::Expr> {
        self.match_at_least_one(Self::expr6, &TokenKind::DblAmpersand)
            .map(|v| ast::expr::Expr::left_assoc_from_vec(v, &TokenKind::DblAmpersand))
    }
    fn expr6(&mut self) -> ParseResult<ast::expr::Expr> {
        let lhs = self.expr7()?;
        if let Some(token) = self
            .accept(&TokenKind::Eq)
            .or_else(|| self.accept(&TokenKind::Eq))
            .or_else(|| self.accept(&TokenKind::LtGt))
            .or_else(|| self.accept(&TokenKind::Lt))
            .or_else(|| self.accept(&TokenKind::Gt))
            .or_else(|| self.accept(&TokenKind::LEq))
            .or_else(|| self.accept(&TokenKind::GEq))
            .or_else(|| self.accept(&TokenKind::DblEq))
            .or_else(|| self.accept(&TokenKind::ExclamEq))
        {
            let rhs = Box::new(self.expr7()?);
            Ok(ast::expr::Expr::Binop {
                lhs: Box::new(lhs),
                op: (&token.kind).into(),
                rhs,
            })
        } else {
            Ok(lhs)
        }
    }
    fn expr7(&mut self) -> ParseResult<ast::expr::Expr> {
        let mut lhs = self.expr8()?;
        while let Some(token) = self
            .accept(&TokenKind::Plus)
            .or_else(|| self.accept(&TokenKind::Minus))
            .or_else(|| self.accept(&TokenKind::PlusDot))
            .or_else(|| self.accept(&TokenKind::MinusDot))
        {
            let rhs = Box::new(self.expr8()?);
            lhs = ast::expr::Expr::Binop {
                lhs: Box::new(lhs),
                op: (&token.kind).into(),
                rhs,
            };
        }
        Ok(lhs)
    }
    fn expr8(&mut self) -> ParseResult<ast::expr::Expr> {
        let mut lhs = self.expr9()?;
        while let Some(token) = self
            .accept(&TokenKind::Star)
            .or_else(|| self.accept(&TokenKind::Slash))
            .or_else(|| self.accept(&TokenKind::Mod))
            .or_else(|| self.accept(&TokenKind::StarDot))
            .or_else(|| self.accept(&TokenKind::SlashDot))
        {
            let rhs = Box::new(self.expr9()?);
            lhs = ast::expr::Expr::Binop {
                lhs: Box::new(lhs),
                op: (&token.kind).into(),
                rhs,
            };
        }
        Ok(lhs)
    }
    fn expr9(&mut self) -> ParseResult<ast::expr::Expr> {
        let lhs = self.expr10()?;
        if let Some(token) = self.accept(&TokenKind::DblStar) {
            let rhs = Box::new(self.expr9()?);
            Ok(ast::expr::Expr::Binop {
                lhs: Box::new(lhs),
                op: (&token.kind).into(),
                rhs,
            })
        } else {
            Ok(lhs)
        }
    }
    fn expr10(&mut self) -> ParseResult<ast::expr::Expr> {
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
        let expr = self.expr11()?;
        Ok(unops
            .into_iter()
            .rfold(expr, |expr, token| ast::expr::Expr::Unop {
                op: (&token.kind).into(),
                operand: Box::new(expr),
            }))
    }
    fn expr11(&mut self) -> ParseResult<ast::expr::Expr> {
        let match_array_access = |s: &mut Self, id| {
            let indexes = s.match_at_least_one(Self::expr, &TokenKind::Comma)?;
            s.expect(TokenKind::RBracket)?;
            Ok(ast::expr::Expr::ArrayAccess { id, indexes })
        };
        let match_call =
            |s: &mut Self, id, make: fn(String, Vec<ast::expr::Expr>) -> ast::expr::Expr| {
                let args = s.match_zero_or_more_multiple(
                    Self::expr11,
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
        let match_array_access_or_call = |s: &mut Self, id, make| {
            if s.accept(&TokenKind::LBracket).is_some() {
                match_array_access(s, id)
            } else {
                match_call(s, id, make)
            }
        };
        let deref_cnt = self.accept_many_and_count(&TokenKind::Exclam);
        if deref_cnt > 0 {
            let inner_expr = if let Some(token) = self.accept(&TokenKind::IdLower) {
                let id = token.extract_string_value();
                match_array_access_or_call(self, id, |id, args| ast::expr::Expr::Call { id, args })?
            } else {
                self.expr_primary()?
            };
            Ok(
                (0..deref_cnt).rfold(inner_expr, |expr, _| ast::expr::Expr::Unop {
                    op: (&TokenKind::Exclam).into(),
                    operand: Box::new(expr),
                }),
            )
        } else {
            if let Some(token) = self.accept(&TokenKind::IdLower) {
                let id = token.extract_string_value();
                match_array_access_or_call(self, id, |id, args| ast::expr::Expr::Call { id, args })
            } else if let Some(token) = self.accept(&TokenKind::IdUpper) {
                match_call(self, token.extract_string_value(), |id, args| {
                    ast::expr::Expr::ConstrCall { id, args }
                })
            } else {
                self.expr_primary()
            }
        }
    }
    fn expr_primary(&mut self) -> ParseResult<ast::expr::Expr> {
        expect_any_of!(self,
            TokenKind::IntLiteral => |token: Token| Ok(ast::expr::Expr::IntLiteral(token.extract_int_value())),
            TokenKind::FloatLiteral => |token: Token| Ok(ast::expr::Expr::FloatLiteral(token.extract_float_value())),
            TokenKind::CharLiteral => |token: Token| Ok(ast::expr::Expr::CharLiteral(token.extract_char_value())),
            TokenKind::StringLiteral => |token: Token| Ok(ast::expr::Expr::StringLiteral(token.extract_string_value())),
            TokenKind::True => |_| Ok(ast::expr::Expr::BoolLiteral(true)),
            TokenKind::False => |_| Ok(ast::expr::Expr::BoolLiteral(false)),
            TokenKind::LParen => |_| {
                if self.accept(&TokenKind::RParen).is_some() {
                    Ok(ast::expr::Expr::UnitLiteral)
                } else {
                    let retval = Ok(ast::expr::Expr::maybe_tuple(
                        self.match_at_least_one(Self::expr, &TokenKind::Comma)?
                    ));
                    self.expect(TokenKind::RParen)?;
                    retval
                }
            },
            TokenKind::Dim => |_| {
                let dim = self.accept(&TokenKind::IntLiteral).map_or(1, |token| token.extract_int_value());
                let id = self.expect(TokenKind::IdLower)?.extract_string_value();
                Ok(ast::expr::Expr::Dim{id, dim})
            },
            TokenKind::New => |_| Ok(ast::expr::Expr::New(self.r#type()?)),
            TokenKind::Begin => |_| {
                let expr = self.expr()?;
                self.expect(TokenKind::End)?;
                Ok(expr)
            },
            TokenKind::While => |_| {
                let cond = Box::new(self.expr()?);
                self.expect(TokenKind::Do)?;
                let body = Box::new(self.expr()?);
                self.expect(TokenKind::Done)?;
                Ok(ast::expr::Expr::While{cond, body})
            },
            TokenKind::For => |_| {
                let id = self.expect(TokenKind::IdLower)?.extract_string_value();
                self.expect(TokenKind::Eq)?;
                let from = Box::new(self.expr()?);
                let ascending = expect_any_of!(self,
                    TokenKind::To => |_| Ok(true),
                    TokenKind::Downto => |_| Ok(false)
                )?;
                let to = Box::new(self.expr()?);
                self.expect(TokenKind::Do)?;
                let body = Box::new(self.expr()?);
                self.expect(TokenKind::Done)?;
                Ok(ast::expr::Expr::For{id, from, ascending, to, body})
            },
            TokenKind::Match => |_| {
                let to_match = Box::new(self.expr()?);
                self.expect(TokenKind::With)?;
                let clauses = self.match_at_least_one(Self::clause, &TokenKind::Bar)?;
                self.expect(TokenKind::End)?;
                Ok(ast::expr::Expr::Match{to_match, clauses})
            }
        )
    }

    fn clause(&mut self) -> ParseResult<ast::expr::Clause> {
        let pattern = self.pattern()?;
        self.expect(TokenKind::Arrow)?;
        let expr = self.expr()?;
        Ok(ast::expr::Clause { pattern, expr })
    }
    fn pattern(&mut self) -> ParseResult<ast::expr::Pattern> {
        expect_any_of!(self,
            TokenKind::Plus | TokenKind::Minus => |token: Token| {
                let mut integer = self.expect(TokenKind::IntLiteral)?.extract_int_value();
                if token.kind == TokenKind::Minus {
                    integer = -integer;
                }
                Ok(ast::expr::Pattern::IntLiteral(integer))
            },
            TokenKind::PlusDot | TokenKind::MinusDot => |token: Token| {
                let mut float = self.expect(TokenKind::FloatLiteral)?.extract_float_value();
                if token.kind == TokenKind::MinusDot {
                    float = -float;
                }
                Ok(ast::expr::Pattern::FloatLiteral(float))
            },
            TokenKind::CharLiteral => |token: Token| {
                let char = token.extract_char_value();
                Ok(ast::expr::Pattern::CharLiteral(char))
            },
            TokenKind::StringLiteral => |token: Token| {
                let string = token.extract_string_value();
                Ok(ast::expr::Pattern::StringLiteral(string))
            },
            TokenKind::IntLiteral => |token: Token| {
                let integer = token.extract_int_value();
                Ok(ast::expr::Pattern::IntLiteral(integer))
            },
            TokenKind::FloatLiteral => |token: Token| {
                let float = token.extract_float_value();
                Ok(ast::expr::Pattern::FloatLiteral(float))
            },
            TokenKind::False => |_| {Ok(ast::expr::Pattern::BoolLiteral(false))},
            TokenKind::True => |_| {Ok(ast::expr::Pattern::BoolLiteral(true))},
            TokenKind::IdLower => |token: Token| {
                Ok(ast::expr::Pattern::IdLower(token.extract_string_value()))
            },
            TokenKind::LParen => |_| {
                let patterns = self
                    .match_at_least_one(Self::pattern, &TokenKind::Comma)
                    .map(ast::expr::Pattern::maybe_tuple)?;
                self.expect(TokenKind::RParen)?;
                Ok(patterns)
            },
            TokenKind::IdUpper => |token: Token| {
                let id = token.extract_string_value();
                let args = self.match_zero_or_more_multiple(Self::pattern, &[
                    TokenKind::Plus, TokenKind::Minus, TokenKind::PlusDot, TokenKind::MinusDot,
                    TokenKind::CharLiteral, TokenKind::StringLiteral, TokenKind::IntLiteral,
                    TokenKind::FloatLiteral, TokenKind::False, TokenKind::True, TokenKind::IdLower,
                    TokenKind::LParen, TokenKind::IdUpper
                ])?;
                Ok(ast::expr::Pattern::IdUpper{id, args})
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
            if let Some(peek_kind) = self.peek_token().map(|t| &t.kind) {
                if follow_tokens.contains(peek_kind) {
                    break;
                }
            } else {
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
