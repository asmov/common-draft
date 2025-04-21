#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SynParse(#[from] syn::Error),
    #[error("```hardpath``` comment block not found")]
    CommentBlockNotFound(syn::Ident),
    #[error("```hardpath``` comment block is empty")]
    CommentBlockEmpty(syn::Ident),
}

impl Error {
    pub fn into_syn(self) -> syn::Error {
        match self {
            Self::SynParse(e) => e,
            Self::CommentBlockNotFound(ref span)
            | Self::CommentBlockEmpty(ref span) =>
            {
                syn::Error::new_spanned(span, self.to_string())
            }
        }
    }
}
