// listed from lowest to highest precedence
#[derive(Copy, Clone, Debug)]
pub enum BinOp {
    Implements,

    RangeExclusive,
    RangeInclusive,

    LogicalOr,

    LogicalAnd,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,

    BitOr,
    BitXor,
    BitAnd,

    LShiftLeft,
    LShiftRight,
    AShiftRight,

    Add,
    Sub,

    Mul,
    Div,
    Rem,
    DivRem,
    Mod,
}

#[derive(Copy, Clone, Debug)]
pub enum AssignOp {
    Assign,

    LogicalOr,
    LogicalAnd,

    BitOr,
    BitXor,
    BitAnd,

    LShiftLeft,
    LShiftRight,
    AShiftRight,

    Add,
    Sub,

    Mul,
    Div,
    Rem,
    Mod,
}