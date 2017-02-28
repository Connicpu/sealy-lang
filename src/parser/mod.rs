#![allow(unused_imports)]

use lexer;
use lexer::Lexer;

pub mod parser;
pub mod ast;
pub mod string_literal;

pub type Error<'i> = ::lalrpop_util::ParseError<lexer::Location,
                                                  (lexer::TokenType, &'i str),
                                                  lexer::LexicalError>;

// force monomorphization here
#[inline(never)]
pub fn parse(lexer: Lexer) -> Result<(), Error> {
    parser::parse_seal(lexer)
}
