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
    #[error("Unable to parse hardpath schema entry as `name :: subline`: Line {0}")]
    EntryHeader(usize),
    #[error("Unexpected content found in hardpath schema entry: Line {0}")]
    UnexpectedContent(usize),
    #[error("Expected content in hardpath schema entry: Line {0}")]
    ExpectedContent(usize),
}

impl Error {
    pub fn line(&self) -> Option<usize> {
        match self {
            Error::InvalidSchema => None,
            Error::TreeRootNotFound => None,
            Error::LineIndent(line) => Some(*line),
            Error::EntryKind => None,
            Error::EntryHeader(line) => Some(*line),
            Error::UnexpectedContent(line) => Some(*line),
            Error::ExpectedContent(line) => Some(*line),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
