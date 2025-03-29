pub mod parsers;

use core::iter::ExactSizeIterator;

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
