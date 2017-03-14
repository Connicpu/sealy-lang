use ExprNode;
use GenericType;
use Literal;
use Node;
use ScopeNode;
use TypeName;
use lexer::Location;
use sym::Sym;

#[derive(Debug)]
pub struct Module {
    pub docs: Vec<String>,
    pub items: Vec<Node<Item>>,
}

#[derive(Debug)]
pub struct Item {
    pub attributes: Vec<Node<Attribute>>,
    pub item: Node<ItemKind>,
}

#[derive(Debug)]
pub enum ItemKind {
    UseImport(UseImport),
    ExternImport(ExternImport),
    Impl(Impl),
    TypeDecl(TypeDecl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    TraitDecl(TraitDecl),
    Constant(Constant),
    Function(Function),
    ModDecl(ModDecl),
}

#[derive(Debug)]
pub enum Attribute {
    Doc(String),
    Attribute(AttributeValue),
}

#[derive(Debug)]
pub enum AttributeValue {
    Id(Sym),
    IdValue(Sym, Box<Node<AttributeValue>>),
    IdList(Sym, Vec<Node<AttributeValue>>),
    Literal(Node<Literal>),
}

#[derive(Debug)]
pub struct UseImport {
    pub base: TypeName,
    pub glob: bool,
    pub multi: Vec<Sym>,
}

#[derive(Debug)]
pub struct ExternImport {
    pub name: Sym,
}

#[derive(Debug)]
pub struct Impl {
    pub impl_type: Node<TypeName>,
    pub generics: Vec<GenericType>,
    pub interface: Option<Node<TypeName>>,
    pub items: Node<Module>,
}

#[derive(Debug)]
pub struct TypeDecl {
    pub name: Sym,
    pub generics: Vec<GenericType>,
    pub def: TypeName,
}

#[derive(Debug)]
pub struct StructDecl {
    pub name: Sym,
    pub generics: Vec<GenericType>,
    pub members: Vec<Node<StructItem>>,
}

#[derive(Debug)]
pub struct StructItem {
    pub attributes: Vec<Node<Attribute>>,
    pub name: Sym,
}

#[derive(Debug)]
pub struct EnumDecl {
    pub name: Sym,
    pub generics: Vec<GenericType>,
    pub members: Vec<Node<EnumItem>>,
}

#[derive(Debug)]
pub struct EnumItem {
    pub attributes: Vec<Node<Attribute>>,
    pub name: Sym,
    pub members: Option<Vec<Sym>>,
}

#[derive(Debug)]
pub struct TraitDecl {
    pub name: Sym,
    pub generics: Vec<GenericType>,
    pub members: Vec<Node<TraitItem>>,
}

#[derive(Debug)]
pub struct TraitItem {
    pub attributes: Vec<Node<Attribute>>,
    pub name: Sym,
    pub kind: TraitItemKind,
}

#[derive(Debug)]
pub enum TraitItemKind {
    Function {
        parameters: Vec<FnParam>,
        generics: Vec<GenericType>,
    },
    Constant(TypeName),
    Type(TypeName),
}

#[derive(Debug)]
pub struct Constant {
    pub name: Sym,
    pub type_bound: TypeName,
    pub expression: ExprNode,
}

#[derive(Debug)]
pub struct Function {
    pub name: Sym,
    pub parameters: Vec<FnParam>,
    pub body: ScopeNode,
    pub throws: bool,
    pub return_type: Option<TypeName>,
    pub generics: Vec<GenericType>,
    pub decl_end: Location,
}

#[derive(Debug)]
pub struct FnParam {
    pub name: Sym,
    pub type_bound: TypeName,
}

#[derive(Debug)]
pub enum ModDecl {
    Import(Sym),
    Inline(Sym, Box<Module>),
}
