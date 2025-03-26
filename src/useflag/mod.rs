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
