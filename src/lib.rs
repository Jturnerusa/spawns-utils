pub mod atom;
pub mod depend;
pub mod parser_utils;
pub mod useflag;
pub mod vdb;

pub type ParseResult<'a, T> = nom::IResult<&'a str, T>;
