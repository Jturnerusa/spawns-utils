use core::str;
use std::path::PathBuf;

use nom::{AsChar, Parser};

use crate::{
    vdb::{Content, Dir, Obj, Sym},
    ParseResult,
};

#[derive(Clone, Debug)]
pub enum Error<'a> {
    Failure(&'a str),
    Incomplete,
}

pub fn contents(input: &str) -> ParseResult<Vec<Content>> {
    use nom::{branch::alt, multi::many0};

    many0(alt((
        obj.map(Content::Obj),
        dir.map(Content::Dir),
        sym.map(Content::Sym),
    )))
    .parse(input)
}

fn md5(input: &str) -> ParseResult<String> {
    use nom::bytes::complete::take_while_m_n;

    take_while_m_n(32, 32, |c: char| c.is_hex_digit())
        .map(|input: &str| input.to_string())
        .parse(input)
}

fn size(input: &str) -> ParseResult<u64> {
    use nom::bytes::take_while1;

    take_while1(|c: char| c.is_ascii_digit())
        .map(|input: &str| input.parse::<u64>().unwrap())
        .parse(input)
}

fn obj(input: &str) -> ParseResult<Obj> {
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

fn dir(input: &str) -> ParseResult<Dir> {
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

fn sym(input: &str) -> ParseResult<Sym> {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_contents() {
        let input = "obj /usr/share/alsa/ucm2/NXP/iMX8/Librem_5_Devkit/Librem 5 Devkit.conf 6c0d51586d94c272b160eb7ba6c61331 1739589188\ndir /a/path to something\nsym /a/path to something -> ../another path 102021\n";

        let expected = [
            Content::Obj(Obj {
                path: PathBuf::from(
                    "/usr/share/alsa/ucm2/NXP/iMX8/Librem_5_Devkit/Librem 5 Devkit.conf",
                ),
                md5: String::from("6c0d51586d94c272b160eb7ba6c61331"),
                size: 1739589188,
            }),
            Content::Dir(Dir {
                path: PathBuf::from("/a/path to something"),
            }),
            Content::Sym(Sym {
                src: PathBuf::from("/a/path to something"),
                dest: PathBuf::from("../another path"),
                size: 102021,
            }),
        ];

        let (_, contents) = contents(input).unwrap();

        for (received, expected) in contents.iter().zip(expected) {
            assert_eq!(*received, expected);
        }
    }
}
