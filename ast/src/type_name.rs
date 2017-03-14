use sym::Sym;

#[derive(Debug)]
pub struct TypeName {
    pub parts: Vec<TypePart>,
    pub bounds: Vec<TypeName>,
}

#[derive(Debug)]
pub struct TypePart {
    pub name: Sym,
    pub generics: Vec<TypeName>,
}
