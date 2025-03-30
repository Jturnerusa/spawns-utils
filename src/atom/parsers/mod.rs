use core::iter::Iterator;

use nom::{
    branch::alt,
    bytes::{complete::tag, take_while, take_while1},
    combinator::{cut, eof, not, opt, peek, recognize, verify},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    Parser,
};

use crate::{
    atom::{Atom, Category, VersionSuffix},
    parser_utils::{debug, ignore, search, take_1_if},
    useflag::parsers::usedep,
    ParseResult,
};

use super::{
    Blocker, Name, Slot, SlotOperator, Version, VersionNumber, VersionOperator, VersionSuffixKind,
};

pub fn category(input: &str) -> ParseResult<Category> {
    recognize((
        take_1_if(|c: char| c.is_ascii_alphanumeric() || matches!(c, '_')),
        take_while1(|c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '+')),
    ))
    .map(|input: &str| Category(input.to_string()))
    .parse_complete(input)
}

pub fn name(input: &str) -> ParseResult<Name> {
    verify(
        recognize((
            take_1_if(|c: char| c.is_ascii_alphanumeric() || matches!(c, '_')),
            search(alt((
                ignore(eof),
                ignore((
                    preceded(tag("-"), version),
                    not(search((
                        preceded(tag("-"), version),
                        alt((eof, search(tag(":")))),
                    ))),
                )),
                ignore(not(take_1_if(|c: char| {
                    c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '+')
                }))),
            ))),
        )),
        |result: &str| {
            debug(search((preceded(tag("-"), version), eof)))
                .parse_complete(result)
                .is_err()
        },
    )
    .map(|result| Name(result.to_string()))
    .parse_complete(input)
}

pub fn version(input: &str) -> ParseResult<Version> {
    let numbers = separated_list1(tag("."), version_number);
    let suffixes = separated_list1(tag("_"), version_suffix);

    (
        numbers,
        opt(take_1_if(|c: char| c.is_ascii_lowercase()).map(|s: &str| s.chars().next().unwrap())),
        opt(preceded(tag("_"), suffixes)),
        opt(preceded(tag("-"), version_revision)),
    )
        .map(|(numbers, letter, suffixes, revision)| Version {
            numbers,
            letter,
            suffixes: suffixes.unwrap_or_default(),
            revision,
        })
        .parse_complete(input)
}

pub fn atom(input: &str) -> ParseResult<Atom> {
    (
        opt(blocker),
        opt(version_operator),
        terminated(category, tag("/")),
        name,
        opt(preceded(tag("-"), version)),
        opt(preceded(tag(":"), slot)),
        opt(delimited(
            tag("["),
            separated_list1(tag(","), usedep),
            tag("]"),
        )),
    )
        .map(
            |(blocker, version_operator, category, name, version, slot, usedeps)| Atom {
                blocker,
                version_operator,
                category,
                name,
                version,
                slot,
                usedeps: usedeps.unwrap_or_default(),
            },
        )
        .parse_complete(input)
}

fn slot(input: &str) -> ParseResult<Slot> {
    let primary = {
        cut(alt((
            recognize((
                take_1_if(|c: char| c.is_ascii_alphanumeric() || matches!(c, '_')),
                take_while(|c: char| {
                    c.is_ascii_alphanumeric() || matches!(c, '+' | '_' | '.' | '-')
                }),
            ))
            .map(|result: &str| result.to_string()),
            recognize(tag("*")).map(|_| String::from("*")),
            peek(tag("=")).map(|_| String::new()),
        )))
    };

    let subslot = cut(recognize((
        take_1_if(|c: char| c.is_ascii_alphanumeric() || matches!(c, '_')),
        take_while(|c: char| c.is_ascii_alphanumeric() || matches!(c, '+' | '_' | '.' | '-')),
    )))
    .map(|result: &str| result.to_string());

    (
        primary,
        opt(preceded(tag::<&str, &str, _>("/"), subslot)),
        opt(tag("=").map(|_| SlotOperator::Eq)),
    )
        .map(|(primary, sub, operator)| Slot {
            primary,
            sub,
            operator,
        })
        .parse_complete(input)
}

fn blocker(input: &str) -> ParseResult<Blocker> {
    let weak = tag("!").map(|_| Blocker::Weak);
    let strong = tag("!!").map(|_| Blocker::Strong);

    alt((strong, weak)).parse_complete(input)
}

fn version_operator(input: &str) -> ParseResult<VersionOperator> {
    let eq = tag("=").map(|_| VersionOperator::Eq);
    let lt = tag("<").map(|_| VersionOperator::Lt);
    let lt_eq = tag("<=").map(|_| VersionOperator::LtEq);
    let gt = tag(">").map(|_| VersionOperator::Gt);
    let gt_eq = tag(">=").map(|_| VersionOperator::GtEq);
    let roughly = tag("~").map(|_| VersionOperator::Roughly);

    alt((lt_eq, gt_eq, eq, lt, gt, roughly)).parse_complete(input)
}

