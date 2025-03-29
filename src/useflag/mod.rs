use core::{fmt, write};
use std::fmt::Display;

pub mod parsers;

#[derive(Clone, Debug)]
pub struct UseFlag(String);

#[derive(Clone, Copy, Debug)]
pub enum Negate {
    Minus,
    Exclamation,
}

#[derive(Clone, Copy, Debug)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Equal,
    Question,
}

#[derive(Clone, Debug)]
pub struct UseDep(Option<Negate>, UseFlag, Option<Sign>, Option<Operator>);

impl UseFlag {
    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl UseDep {
    pub fn negate(&self) -> Option<Negate> {
        self.0
    }

    pub fn useflag(&self) -> &UseFlag {
        &self.1
    }

    pub fn sign(&self) -> Option<Sign> {
        self.2
    }

    pub fn operator(&self) -> Option<Operator> {
        self.3
    }
}

impl Display for UseFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Negate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Exclamation => write!(f, "!"),
        }
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "(+)"),
            Self::Minus => write!(f, "(-)"),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Equal => write!(f, "="),
            Self::Question => write!(f, "?"),
        }
    }
}

impl Display for UseDep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(negate) = self.0 {
            write!(f, "{}", negate)?;
        }

        write!(f, "{}", self.1)?;

        if let Some(sign) = self.2 {
            write!(f, "{}", sign)?;
        }

        if let Some(operator) = self.3 {
            write!(f, "{}", operator)?;
        }

        Ok(())
    }
}
