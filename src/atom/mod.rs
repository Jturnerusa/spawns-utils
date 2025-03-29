pub mod parsers;

use core::{
    fmt::{self, Display},
    iter::ExactSizeIterator,
    write,
};

use crate::useflag::UseDep;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blocker {
    Weak,
    Strong,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VersionOperator {
    Eq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Roughly,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Category(String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

#[derive(Clone, Debug)]
pub struct VersionNumber(String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SlotOperator {
    Eq,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    primary: String,
    sub: Option<String>,
    operator: Option<SlotOperator>,
}

#[derive(Clone, Copy, Debug)]
pub enum VersionSuffixKind {
    Alpha,
    Beta,
    Pre,
    Rc,
    P,
}

#[derive(Clone, Debug)]
pub struct VersionSuffix {
    kind: VersionSuffixKind,
    number: Option<VersionNumber>,
}

#[derive(Clone, Debug)]
pub struct Version {
    numbers: Vec<VersionNumber>,
    letter: Option<char>,
    suffixes: Vec<VersionSuffix>,
    revision: Option<VersionNumber>,
}

#[derive(Clone, Debug)]
pub struct Atom {
    blocker: Option<Blocker>,
    version_operator: Option<VersionOperator>,
    category: Category,
    name: Name,
    version: Option<Version>,
    slot: Option<Slot>,
    usedeps: Vec<UseDep>,
}

impl Category {
    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl Name {
    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl VersionNumber {
    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

impl VersionSuffix {
    pub fn kind(&self) -> VersionSuffixKind {
        self.kind
    }

    pub fn number(&self) -> Option<&VersionNumber> {
        self.number.as_ref()
    }
}

impl Version {
    pub fn numbers(&self) -> impl ExactSizeIterator<Item = &VersionNumber> {
        self.numbers.iter()
    }

    pub fn letter(&self) -> Option<char> {
        self.letter
    }

    pub fn suffixes(&self) -> impl ExactSizeIterator<Item = &VersionSuffix> {
        self.suffixes.iter()
    }

    pub fn revision(&self) -> Option<&VersionNumber> {
        self.revision.as_ref()
    }
}

impl Atom {
    pub fn blocker(&self) -> Option<Blocker> {
        self.blocker
    }

    pub fn version_operator(&self) -> Option<VersionOperator> {
        self.version_operator
    }

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }

    pub fn slot(&self) -> Option<&Slot> {
        self.slot.as_ref()
    }

    pub fn usedeps(&self) -> impl ExactSizeIterator<Item = &UseDep> {
        self.usedeps.iter()
    }
}

impl Display for Blocker {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Strong => write!(f, "!!"),
            Self::Weak => write!(f, "!"),
        }
    }
}

impl Display for VersionOperator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Eq => write!(f, "="),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
            Self::LtEq => write!(f, "<="),
            Self::GtEq => write!(f, ">="),
            Self::Roughly => write!(f, "~"),
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for VersionNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for SlotOperator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Eq => write!(f, "="),
        }
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.primary)?;

        if let Some(subslot) = self.sub.as_ref() {
            write!(f, "/{}", subslot)?;
        }

        if let Some(operator) = self.operator.as_ref() {
            write!(f, "{}", operator)?;
        }

        Ok(())
    }
}

impl Display for VersionSuffixKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Alpha => write!(f, "alpha"),
            Self::Beta => write!(f, "beta"),
            Self::Pre => write!(f, "pre"),
            Self::Rc => write!(f, "rc"),
            Self::P => write!(f, "p"),
        }
    }
}

impl Display for VersionSuffix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.kind)?;

        if let Some(number) = self.number.as_ref() {
            write!(f, "{}", number)?;
        }

        Ok(())
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        intersperse(self.numbers.iter(), ".", f)?;

        if let Some(letter) = self.letter.as_ref() {
            write!(f, "{}", letter)?;
        }

        if !self.suffixes.is_empty() {
            write!(f, "_")?;
            intersperse(self.suffixes.iter(), "_", f)?;
        }

        if let Some(revision) = self.revision.as_ref() {
            write!(f, "-r{}", revision.get())?;
        }

        Ok(())
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(blocker) = self.blocker {
            write!(f, "{}", blocker)?;
        }

        if let Some(version_operator) = self.version_operator.as_ref() {
            write!(f, "{}", version_operator)?;
        }

        write!(f, "{}/{}", self.category, self.name)?;

        if let Some(version) = self.version.as_ref() {
            write!(f, "-{}", version)?;
        }

        if let Some(slot) = self.slot.as_ref() {
            write!(f, ":{}", slot)?;
        }

        if !self.usedeps.is_empty() {
            write!(f, "[")?;
            intersperse(self.usedeps.iter(), ",", f)?;
            write!(f, "]")?;
        }

        Ok(())
    }
}

fn intersperse(
    iter: impl ExactSizeIterator<Item = impl Display>,
    separator: &str,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let len = iter.len();

    for (i, item) in iter.enumerate() {
        write!(f, "{}", item)?;

        if i < len - 1 {
            write!(f, "{}", separator)?;
        }
    }

    Ok(())
}
