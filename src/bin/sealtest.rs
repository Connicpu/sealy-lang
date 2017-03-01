extern crate sealy_lang;

use sealy_lang::lexer::Lexer;
use sealy_lang::parser;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let script = env::args().nth(1);
    let script = script.as_ref().map(|s| &s[..]);
    let script = script.unwrap_or("scripts/parsetest.seal");

    let mut file = File::open(script).unwrap();
    let mut buf = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut buf).unwrap();
    drop(file);
    let input = &buf[..];

    // Wrap the input in our lexer
    let lexer = Lexer::new(input);
    // Parse the input
    let result = parser::parse(lexer);
    // Print out the AST
    println!("{:#?}", result);

    if let Ok(ref m) = result {
        print_functions(input, m);
    }
}

fn print_functions(src: &str, module: &sealy_lang::parser::ast::Module) {
    use sealy_lang::parser::ast;
    for item in module.items.iter() {
        let ref item = item.node.item;
        match item.node {
            ast::ItemKind::Function(ref func) => {
                let lhs = item.start.index;
                let rhs = func.decl_end.index;
                println!("{}", &src[lhs..rhs]);
            }
            ast::ItemKind::Module(ref module) => {}
            _ => (),
        }
    }
}
