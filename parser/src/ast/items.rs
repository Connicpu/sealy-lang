use super::ExprNode;
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
    ExternImport(ExternImport<'input>),
    Impl(Impl<'input>),
    TypeDecl(TypeDecl<'input>),
    StructDecl(StructDecl<'input>),
    EnumDecl(EnumDecl<'input>),
    TraitDecl(TraitDecl<'input>),
    Constant(Constant<'input>),
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
pub struct ExternImport<'input> {
    pub name: &'input str,
}

#[derive(Debug)]
pub struct Impl<'input> {
    pub impl_type: TypeName<'input>,
    pub interface: Option<TypeName<'input>>,
    pub items: Node<Module<'input>>,
}

#[derive(Debug)]
pub struct TypeDecl<'input> {
    pub name: &'input str,
}

#[derive(Debug)]
pub struct StructDecl<'input> {
    pub name: &'input str,
    pub members: Vec<Node<StructItem<'input>>>,
}

#[derive(Debug)]
pub struct StructItem<'input> {
    pub attributes: Vec<Node<Attribute<'input>>>,
    pub name: &'input str,
}

#[derive(Debug)]
pub struct EnumDecl<'input> {
    pub name: &'input str,
    pub members: Vec<Node<EnumItem<'input>>>,
}

#[derive(Debug)]
pub struct EnumItem<'input> {
    pub attributes: Vec<Node<Attribute<'input>>>,
    pub name: &'input str,
    pub members: Option<Vec<&'input str>>,
}

#[derive(Debug)]
pub struct TraitDecl<'input> {
    pub name: &'input str,
    pub members: Vec<Node<TraitItem<'input>>>,
}

#[derive(Debug)]
pub struct TraitItem<'input> {
    pub attributes: Vec<Node<Attribute<'input>>>,
    pub name: &'input str,
    pub kind: TraitItemKind<'input>,
}

#[derive(Debug)]
pub enum TraitItemKind<'input> {
    Function(Vec<&'input str>),
    Constant,
}

#[derive(Debug)]
pub struct Constant<'input> {
    pub name: &'input str,
    pub expression: ExprNode<'input>,
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
