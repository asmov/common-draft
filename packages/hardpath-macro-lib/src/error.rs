use proc_macro2::Span;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SynParse(#[from] syn::Error),
    #[error("Docblock codefence ```hardpath``` not found or empty")]
    CodefenceNotFound(Span),
}

impl Into<syn::Error> for Error {
    fn into(self) -> syn::Error {
        match self {
            Self::SynParse(e) => e,
            Self::CodefenceNotFound(span) =>
            {
                let msg = self.to_string();
                syn::Error::new(span, msg)
            }
        }
    }
}
