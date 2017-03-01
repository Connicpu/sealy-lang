#![allow(unused_imports)]

extern crate lalrpop_util;
extern crate seal_lexer as lexer;

use lexer::Lexer;

mod parser;
pub mod ast;
pub mod string_literal;

pub type Error<'i> = ::lalrpop_util::ParseError<lexer::Location,
                                                  (lexer::TokenType, &'i str),
                                                  lexer::LexicalError>;

// force monomorphization here
#[inline(never)]
pub fn parse(lexer: Lexer) -> Result<ast::Module, Error> {
    parser::parse_seal(lexer)
}
