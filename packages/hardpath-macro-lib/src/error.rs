#[macro_export]
macro_rules! syn_error {
    ($span:expr, $msg:ident) => {
        ::syn::Error::new($span, $msg)
    };
}

#[macro_export]
macro_rules! syn_err {
    ($span:expr, $msg:ident) => {
        Err(syn_error!($span, $msg))
    };
}

pub mod msg {
    pub const E_CODEFENCE_NOT_FOUND: &str = "Docblock code-fence ```hardpath``` not found or empty";
}
