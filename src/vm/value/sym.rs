use std::collections::HashMap;
use std::u32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sym(u32);

#[derive(Clone)]
pub struct SymTable {
    names: Vec<String>,
    lookup: HashMap<String, Sym>,
}

impl SymTable {
    pub fn new() -> Self {
        let mut table = SymTable {
            names: Default::default(),
            lookup: Default::default(),
        };

        table.insert("_");

        table
    }

    pub fn underscore(&self) -> Sym {
        Sym(0)
    }

    pub fn get_name(&self, sym: Sym) -> Option<&str> {
        self.names.get(sym.0 as usize).map(|s| &s[..])
    }

    pub fn insert(&mut self, label: &str) -> Sym {
        if let Some(&sym) = self.lookup.get(label) {
            return sym;
        }

        assert!(self.names.len() < u32::MAX as usize);

        let id = self.names.len() as u32;
        self.names.push(label.into());
        self.lookup.insert(label.into(), Sym(id));
        Sym(id)
    }
}
