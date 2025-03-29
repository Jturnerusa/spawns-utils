pub mod parsers;

use crate::{atom::Atom, useflag::UseFlag};

#[derive(Clone, Debug)]
pub enum Conditional {
    Negative(UseFlag),
    Positive(UseFlag),
}

#[derive(Clone, Debug)]
pub enum UseRequirement {
    Negative(UseFlag),
    Positive(UseFlag),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Atom(Atom),
    UseRequirement(UseRequirement),
    AllOf(Vec<Expr>),
    AnyOf(Vec<Expr>),
    OneOf(Vec<Expr>),
    Condtional(Conditional, Vec<Expr>),
}
