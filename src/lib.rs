#![feature(alloc, heap_api)]
extern crate alloc;

#[macro_use]
extern crate lazy_static;

extern crate unicode_xid;
extern crate lalrpop_util;

pub mod lexer;
pub mod parser;
pub mod vm;

