use {Sym, SymTable};
use std::{slice, str};

#[no_mangle]
pub unsafe extern "C" fn seal_create_sym_table() -> *mut SymTable {
    Box::into_raw(Box::new(SymTable::new()))
}

#[no_mangle]
pub unsafe extern "C" fn seal_free_sym_table(table: *mut SymTable) {
    Box::from_raw(table);
}

#[no_mangle]
pub unsafe extern "C" fn seal_intern_sym(table: *mut SymTable, string: *const u8, len: usize,
                                         sym: &mut u32)
                                         -> bool {
    let slice = slice::from_raw_parts(string, len);
    let text = match str::from_utf8(slice) {
        Ok(text) => text,
        Err(_) => return false,
    };

    *sym = (*table).intern(text).0;
    true
}

#[no_mangle]
pub unsafe extern "C" fn seal_lookup_sym(table: *const SymTable, sym: u32,
                                         string: &mut *const u8, len: &mut usize)
                                         -> bool {
    let text = match (*table).lookup_str(Sym(sym)) {
        Some(text) => text,
        None => return false,
    };

    *string = text.as_ptr();
    *len = text.len();
    true
}