fn version_number(input: &str) -> ParseResult<VersionNumber> {
    take_while1(|c: char| c.is_ascii_digit())
        .map(|result: &str| VersionNumber(result.to_string()))
        .parse_complete(input)
}

fn version_revision(input: &str) -> ParseResult<VersionNumber> {
    preceded(tag("r"), version_number).parse_complete(input)
}

fn version_suffix(input: &str) -> ParseResult<VersionSuffix> {
    let alpha = tag("alpha").map(|_| VersionSuffixKind::Alpha);
    let beta = tag("beta").map(|_| VersionSuffixKind::Beta);
    let pre = tag("pre").map(|_| VersionSuffixKind::Pre);
    let rc = tag("rc").map(|_| VersionSuffixKind::Rc);
    let p = tag("p").map(|_| VersionSuffixKind::P);

    let suffix = alt((alpha, beta, pre, rc, p));

    (suffix, opt(version_number))
        .map(|(kind, number)| VersionSuffix { kind, number })
        .parse_complete(input)
}

#[cfg(test)]
mod tests {

    use core::assert_eq;

    use super::*;

    #[test]
    fn test_simple_version() {
        let input = "1.0.0";

        let (_, version) = version(input).unwrap();

        assert_eq!(version.numbers().next().unwrap().get(), "1");

        assert_eq!(version.numbers().nth(1).unwrap().get(), "0");

        assert_eq!(version.numbers().nth(2).unwrap().get(), "0");
    }

    #[test]
    fn test_version_with_suffix() {
        let input = "1.0.0_alpha1";

        let (_, version) = version(input).unwrap();

        assert!(matches!(
            version.suffixes().next().unwrap().kind(),
            VersionSuffixKind::Alpha
        ));

        assert_eq!(
            version.suffixes().next().unwrap().number().unwrap().get(),
            "1"
        );
    }

    #[test]
    fn test_version_with_revision() {
        let input = "1.0.0-r1";

        let (_, version) = version(input).unwrap();

        assert_eq!(version.revision().unwrap().get(), "1");
    }

    #[test]
    fn test_complex_version() {
        let input = "8.3_beta_p20250128";

        let (_, version) = version(input).unwrap();

        assert_eq!(version.numbers().next().unwrap().get(), "8");

        assert_eq!(version.numbers().nth(1).unwrap().get(), "3");

        assert!(matches!(
            version.suffixes().next().unwrap().kind(),
            VersionSuffixKind::Beta
        ));

        assert!(version.suffixes().next().unwrap().number().is_none());

        assert!(matches!(
            version.suffixes().nth(1).unwrap().kind(),
            VersionSuffixKind::P
        ));

        assert_eq!(
            version.suffixes().nth(1).unwrap().number().unwrap().get(),
            "20250128"
        );
    }

    #[test]
    fn test_version_with_incorrect_suffix() {
        let input = "1.0.0alpha32-r1";

        assert!(terminated(version, eof).parse_complete(input).is_err())
    }

    #[test]
    fn test_version_with_incorrect_revision() {
        let input = "1.0.0-ra";

        assert!(terminated(version, eof).parse_complete(input).is_err());
    }

    #[test]
    fn test_simple_name() {
        let input = "foo-bar";

        let (_, name) = name(input).unwrap();

        assert_eq!(name.get(), "foo-bar");
    }

    #[test]
    fn test_name_with_version() {
        let input = "foo-bar-1.0.0";

        let (_, name) = name(input).unwrap();

        assert_eq!(name.get(), "foo-bar");
    }

    #[test]
    fn test_name_with_version_ending() {
        let input = "foo-bar-1-1.0-r1";

        assert!(name.parse_complete(input).is_err());
    }

    #[test]
    fn test_atom() {
        let input = "!!>=cat/pkg-1.0.0v_alpha1_p20250326-r1:primary/sub=[!a(+),-b(-)=,c?]";

        let (_, atom) = atom(input).unwrap();

        assert_eq!(atom.to_string(), input);
    }

    #[test]
    fn test_atom_with_empty_slot() {
        let input = "!!>=cat/pkg-1.0.0v_alpha1_p20250326-r1:[!a(+),-b?,c(-)]";

        assert!(atom(input).is_err());
    }

    #[test]
    fn test_cursed_atom() {
        let input = "!!>=_.+-0-/_-test-T-123_beta1_-4a-6+-_p--1.00.02b_alpha3_pre_p4-r5:slot/_-+6-9=[test(+),test(-)]";

        let (_, atom) = atom(input).unwrap();

        assert_eq!(atom.to_string(), input);
    }
}
