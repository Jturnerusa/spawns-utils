use nom::{
    branch::alt,
    bytes::{complete::tag, take_while},
    combinator::{opt, recognize},
    Parser,
};

use crate::{
    parser_utils::take_1_if,
    useflag::{Operator, Sign},
    ParseResult,
};

use super::{Negate, UseDep, UseFlag};

pub fn useflag(input: &str) -> ParseResult<UseFlag> {
    recognize((
        take_1_if(|c: char| c.is_ascii_alphanumeric()),
        take_while(|c: char| c.is_ascii_alphanumeric() || matches!(c, '+' | '_' | '@' | '-')),
    ))
    .map(|result: &str| UseFlag(result.to_string()))
    .parse_complete(input)
}

pub fn usedep(input: &str) -> ParseResult<UseDep> {
    let negate = alt((
        tag("-").map(|_| Negate::Minus),
        tag("!").map(|_| Negate::Exclamation),
    ));

    let sign = alt((
        tag("(+)").map(|_| Sign::Plus),
        tag("(-)").map(|_| Sign::Minus),
    ));

    let operator = alt((
        tag("=").map(|_| Operator::Equal),
        tag("?").map(|_| Operator::Question),
    ));

    (opt(negate), useflag, opt(sign), opt(operator))
        .map(|(negate, useflag, sign, operator)| UseDep(negate, useflag, sign, operator))
        .parse_complete(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_allowed_chars() {
        assert!(useflag("valid").is_ok());

        assert!(useflag("-invalid").is_err());

        assert!(useflag("1valid+_@-").is_ok());
    }
}
