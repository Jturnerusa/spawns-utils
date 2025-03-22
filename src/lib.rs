pub mod parser_utils;
pub mod vdb;

pub type ParseResult<'a, T> = nom::IResult<&'a str, T>;
