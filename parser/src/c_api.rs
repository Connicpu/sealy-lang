use ast::Module;
use lexer::Lexer;
use parse;
use std::slice;
use sym::SymTable;

#[no_mangle]
pub unsafe extern "C" fn seal_parse(lexer: *mut Lexer<'static>, syms: *mut SymTable,
                                    result: &mut ParseResult)
                                    -> bool {
    let lexer = *Box::from_raw(lexer);
    let ast = match parse(lexer, &mut *syms) {
        Ok(ast) => ast,
        Err(e) => {
            let message = Box::into_raw(format!("{:?}", e)
                .into_bytes()
                .into_boxed_slice());
            let err = ParseError {
                message: (*message).as_mut_ptr(),
                message_len: (*message).len(),
            };
            result.err = Box::into_raw(Box::new(err));
            return false;
        }
    };

    result.ast = Box::into_raw(Box::new(ast));
    true
}

#[no_mangle]
pub unsafe extern "C" fn seal_free_ast(ast: *mut Module) {
    Box::from_raw(ast);
}

#[no_mangle]
pub unsafe extern "C" fn seal_free_parse_error(err: *mut ParseError) {
    let err = Box::from_raw(err);
    let slice = slice::from_raw_parts_mut(err.message, err.message_len) as *mut _;
    Box::from_raw(slice);
}

#[no_mangle]
pub unsafe extern "C" fn seal_print_ast(ast: *const Module) {
    println!("{:#?}", *ast);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ParseResult {
    ast: *mut Module,
    err: *mut ParseError,
}

#[repr(C)]
pub struct ParseError {
    message: *mut u8,
    message_len: usize,
}
