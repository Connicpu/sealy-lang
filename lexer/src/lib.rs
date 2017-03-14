#![feature(untagged_unions)]

#[macro_use]
extern crate lazy_static;

extern crate unicode_xid;

use std::iter::Peekable;
use std::str::CharIndices;

pub mod dfa;
pub mod emoji;
pub mod keywords;
pub mod seal_dfa;
pub mod c_api;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
pub type Tok<'input> = (TokenType, &'input str);

/// The lexer
#[derive(Clone)]
pub struct Lexer<'input> {
    inner: Peekable<WhitespaceStripper<'input>>,
    prev: Option<TokenType>,
    paren_lvl: Vec<i32>,
}

#[derive(Clone)]
pub struct WhitespaceStripper<'input> {
    inner: InnerLexer<'input>,
}

#[derive(Clone)]
pub struct InnerLexer<'input> {
    source: &'input str,
    chars: Peekable<CharIndices<'input>>,
    loc: Location,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let inner = InnerLexer {
            source: input,
            chars: input.char_indices().peekable(),
            loc: Location { index: 0 },
        };

        let inner = WhitespaceStripper { inner: inner };

        Lexer {
            inner: inner.peekable(),
            prev: None,
            paren_lvl: vec![0],
        }
    }
}

lazy_static!{
    static ref SEAL_DFA: dfa::Dfa<TokenType, char> = seal_dfa::create_dfa();
}

impl<'input> Iterator for InnerLexer<'input> {
    type Item = Spanned<Tok<'input>, Location, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let dfa: &dfa::Dfa<TokenType, char> = &*SEAL_DFA;

        let mut initial_iter = self.chars.clone();

        let start = self.loc;
        let mut node = dfa.root();
        let mut last_accepting = None;
        let &(_, first) = match self.chars.peek() {
            Some(c) => c,
            None => return None,
        };

        loop {
            let (i, c) = match self.chars.next() {
                None => break,
                Some(i) => i,
            };

            self.loc.index = i as u32;

            if let Some(next) = dfa.next(node, &c) {
                if let Some(&state) = dfa.state(next) {
                    let mut end = self.loc;
                    end.index = (i + c.len_utf8()) as u32;

                    let span = &self.source[start.index as usize..end.index as usize];

                    let state = if state == TokenType::Whitespace && span.contains('\n') {
                        TokenType::NewLine
                    } else {
                        state
                    };

                    let tok = Token {
                        kind: state,
                        span: span,
                    };
                    last_accepting = Some((tok, end, self.chars.clone()));
                }

                node = next;
            } else {
                break;
            }
        }

        Some(match last_accepting {
            Some((mut tok, loc, iter)) => {
                self.loc = loc;
                self.chars = iter;
                if tok.kind == TokenType::Identifier {
                    if let Some(tt) = keywords::match_keyword(tok.span) {
                        tok.kind = tt;
                    }
                }
                Ok((start, (tok.kind, tok.span), loc))
            }
            None => {
                let (i, _) = initial_iter.next().unwrap();
                self.loc = start;
                self.loc.index = i as u32;
                self.chars = initial_iter;
                Err(LexicalError::Unexpected(first, start))
            }
        })
    }
}

