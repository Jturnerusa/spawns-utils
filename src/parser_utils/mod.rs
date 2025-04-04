use core::{clone::Clone, fmt};

use nom::{
    bytes::{complete::take_while1, take, take_while_m_n},
    combinator::{complete, peek},
    error::ParseError,
    Input, Mode, Parser,
};

use crate::ParseResult;

pub struct Debug<F>(F);

pub struct Lookahead<F>(F);

pub struct Search<F>(F);

pub struct Uncut<F>(F);

impl<I, O, E, F> Parser<I> for Debug<F>
where
    I: Input + fmt::Debug,
    O: fmt::Debug,
    E: ParseError<I>,
    F: Parser<I, Output = O, Error = E>,
{
    type Output = F::Output;
    type Error = F::Error;

    fn process<OM: nom::OutputMode>(
        &mut self,
        input: I,
    ) -> nom::PResult<OM, I, Self::Output, Self::Error> {
        eprintln!("input: {:?}", input);

        match self.0.process::<OM>(input) {
            Ok((rest, result)) => {
                eprintln!("rest: {:?}", rest);
                eprintln!();

                Ok((rest, result))
            }
            e => e,
        }
    }
}

impl<I, E, F> Parser<I> for Lookahead<F>
where
    I: Input + Clone,
    E: ParseError<I>,
    F: Parser<I, Error = E>,
{
    type Output = I;
    type Error = F::Error;

    fn process<OM: nom::OutputMode>(
        &mut self,
        input: I,
    ) -> nom::PResult<OM, I, Self::Output, Self::Error> {
        let mut remaining = input.clone();
        let mut consumed = 0usize;

        while self.0.parse(remaining.clone()).is_err() {
            remaining = match take(1usize).parse(remaining.clone()) {
                Ok((rest, _)) => rest,
                Err(nom::Err::Error(e)) => return Err(nom::Err::Error(OM::Error::bind(|| e))),
                Err(nom::Err::Failure(e)) => return Err(nom::Err::Failure(e)),
                Err(nom::Err::Incomplete(i)) => return Err(nom::Err::Incomplete(i)),
            };

            consumed += 1;
        }

        Ok((remaining, OM::Output::bind(|| input.take(consumed))))
    }
}

impl<I, E, F> Parser<I> for Uncut<F>
where
    I: Input,
    E: ParseError<I>,
    F: Parser<I, Error = E>,
{
    type Output = F::Output;
    type Error = F::Error;

    fn process<OM: nom::OutputMode>(
        &mut self,
        input: I,
    ) -> nom::PResult<OM, I, Self::Output, Self::Error> {
        match self.0.process::<OM>(input) {
            Ok((rest, result)) => Ok((rest, result)),
            Err(nom::Err::Failure(e)) => Err(nom::Err::Error(OM::Error::bind(|| e))),
            e => e,
        }
    }
}

// not sure why we cant return impl Parser here
pub fn debug<I, O, E, F>(parser: F) -> Debug<F>
where
    I: Input + fmt::Debug,
    E: ParseError<I>,
    F: Parser<I, Output = O, Error = E>,
{
    Debug(parser)
}

pub fn lookahead<I, E, F>(parser: F) -> impl Parser<I, Output = I, Error = E>
where
    I: Input + Clone,
    E: ParseError<I>,
    F: Parser<I, Error = E>,
{
    Lookahead(peek(parser))
}

pub fn take_1_if<I, E, F>(f: F) -> impl Parser<I, Output = I, Error = E>
where
    I: Input,
    F: Fn(I::Item) -> bool,
    E: ParseError<I>,
{
    take_while_m_n(1, 1, f)
}

pub fn whitespace(input: &str) -> ParseResult<&str> {
    take_while1(|c: char| c.is_ascii_whitespace()).parse_complete(input)
}

pub fn search<I, E, F>(parser: F) -> impl Parser<I, Output = I, Error = E>
where
    I: Input + Clone,
    E: ParseError<I>,
    F: Parser<I, Error = E>,
{
    complete(lookahead(parser))
}

pub fn ignore<I, E, F>(parser: F) -> impl Parser<I, Output = (), Error = E>
where
    I: Input,
    E: ParseError<I>,
    F: Parser<I, Error = E>,
{
    parser.map(|_| ())
}

pub fn uncut<I, E, F>(parser: F) -> impl Parser<I, Error = E>
where
    I: Input,
    E: ParseError<I>,
    F: Parser<I, Error = E>,
{
    Uncut(parser)
}

#[cfg(test)]
mod tests {

    use nom::{
        bytes::{complete::take_while_m_n, tag, take_while1},
        AsChar, Parser,
    };

    use super::*;

    #[test]
    fn test_lookahead() {
        let input = "/usr/share/alsa/ucm2/NXP/iMX8/Librem_5_Devkit/Librem 5 Devkit.conf 6c0d51586d94c272b160eb7ba6c61331 1739589188
";

        let md5 = take_while_m_n(32, 32, |c: char| c.is_hex_digit());
        let size = take_while1(|c: char| c.is_ascii_digit());

        let (_, result) = lookahead::<&str, nom::error::Error<&str>, _>((
            tag(" "),
            md5,
            tag(" "),
            size,
            tag("\n"),
        ))
        .parse_complete(input)
        .unwrap();

        assert_eq!(
            result,
            "/usr/share/alsa/ucm2/NXP/iMX8/Librem_5_Devkit/Librem 5 Devkit.conf"
        );
    }

    #[test]
    fn test_search() {
        let input = r#"
DESCRIPTION=an example description
DEPEND=app-editors/emacs-31.0.1
"#;

        let (rest, result) = search((
            tag::<&str, &str, nom::error::Error<&str>>("\nDEPEND="),
            take_while1(|c: char| !c.is_ascii_whitespace()),
        ))
        .parse_complete(input)
        .unwrap();

        assert_eq!(result, "\nDESCRIPTION=an example description");

        assert_eq!(rest, "\nDEPEND=app-editors/emacs-31.0.1\n");
    }
}
