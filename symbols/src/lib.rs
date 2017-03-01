extern crate owning_ref;

use owning_ref::RcRef;
use std::collections::HashMap;
use std::rc::Rc;
use std::u32;

pub type RefString = RcRef<String, str>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sym(u32);

#[derive(Clone)]
pub struct SymTable {
    names: Vec<RefString>,
    lookup: HashMap<RefString, Sym>,
}

impl SymTable {
    pub fn new() -> Self {
        let mut table = SymTable {
            names: Default::default(),
            lookup: Default::default(),
        };

        table.intern("_");

        table
    }

    pub fn underscore(&self) -> Sym {
        Sym(0)
    }

    pub fn get_name(&self, sym: Sym) -> Option<RefString> {
        self.names.get(sym.0 as usize).cloned()
    }

    pub fn intern(&mut self, label: &str) -> Sym {
        if let Some(&sym) = self.lookup.get(label) {
            return sym;
        }

        assert!(self.names.len() < u32::MAX as usize);

        let s = RcRef::new(Rc::new(label.to_string())).map(|s| &s[..]);

        let id = self.names.len() as u32;
        self.names.push(s.clone());
        self.lookup.insert(s, Sym(id));
        Sym(id)
    }
}