impl<'input> Iterator for WhitespaceStripper<'input> {
    type Item = Spanned<Tok<'input>, Location, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(t) = self.inner.next() {
            if let Ok((_, (tt, _), _)) = t {
                match tt {
                    TokenType::Whitespace => continue,
                    TokenType::Comment => continue,
                    _ => (),
                }
            }
            return Some(t);
        }

        None
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, Location, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(t) = self.inner.next() {
            if let Ok((left, (tt, _), _)) = t {
                match tt {
                    TokenType::NewLine => {
                        if *self.paren_lvl.last().unwrap() > 0 {
                            continue;
                        }
                        if let Some(prev) = self.prev {
                            match prev {
                                TokenType::Dot => continue,
                                TokenType::Scope => continue,
                                TokenType::Arrow => continue,
                                TokenType::FatArrow => continue,
                                TokenType::NewLine => continue,
                                TokenType::OpenCurly => continue,
                                TokenType::Semicolon => continue,
                                TokenType::Comma => continue,

                                TokenType::LogicalAnd => continue,
                                TokenType::LogicalOr => continue,
                                TokenType::LogicalAndAssign => continue,
                                TokenType::LogicalOrAssign => continue,
                                TokenType::Add => continue,
                                TokenType::Sub => continue,
                                TokenType::Mul => continue,
                                TokenType::Div => continue,
                                TokenType::Rem => continue,
                                TokenType::DivRem => continue,
                                TokenType::Mod => continue,
                                TokenType::BitAnd => continue,
                                TokenType::BitOr => continue,
                                TokenType::BitXor => continue,
                                TokenType::Shl => continue,
                                TokenType::Shr => continue,
                                TokenType::LShr => continue,
                                TokenType::Assign => continue,
                                TokenType::AddAssign => continue,
                                TokenType::SubAssign => continue,
                                TokenType::MulAssign => continue,
                                TokenType::DivAssign => continue,
                                TokenType::RemAssign => continue,
                                TokenType::ModAssign => continue,
                                TokenType::BitAndAssign => continue,
                                TokenType::BitOrAssign => continue,
                                TokenType::BitXorAssign => continue,
                                TokenType::ShlAssign => continue,
                                TokenType::ShrAssign => continue,
                                TokenType::LShrAssign => continue,
                                _ => (),
                            }
                        }
                        if let Some(next) = self.inner.peek() {
                            if let Ok((_, (tt, _), _)) = *next {
                                match tt {
                                    TokenType::Dot => continue,
                                    TokenType::Scope => continue,
                                    TokenType::Arrow => continue,
                                    TokenType::FatArrow => continue,
                                    TokenType::Else => continue,
                                    TokenType::Semicolon => continue,

                                    TokenType::LogicalAnd => continue,
                                    TokenType::LogicalOr => continue,
                                    TokenType::LogicalAndAssign => continue,
                                    TokenType::LogicalOrAssign => continue,
                                    TokenType::Assign => continue,
                                    TokenType::AddAssign => continue,
                                    TokenType::SubAssign => continue,
                                    TokenType::MulAssign => continue,
                                    TokenType::DivAssign => continue,
                                    TokenType::RemAssign => continue,
                                    TokenType::ModAssign => continue,
                                    TokenType::BitAndAssign => continue,
                                    TokenType::BitOrAssign => continue,
                                    TokenType::BitXorAssign => continue,
                                    TokenType::ShlAssign => continue,
                                    TokenType::ShrAssign => continue,
                                    TokenType::LShrAssign => continue,
                                    _ => (),
                                }
                            }
                        }
                    }
                    TokenType::OpenParen | TokenType::OpenBracket => {
                        *self.paren_lvl.last_mut().unwrap() += 1;
                    }
                    TokenType::CloseParen | TokenType::CloseBracket => {
                        *self.paren_lvl.last_mut().unwrap() -= 1;
                    }
                    TokenType::OpenCurly => {
                        self.paren_lvl.push(0);
                    }
                    TokenType::CloseCurly => {
                        if self.paren_lvl.len() == 1 {
                            return Some(Err(LexicalError::TooManyCloseCurlies(left)));
                        }
                        self.paren_lvl.pop();
                    }
                    _ => (),
                }

                self.prev = Some(tt);
            }

            return Some(t);
        }

        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Location {
    pub index: u32,
}

#[derive(Debug, Clone)]
pub enum LexicalError {
    Unexpected(char, Location),
    TooManyCloseCurlies(Location),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Token<'input> {
    pub kind: TokenType,
    pub span: &'input str,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum TokenType {
    Identifier = 0,
    Whitespace,
    NewLine,

    Comment,
    DocComment,
    ModuleDocComment,

    IntLiteral,
    HexLiteral,
    OctLiteral,
    BinLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    Label,

    Break,
    Const,
    Continue,
    Else,
    Enum,
    Extern,
    Function,
    For,
    If,
    Impl,
    Impls,
    In,
    Let,
    Loop,
    Match,
    Mod,
    New,
    Nil,
    Return,
    SelfKw,
    Struct,
    Throw,
    Trait,
    Type,
    Use,
    While,

    OpenCurly,
    CloseCurly,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Semicolon,
    Colon,
    Comma,
    Question,
    Dot,
    Scope,
    Arrow,
    FatArrow,

    RangeExclusive,
    RangeInclusive,

    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,

    Not,
    Tilde,
    At,

    LogicalAnd,
    LogicalOr,

    LogicalAndAssign,
    LogicalOrAssign,

    Add,
    Sub,
    Mul,
    Div,
    Rem,
    DivRem,

    BitAnd,
    BitOr,
    BitXor,

    Shl,
    Shr,
    LShr,

    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    ModAssign,

    BitAndAssign,
    BitOrAssign,
    BitXorAssign,

    ShlAssign,
    ShrAssign,
    LShrAssign,
}
