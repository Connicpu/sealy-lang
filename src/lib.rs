#![feature(alloc, heap_api)]

extern crate alloc;
pub extern crate seal_lexer as lexer;
pub extern crate seal_symbols as sym;
pub extern crate seal_ast as ast;
pub extern crate seal_parser as parser;
pub extern crate seal_parser_passes as parser_passes;
pub extern crate seal_ir as ir;

pub mod vm;
