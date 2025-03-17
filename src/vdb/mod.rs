use std::path::PathBuf;

pub mod parsers;

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
