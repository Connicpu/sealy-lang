use alloc::heap::allocate;
use std::collections::HashMap;
use std::rc::Rc;
use sym::Sym;
use vm::value::Value;
use vm::value::traits::Implementation;

#[derive(Debug)]
pub struct MetaType {
    pub name: Sym,
    pub fully_qualified: Sym,
    pub field_count: u32,
    pub fields: HashMap<Sym, u32>,
    pub inherent_impl: Implementation, 
    // TODO: Traits
}

#[derive(Debug)]
#[repr(C)]
pub struct StructObject {
    pub meta: Rc<MetaType>,
    pub members: [Value],
}

pub fn build_struct(meta: Rc<MetaType>) -> Box<StructObject> {
    use std::{mem, ptr};

    // This is the layout of an array DST's fat pointer
    #[repr(C)]
    struct StructDST {
        memory: *mut u8,
        len: usize,
    }

    let n = meta.field_count as usize;

    // Calculate the required memory and alignment
    let size = mem::size_of_val(&meta) + mem::size_of::<Value>() * n;
    let meta_align = mem::align_of_val(&meta);
    let value_align = mem::align_of::<Value>();
    let (size, align) = if value_align > meta_align {
        (size + value_align - meta_align, value_align)
    } else {
        (size, meta_align)
    };

    unsafe {
        // Allocate an object
        let memory = allocate(size, align);
        // Build the fat pointer description
        let dst = StructDST {
            memory: memory,
            len: n,
        };
        // Create the fat pointer
        let obj: *mut StructObject = mem::transmute(dst);
        // Write the metatable pointer
        ptr::write(&mut (*obj).meta, meta);
        // Initialize all of the values to nil
        for i in 0..n {
            let v = (*obj).members.get_unchecked_mut(i);
            ptr::write(v, Value::Nil);
        }
        // Wrap it up in a box
        Box::from_raw(obj)
    }
}
