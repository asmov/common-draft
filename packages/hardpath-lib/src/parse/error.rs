use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Hardpath schema is empty or malformed")]
    InvalidSchema,
    #[error("Tree root entry '.' not found in hardpath schema header")]
    TreeRootNotFound,
    #[error("Indentation for hardpath schema item does not match parent: Line {0}")]
    LineIndent(usize),
    #[error("Hardpath schema entry must begin with a leaf `|-- `, branch `|-+ `, or continuation `|   `")]
    EntryKind,
    #[error("Unable to parse hardpath schema entry as `name :: subline`")]
    EntryHeader,
}

pub type Result<T> = std::result::Result<T, Error>;
