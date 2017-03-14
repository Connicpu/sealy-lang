use sym::Sym;

#[derive(Debug)]
pub struct TypeName {
    pub parts: Vec<TypePart>,
}

#[derive(Debug)]
pub struct TypePart {
    pub name: Sym,
    pub generics: Vec<TypeName>,
}

#[derive(Debug)]
pub struct GenericType {
    pub name: Sym,
    pub bounds: Vec<TypeName>,
}
