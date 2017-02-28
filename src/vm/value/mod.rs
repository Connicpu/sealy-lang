pub use self::sym::Sym;

pub mod structobj;
pub mod traits;
pub mod sym;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
}
