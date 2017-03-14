use {Lexer, LexicalError, Location, TokenType};
use std::{ptr, slice, str};

#[no_mangle]
pub unsafe extern "C" fn seal_create_lexer(input: *const u8, length: usize) -> *mut Lexer<'static> {
    let input = slice::from_raw_parts(input, length);
    let input = match str::from_utf8(input) {
        Ok(input) => input,
        Err(_) => return ptr::null_mut(),
    };
    Box::into_raw(Box::new(Lexer::new(input)))
}

#[no_mangle]
pub unsafe extern "C" fn seal_clone_lexer(lexer: *const Lexer<'static>) -> *mut Lexer<'static> {
    Box::into_raw(Box::new((*lexer).clone()))
}

#[no_mangle]
pub unsafe extern "C" fn seal_free_lexer(lexer: *mut Lexer<'static>) {
    Box::from_raw(lexer);
}

#[no_mangle]
pub unsafe extern "C" fn seal_next_token(lexer: *mut Lexer<'static>, token: &mut TokResult)
                                         -> NextResult {
    let next = (*lexer).next();
    let result = match next {
        Some(result) => result,
        None => return NextResult::None,
    };

    match result {
        Ok((left, tok, right)) => {
            token.tok = Tok {
                left: left,
                right: right,
                tt: tok.0,
                span: tok.1.as_ptr(),
                span_len: tok.1.len(),
            };
            NextResult::Token
        }
        Err(err) => {
            let (c, loc, e) = match err {
                LexicalError::Unexpected(c, loc) => {
                    (c as u32, loc, TokErrorKind::UnexpectedCharacter)
                }
                LexicalError::TooManyCloseCurlies(loc) => {
                    (0, loc, TokErrorKind::TooManyCloseCurlies)
                }
            };
            token.err = TokError {
                error: e,
                loc: loc,
                character: c,
            };
            NextResult::Error
        }
    }
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum NextResult {
    None = 0,
    Token = 1,
    Error = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union TokResult {
    pub tok: Tok,
    pub err: TokError,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tok {
    pub tt: TokenType,

    pub left: Location,
    pub right: Location,

    pub span: *const u8,
    pub span_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TokError {
    pub error: TokErrorKind,
    pub loc: Location,
    pub character: u32,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum TokErrorKind {
    UnexpectedCharacter = 0,
    TooManyCloseCurlies,
}
