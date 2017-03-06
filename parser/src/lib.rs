#![allow(unused_imports)]

extern crate lalrpop_util;
extern crate seal_lexer as lexer;
extern crate seal_symbols as sym;
extern crate seal_ast as ast;

use lexer::Lexer;
use sym::SymTable;

mod parser;
pub mod string_literal;
pub mod num_literal;

pub type Error<'i> = ::lalrpop_util::ParseError<lexer::Location,
                                                  (lexer::TokenType, &'i str),
                                                  lexer::LexicalError>;

// force monomorphization here
#[inline(never)]
pub fn parse<'i>(lexer: Lexer<'i>, symbols: &mut SymTable) -> Result<ast::Module, Error<'i>> {
    parser::parse_seal(symbols, lexer)
}
