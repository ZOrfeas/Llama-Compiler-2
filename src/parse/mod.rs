pub mod ast;

use thiserror::Error;

use crate::{
    lex::token::{Token, TokenKind},
    long_peekable::{LongPeek, LongPeekableIterator},
};

use self::ast::Span;

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
    consumed_token_span: Span,
}

impl<L: Iterator<Item = Token>> Parser<L> {
    pub fn new(lexer: L) -> Self {
        Self {
            lexer: lexer.long_peekable(),
            consumed_token_span: Default::default(),
        }
    }

    pub fn program(&mut self) -> ParseResult<ast::Program> {
        let mut definitions: Vec<ast::def::Definition> = Vec::new();
        while self.accept(&TokenKind::EOF).is_none() {
            let definition = expect_any_of!(self,
                TokenKind::Let  => |_| self.letdef().map(ast::def::Definition::Let),
                TokenKind::Type => |_| self.typedef().map(ast::def::Definition::Type)
            )?;
            definitions.push(definition);
        }
        Ok(ast::Program { definitions })
    }
    fn letdef(&mut self) -> ParseResult<ast::def::Letdef> {
        let from = self.consumed_token_span.start.clone();
        Ok(ast::def::Letdef {
            rec: self.accept(&TokenKind::Rec).is_some(),
            defs: self.match_at_least_one(Self::def, &TokenKind::And)?,
            span: Span::new(from, self.consumed_token_span.end.clone()),
        })
    }
    fn def(&mut self) -> ParseResult<ast::def::Def> {
        expect_any_of!(self,
            TokenKind::IdLower => |token: Token| {
                let from = token.from.clone();
                let id = token.extract_value();
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
                let span = Span::new(from, self.consumed_token_span.end.clone());
                if pars.is_empty() {
                    Ok(ast::def::Def { id, type_, kind: ast::def::DefKind::Const{ expr }, span })
                } else {
                    Ok(ast::def::Def {
                        id, type_, span,
                        kind: ast::def::DefKind::Function {pars, expr},
                    })
                }
            },
            TokenKind::Mutable => |_| {
                let from = self.consumed_token_span.start.clone();
                let id = self.expect(TokenKind::IdLower)?.extract_value();
                let dims = if self.accept(&TokenKind::LBracket).is_some() {
                    let dims = self.match_at_least_one(Self::expr, &TokenKind::Comma)?;
                    self.expect(TokenKind::RBracket)?;
                    dims
                } else {
                    Vec::new()
                };
                let type_ = if self.accept(&TokenKind::Colon).is_some() {
                    Some(ast::annotation::TypeAnnotation::Ref(Box::new(self.r#type()?)))
                } else {
                    None
                };
                let span = Span::new(from, self.consumed_token_span.end.clone());
                if dims.is_empty() {
                    Ok(ast::def::Def { id, type_, kind: ast::def::DefKind::Variable, span })
                } else {
                    Ok(ast::def::Def{ id, type_, kind: ast::def::DefKind::Array{dims}, span })
                }
            }
        )
    }
    fn typedef(&mut self) -> ParseResult<ast::def::Typedef> {
        let from = self.consumed_token_span.start.clone();
        Ok(ast::def::Typedef {
            tdefs: self.match_at_least_one(Self::tdef, &TokenKind::And)?,
            span: Span::new(from, self.consumed_token_span.end.clone()),
        })
    }
    fn tdef(&mut self) -> ParseResult<ast::def::TDef> {
        let id = self.expect(TokenKind::IdLower)?;
        let from = id.from.clone();
        self.expect(TokenKind::Eq)?;
        Ok(ast::def::TDef {
            id: id.extract_value(),
            constrs: self.match_at_least_one(Self::constr, &TokenKind::Bar)?,
            span: Span::new(from, self.consumed_token_span.end.clone()),
        })
    }
    fn constr(&mut self) -> ParseResult<ast::def::Constr> {
        let id = self.expect(TokenKind::IdUpper)?.extract_value();
        let from = self.consumed_token_span.start.clone();
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
        Ok(ast::def::Constr {
            id,
            types,
            span: Span::new(from, self.consumed_token_span.end.clone()),
        })
    }
    fn par(&mut self) -> ParseResult<ast::def::Par> {
        expect_any_of!(self,
            TokenKind::IdLower => |token: Token| {
                let span = Span::new(token.from.clone(), token.to.clone());
                Ok(ast::def::Par {
                    id: token.extract_value(),
                    type_: None,
                    span
                })
            },
            TokenKind::LParen => |token: Token| {
                let from = token.from;
                let id = self.expect(TokenKind::IdLower)?.extract_value();
                self.expect(TokenKind::Colon)?;
                let type_ = self.r#type()?;
                self.expect(TokenKind::RParen)?;
                Ok(ast::def::Par {
                    id,
                    type_: Some(type_),
                    span: Span::new(from, self.consumed_token_span.end.clone()),
                })
            }
        )
    }

    fn r#type(&mut self) -> ParseResult<ast::annotation::TypeAnnotation> {
        let t1 = self.type_precedence_helper()?;
        if self.accept(&TokenKind::Arrow).is_some() {
            let lhs = Box::new(t1);
            let rhs = Box::new(self.r#type()?);
            Ok(ast::annotation::TypeAnnotation::Func { lhs, rhs })
        } else {
            Ok(t1)
        }
    }
    fn type_precedence_helper(&mut self) -> ParseResult<ast::annotation::TypeAnnotation> {
        let mut t = expect_any_of!(self,
            TokenKind::Unit | TokenKind::Int | TokenKind::Char
            | TokenKind::Bool | TokenKind::Float  => |token: Token| Ok((&token.kind).into()),
            TokenKind::LParen => |_| {
                let t = self
                    .match_at_least_one(Self::r#type, &TokenKind::Comma)
                    .map(ast::annotation::TypeAnnotation::maybe_tuple)?;
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
                    .map(|t| ast::annotation::TypeAnnotation::Array{ inner: Box::new(t), dim_cnt })
            },
            TokenKind::IdLower => |token: Token| {
                Ok(ast::annotation::TypeAnnotation::Custom{ id: token.extract_value() })
            }
        )?;
        // below loop handles type_recursion_helper non-terminal
        while self.accept(&TokenKind::Ref).is_some() {
            t = ast::annotation::TypeAnnotation::Ref(Box::new(t));
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
            let end = expr.span.end.clone();
            Ok(letdefs.into_iter().rfold(expr, |expr, letdef| {
                let expr = Box::new(expr);
                ast::expr::Expr {
                    span: Span::new(letdef.span.start.clone(), end.clone()),
                    kind: ast::expr::ExprKind::LetIn(ast::expr::LetIn { letdef, expr }),
                }
            }))
        }
    }
    fn expr1(&mut self) -> ParseResult<ast::expr::Expr> {
        let lhs = self.expr2()?;
        let exprs = self.match_zero_or_more(Self::expr, &TokenKind::Semicolon)?;
        Ok(exprs.into_iter().fold(lhs, |lhs, rhs| ast::expr::Expr {
            span: Span::new(lhs.span.start.clone(), rhs.span.end.clone()),
            kind: ast::expr::ExprKind::Binop(ast::expr::Binop {
                lhs: Box::new(lhs),
                op: (&TokenKind::Semicolon).into(),
                rhs: Box::new(rhs),
            }),
        }))
    }
    fn expr2(&mut self) -> ParseResult<ast::expr::Expr> {
        if let Some(token) = self.accept(&TokenKind::If) {
            let from = token.from;
            let cond = Box::new(self.expr()?);
            self.expect(TokenKind::Then)?;
            let then_body = Box::new(self.expr()?);
            let else_body = if self.accept(&TokenKind::Else).is_some() {
                Some(Box::new(self.expr()?))
            } else {
                None
            };
            Ok(ast::expr::Expr {
                span: Span::new(
                    from,
                    else_body
                        .as_ref()
                        .map(|e| e.span.end.clone())
                        .unwrap_or_else(|| then_body.span.end.clone()),
                ),
                kind: ast::expr::ExprKind::If(ast::expr::If {
                    cond,
                    then_body,
                    else_body,
                }),
            })
        } else {
            self.expr3()
        }
    }
    // TODO: Improve printing where possible. Think about error messages.
    fn expr3(&mut self) -> ParseResult<ast::expr::Expr> {
        let lhs = self.expr4()?;
        if let Some(token) = self.accept(&TokenKind::ColonEq) {
            let rhs = Box::new(self.expr3()?);
            Ok(ast::expr::Expr {
                span: Span::new(lhs.span.start.clone(), rhs.span.end.clone()),
                kind: ast::expr::ExprKind::Binop(ast::expr::Binop {
                    lhs: Box::new(lhs),
                    op: (&token.kind).into(),
                    rhs,
                }),
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
        const OPS: [Option<&TokenKind>; 8] = [
            Some(&TokenKind::Eq),
            Some(&TokenKind::LtGt),
            Some(&TokenKind::Lt),
            Some(&TokenKind::Gt),
            Some(&TokenKind::LEq),
            Some(&TokenKind::GEq),
            Some(&TokenKind::DblEq),
            Some(&TokenKind::ExclamEq),
        ];
        let lhs = self.expr7()?;
        if let Some(Some(op)) = OPS
            .iter()
            .find(|&t| t == &self.peek_token().map(|t| &t.kind))
        {
            self.consume_token();
            let rhs = Box::new(self.expr7()?);
            Ok(ast::expr::Expr {
                span: Span::new(lhs.span.start.clone(), rhs.span.end.clone()),
                kind: ast::expr::ExprKind::Binop(ast::expr::Binop {
                    lhs: Box::new(lhs),
                    op: (*op).into(),
                    rhs,
                }),
            })
        } else {
            Ok(lhs)
        }
    }
    fn expr7(&mut self) -> ParseResult<ast::expr::Expr> {
        const OPS: [Option<&TokenKind>; 2] = [
            Some(&TokenKind::Plus),
            Some(&TokenKind::Minus),
            // Some(&TokenKind::PlusDot),
            // Some(&TokenKind::MinusDot),
        ];
        let mut lhs = self.expr8()?;
        while let Some(Some(op)) = OPS
            .iter()
            .find(|&t| t == &self.peek_token().map(|t| &t.kind))
        {
            self.consume_token();
            let rhs = Box::new(self.expr8()?);
            lhs = ast::expr::Expr {
                span: Span::new(lhs.span.start.clone(), rhs.span.end.clone()),
                kind: ast::expr::ExprKind::Binop(ast::expr::Binop {
                    lhs: Box::new(lhs),
                    op: (*op).into(),
                    rhs,
                }),
            };
        }
        Ok(lhs)
    }
    fn expr8(&mut self) -> ParseResult<ast::expr::Expr> {
        const OPS: [Option<&TokenKind>; 3] = [
            Some(&TokenKind::Star),
            Some(&TokenKind::Slash),
            Some(&TokenKind::Mod),
            // Some(&TokenKind::StarDot),
            // Some(&TokenKind::SlashDot),
        ];
        let mut lhs = self.expr9()?;
        while let Some(Some(op)) = OPS
            .iter()
            .find(|&t| t == &self.peek_token().map(|t| &t.kind))
        {
            self.consume_token();
            let rhs = Box::new(self.expr9()?);
            lhs = ast::expr::Expr {
                span: Span::new(lhs.span.start.clone(), rhs.span.end.clone()),
                kind: ast::expr::ExprKind::Binop(ast::expr::Binop {
                    lhs: Box::new(lhs),
                    op: (*op).into(),
                    rhs,
                }),
            };
        }
        Ok(lhs)
    }
    fn expr9(&mut self) -> ParseResult<ast::expr::Expr> {
        let lhs = self.expr10()?;
        if let Some(token) = self.accept(&TokenKind::DblStar) {
            let rhs = Box::new(self.expr9()?);
            Ok(ast::expr::Expr {
                span: Span::new(lhs.span.start.clone(), rhs.span.end.clone()),
                kind: ast::expr::ExprKind::Binop(ast::expr::Binop {
                    lhs: Box::new(lhs),
                    op: (&token.kind).into(),
                    rhs,
                }),
            })
        } else {
            Ok(lhs)
        }
    }
    fn expr10(&mut self) -> ParseResult<ast::expr::Expr> {
        const OPS: [Option<&TokenKind>; 4] = [
            Some(&TokenKind::Plus),
            Some(&TokenKind::Minus),
            Some(&TokenKind::Not),
            Some(&TokenKind::Delete),
        ];
        let mut unops = Vec::new();
        while OPS.contains(&self.peek_token().map(|t| &t.kind)) {
            unops.push(
                self.consume_token()
                    .expect("peeeked token should be present"),
            );
        }
        let expr = self.expr11()?;
        Ok(unops
            .into_iter()
            .rfold(expr, |expr, token| ast::expr::Expr {
                span: Span::new(token.from.clone(), expr.span.end.clone()),
                kind: ast::expr::ExprKind::Unop(ast::expr::Unop {
                    op: (&token.kind).into(),
                    operand: Box::new(expr),
                }),
            }))
    }
    fn expr11(&mut self) -> ParseResult<ast::expr::Expr> {
        let match_array_access = |s: &mut Self, id: Token| {
            let from = id.from.clone();
            let indexes = s.match_at_least_one(Self::expr, &TokenKind::Comma)?;
            let to = s.expect(TokenKind::RBracket)?.to;
            Ok(ast::expr::Expr {
                span: Span::new(from, to),
                kind: ast::expr::ExprKind::ArrayAccess(ast::expr::ArrayAccess {
                    id: id.extract_value(),
                    indexes,
                }),
            })
        };
        let match_call =
            |s: &mut Self, id: Token, make: fn(ast::expr::Call) -> ast::expr::ExprKind| {
                let from = id.from.clone();
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
                let to = s.consumed_token_span.end.clone();
                Ok(ast::expr::Expr {
                    kind: make(ast::expr::Call {
                        id: id.extract_value(),
                        args,
                    }),
                    span: Span::new(from, to),
                })
            };
        let match_array_access_or_fn_call = |s: &mut Self, id| {
            if s.accept(&TokenKind::LBracket).is_some() {
                match_array_access(s, id)
            } else {
                match_call(s, id, ast::expr::ExprKind::Call)
            }
        };
        let deref_tokens = self.accept_many(&TokenKind::Exclam);
        if deref_tokens.len() > 0 {
            let inner_expr = if let Some(token) = self.accept(&TokenKind::IdLower) {
                // FIX. This does not allow for `!a` expressions
                // I think this is wrong in gerenal ?
                // *Note: Possibly fixed
                if self.accept(&TokenKind::LBracket).is_some() {
                    match_array_access(self, token)?
                } else {
                    let span = Span::new(token.from.clone(), token.to.clone());
                    ast::expr::Expr {
                        kind: ast::expr::ExprKind::Call(ast::expr::Call {
                            id: token.extract_value(),
                            args: Vec::new(),
                        }),
                        span,
                    }
                }
            } else {
                self.expr_primary()?
            };
            Ok(deref_tokens
                .into_iter()
                .rfold(inner_expr, |expr, deref_tok| ast::expr::Expr {
                    span: Span::new(deref_tok.from.clone(), expr.span.end.clone()),
                    kind: ast::expr::ExprKind::Unop(ast::expr::Unop {
                        op: (&TokenKind::Exclam).into(),
                        operand: Box::new(expr),
                    }),
                }))
        } else {
            if let Some(token) = self.accept(&TokenKind::IdLower) {
                match_array_access_or_fn_call(self, token)
            } else if let Some(token) = self.accept(&TokenKind::IdUpper) {
                match_call(self, token, ast::expr::ExprKind::ConstrCall)
            } else {
                self.expr_primary()
            }
        }
    }
    fn expr_primary(&mut self) -> ParseResult<ast::expr::Expr> {
        expect_any_of!(self,
            TokenKind::IntLiteral | TokenKind::FloatLiteral | TokenKind::CharLiteral
            | TokenKind::StringLiteral | TokenKind::True | TokenKind::False => |token: Token| {
                Ok(ast::expr::Expr::from_literal(token))
            },
            TokenKind::LParen => |token: Token| {
                let from = token.from;
                if let Some(rparen_token) = self.accept(&TokenKind::RParen) {
                    Ok(ast::expr::Expr {
                        span: Span::new(from, rparen_token.to),
                        kind: ast::expr::ExprKind::UnitLiteral,
                    })
                } else {
                    let mut retval = ast::expr::Expr::maybe_tuple(
                        self.match_at_least_one(Self::expr, &TokenKind::Comma)?
                    );
                    let to = self.expect(TokenKind::RParen)?.to;
                    retval.span = Span::new(from, to);
                    Ok(retval)
                }
            },
            TokenKind::Dim => |token: Token| {
                let from = token.from;
                let dim = self.accept(&TokenKind::IntLiteral).map_or(1, |token| token.extract_value());
                let (id_span, id) = self.expect(TokenKind::IdLower)?.into_span_and_value::<String>();
                Ok(ast::expr::Expr {
                    kind: ast::expr::ExprKind::Dim(ast::expr::Dim {id, dim}),
                    span: Span::new(from, id_span.end)
                })
            },
            TokenKind::New => |token: Token| {
                let from = token.from;
                Ok(ast::expr::Expr{
                    kind: ast::expr::ExprKind::New(self.r#type()?),
                    span: Span::new(from, self.consumed_token_span.end.clone())
                })
            },
            TokenKind::Begin => |token: Token| {
                let from = token.from;
                let mut expr = self.expr()?;
                let to = self.expect(TokenKind::End)?.to;
                expr.span = Span::new(from, to);
                Ok(expr)
            },
            TokenKind::While => |token: Token| {
                let from = token.from;
                let cond = Box::new(self.expr()?);
                self.expect(TokenKind::Do)?;
                let body = Box::new(self.expr()?);
                let to = self.expect(TokenKind::Done)?.to;
                Ok(ast::expr::Expr{
                    kind: ast::expr::ExprKind::While(ast::expr::While {cond, body}),
                    span: Span::new(from, to)
                })
            },
            TokenKind::For => |token: Token| {
                let span_from = token.from;
                let id = self.expect(TokenKind::IdLower)?.extract_value();
                self.expect(TokenKind::Eq)?;
                let from = Box::new(self.expr()?);
                let ascending = expect_any_of!(self,
                    TokenKind::To => |_| Ok(true),
                    TokenKind::Downto => |_| Ok(false)
                )?;
                let to = Box::new(self.expr()?);
                self.expect(TokenKind::Do)?;
                let body = Box::new(self.expr()?);
                let span_to = self.expect(TokenKind::Done)?.to;
                Ok(ast::expr::Expr{
                    kind: ast::expr::ExprKind::For(ast::expr::For {id, from, ascending, to, body}),
                    span: Span::new(span_from, span_to),
                })
            },
            TokenKind::Match => |token: Token| {
                let from = token.from;
                let to_match = Box::new(self.expr()?);
                self.expect(TokenKind::With)?;
                let clauses = self.match_at_least_one(Self::clause, &TokenKind::Bar)?;
                let to = self.expect(TokenKind::End)?.to;
                Ok(ast::expr::Expr{
                    kind: ast::expr::ExprKind::Match(ast::expr::Match {to_match, clauses}),
                    span: Span::new(from, to)
                })
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
                let mut integer = self.expect(TokenKind::IntLiteral)?.extract_value::<i32>();
                if token.kind == TokenKind::Minus {
                    integer = -integer;
                }
                Ok(ast::expr::Pattern::IntLiteral(integer))
            },
            TokenKind::CharLiteral => |token: Token| {
                let char = token.extract_value();
                Ok(ast::expr::Pattern::CharLiteral(char))
            },
            TokenKind::StringLiteral => |token: Token| {
                let string = token.extract_value();
                Ok(ast::expr::Pattern::StringLiteral(string))
            },
            TokenKind::IntLiteral => |token: Token| {
                let integer = token.extract_value();
                Ok(ast::expr::Pattern::IntLiteral(integer))
            },
            TokenKind::FloatLiteral => |token: Token| {
                let float = token.extract_value();
                Ok(ast::expr::Pattern::FloatLiteral(float))
            },
            TokenKind::False => |_| {Ok(ast::expr::Pattern::BoolLiteral(false))},
            TokenKind::True => |_| {Ok(ast::expr::Pattern::BoolLiteral(true))},
            TokenKind::IdLower => |token: Token| {
                Ok(ast::expr::Pattern::IdLower(token.extract_value()))
            },
            TokenKind::LParen => |_| {
                let patterns = self
                    .match_at_least_one(Self::pattern, &TokenKind::Comma)
                    .map(ast::expr::Pattern::maybe_tuple)?;
                self.expect(TokenKind::RParen)?;
                Ok(patterns)
            },
            TokenKind::IdUpper => |token: Token| {
                let id = token.extract_value();
                let args = self.match_zero_or_more_multiple(Self::pattern, &[
                    TokenKind::Plus, TokenKind::Minus,
                    TokenKind::CharLiteral, TokenKind::StringLiteral, TokenKind::IntLiteral,
                    TokenKind::FloatLiteral, TokenKind::False, TokenKind::True, TokenKind::IdLower,
                    TokenKind::LParen, TokenKind::IdUpper
                ])?;
                Ok(ast::expr::Pattern::IdUpper{id, args})
            }
        )
    }

    fn expect(&mut self, token_kind: TokenKind) -> ParseResult<Token> {
        self.accept(&token_kind)
            .ok_or_else(|| ParseErr::UnexpectedToken(self.peek_token().cloned(), vec![token_kind]))
    }
    // fn accept_many_and_count(&mut self, token_kind: &TokenKind) -> usize {
    //     let mut cnt = 0;
    //     while self.accept(token_kind).is_some() {
    //         cnt += 1;
    //     }
    //     cnt
    // }
    fn accept_many(&mut self, token_kind: &TokenKind) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.accept(token_kind) {
            tokens.push(token);
        }
        tokens
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
        let tok = self.lexer.next();
        tok.as_ref()
            .map(|t| self.consumed_token_span = Span::new(t.from.clone(), t.to.clone()));
        tok
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
#[derive(Error, Debug)]
pub enum ParseErr {
    #[error(
        "{}, expected any of {{{}}}",
        .0.as_ref().map(|t| format!("{}: Unexpected token \"{}\"", t.from, t.kind))
            .unwrap_or("Unexpected end of input".to_string()),
        .1.iter().map(|t| format!("\"{}\"", t)).collect::<Vec<_>>().join(", ")
    )]
    UnexpectedToken(Option<Token>, Vec<TokenKind>),
}
