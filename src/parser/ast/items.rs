use super::Literal;
use super::Node;
use super::ScopeNode;
use super::TypeName;
use lexer::Location;

#[derive(Debug)]
pub struct Module<'input> {
    pub docs: Vec<&'input str>,
    pub items: Vec<Node<Item<'input>>>,
}

#[derive(Debug)]
pub struct Item<'input> {
    pub attributes: Vec<Node<Attribute<'input>>>,
    pub item: Node<ItemKind<'input>>,
}

#[derive(Debug)]
pub enum ItemKind<'input> {
    UseImport(UseImport<'input>),
    Function(Function<'input>),
    ModDecl(ModDecl<'input>),
}

#[derive(Debug)]
pub enum Attribute<'input> {
    Doc(&'input str),
    Attribute(AttributeValue<'input>),
}

#[derive(Debug)]
pub enum AttributeValue<'input> {
    Id(&'input str),
    IdValue(&'input str, Box<Node<AttributeValue<'input>>>),
    IdList(&'input str, Vec<Node<AttributeValue<'input>>>),
    Literal(Node<Literal<'input>>),
}

#[derive(Debug)]
pub struct UseImport<'input> {
    pub base: TypeName<'input>,
    pub glob: bool,
    pub multi: Vec<&'input str>,
}

#[derive(Debug)]
pub struct Function<'input> {
    pub name: &'input str,
    pub parameters: Vec<&'input str>,
    pub body: ScopeNode<'input>,
    pub throws: bool,
    pub decl_end: Location,
}

#[derive(Debug)]
pub enum ModDecl<'input> {
    Import(&'input str),
    Inline(&'input str, Box<Module<'input>>),
}
