use core::{str, unreachable};
use std::path::PathBuf;

use nom::{AsChar, Parser};

#[derive(Clone, Debug)]
pub enum Error<'a> {
    Failure(&'a str),
    Incomplete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Content {
    Obj(Obj),
    Dir(Dir),
    Sym(Sym),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Obj {
    pub path: PathBuf,
    pub md5: String,
    pub size: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dir {
    pub path: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sym {
    pub src: PathBuf,
    pub dest: PathBuf,
    pub size: u64,
}

pub fn contents(input: &str) -> Result<Vec<Content>, Error> {
    use nom::{branch::alt, multi::many0};

    match many0(alt((
        obj.map(Content::Obj),
        dir.map(Content::Dir),
        sym.map(Content::Sym),
    )))
    .parse(input)
    {
        Ok((_, contents)) => Ok(contents),
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => Err(Error::Failure(e.input)),
        Err(nom::Err::Incomplete(_)) => Err(Error::Incomplete),
    }
}

fn md5(input: &str) -> nom::IResult<&str, String> {
    use nom::bytes::complete::take_while_m_n;

    take_while_m_n(32, 32, |c: char| c.is_hex_digit())
        .map(|input: &str| input.to_string())
        .parse(input)
}

fn size(input: &str) -> nom::IResult<&str, u64> {
    use nom::bytes::take_while1;

    take_while1(|c: char| c.is_ascii_digit())
        .map(|input: &str| input.parse::<u64>().unwrap())
        .parse(input)
}

fn obj(input: &str) -> nom::IResult<&str, Obj> {
    use nom::{
        bytes::complete::{tag, take},
        combinator::peek,
        multi::many_till,
        sequence::terminated,
    };

    let path = many_till(
        take(1usize),
        peek((tag(" "), md5, tag(" "), size, tag("\n"))),
    )
    .map(|(chars, ..)| {
        chars
            .iter()
            .flat_map(|slice| slice.chars())
            .collect::<String>()
            .into()
    });

    (
        terminated(tag("obj"), tag(" ")),
        terminated(path, tag(" ")),
        terminated(md5, tag(" ")),
        terminated(size, tag("\n")),
    )
        .map(|(_, path, md5, size)| Obj { path, md5, size })
        .parse(input)
}

fn dir(input: &str) -> nom::IResult<&str, Dir> {
    use nom::{
        bytes::complete::{tag, take},
        combinator::peek,
        multi::many_till,
        sequence::terminated,
    };

    let path = many_till(take::<usize, &str, _>(1usize), peek(tag("\n"))).map(|(chars, ..)| {
        chars
            .iter()
            .flat_map(|slice| slice.chars())
            .collect::<String>()
            .into()
    });

    (
        terminated(tag("dir"), tag(" ")),
        terminated(path, tag("\n")),
    )
        .map(|(_, path)| Dir { path })
        .parse(input)
}

fn sym(input: &str) -> nom::IResult<&str, Sym> {
    use nom::{
        bytes::complete::{tag, take},
        combinator::peek,
        multi::many_till,
        sequence::terminated,
    };

    let dest = || {
        many_till(take(1usize), peek((tag(" "), size, tag("\n")))).map(|(chars, ..)| {
            PathBuf::from(chars.iter().flat_map(|s| s.chars()).collect::<String>())
        })
    };

    let src = many_till(
        take(1usize),
        peek((tag(" -> "), dest(), tag(" "), size, tag("\n"))),
    )
    .map(|(chars, ..)| PathBuf::from(chars.iter().flat_map(|s| s.chars()).collect::<String>()));

    (
        terminated(tag::<&str, &str, nom::error::Error<&str>>("sym"), tag(" ")),
        terminated(src, tag(" -> ")),
        terminated(dest(), tag(" ")),
        terminated(size, tag("\n")),
    )
        .map(|(_, src, dest, size)| Sym { src, dest, size })
        .parse(input)
}
