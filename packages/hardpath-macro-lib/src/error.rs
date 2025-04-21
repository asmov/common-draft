
macro_rules! syn_error {
    ($span:ident, $msg:ident) => {
        ::syn::Error::new($span, $msg)
    };
}

macro_rules! syn_err {
    ($span:ident, $msg:ident) => {
        Err(syn_error!($span, $msg))
    };
}

pub mod msg {
    pub const E_CODEFENCE_NOT_FOUND: &str = "Docblock codefence ```hardpath``` not found or empty";
}
