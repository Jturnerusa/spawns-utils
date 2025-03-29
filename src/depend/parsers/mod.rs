use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    Parser,
};

use crate::{
    atom::parsers::atom, parser_utils::whitespace, useflag::parsers::useflag, ParseResult,
};

use super::{Conditional, Expr, UseRequirement};

pub fn exprs(input: &str) -> ParseResult<Vec<Expr>> {
    separated_list1(whitespace, expr).parse(input)
}

pub fn expr(input: &str) -> ParseResult<Expr> {
    let group = || delimited((tag("("), whitespace), exprs, (whitespace, tag(")")));
    let any_of = preceded((tag("||"), whitespace), group()).map(Expr::AnyOf);
    let one_of = preceded((tag("^^"), whitespace), group()).map(Expr::OneOf);
    let all_of = group().map(Expr::AnyOf);

    let conditional = (terminated(conditional, whitespace), group())
        .map(|(condtional, expr)| Expr::Condtional(condtional, expr));

    let atom = atom.map(Expr::Atom);

    let use_requirement = use_requirement.map(Expr::UseRequirement);

    alt((atom, conditional, use_requirement, any_of, one_of, all_of)).parse_complete(input)
}

fn conditional(input: &str) -> ParseResult<Conditional> {
    let negative = delimited(tag("!"), useflag, tag("?")).map(Conditional::Negative);
    let positive = terminated(useflag, tag("?")).map(Conditional::Positive);

    alt((negative, positive)).parse_complete(input)
}

fn use_requirement(input: &str) -> ParseResult<UseRequirement> {
    let negative = preceded(tag("!"), useflag).map(UseRequirement::Negative);
    let positive = useflag.map(UseRequirement::Positive);

    alt((negative, positive)).parse_complete(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_expr() {
        let input = r#"cat/pkg use? ( cat/pkg || ( cat/pkg cat/pkg ) ^^ ( cat/pkg cat/pkg ) )"#;

        let (_, exprs) = exprs(input).unwrap();

        assert_eq!(exprs.len(), 2);

        assert!(matches!(&exprs[0], Expr::Atom(_)));

        assert!(matches!(
            &exprs[1],
            Expr::Condtional(_, exprs)
            if
            matches!(exprs[0], Expr::Atom(_)) &&
            matches!(exprs[1], Expr::AnyOf(_)) &&
            matches!(exprs[2], Expr::OneOf(_))
        ));
    }
}
