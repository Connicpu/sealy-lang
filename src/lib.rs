#![feature(alloc, heap_api)]

extern crate alloc;
pub extern crate seal_lexer as lexer;
pub extern crate seal_parser as parser;
pub extern crate seal_symbols as sym;

pub mod vm;
