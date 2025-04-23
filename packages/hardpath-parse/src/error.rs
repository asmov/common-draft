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
    pub const E_TREE_ROOT_NOT_FOUND: &str = "Tree root '.' not found at first line of ```hardpath``` code-fence";
    pub const E_LINE_INDENT: &str = "Indentation for hardpath tree item does not match parent";
    pub const E_ENTRY_KIND: &str = "Hardpath entry does not begin with a leaf `|-- `, branch `|-+ `, or continuation `| `";
    pub const E_ENTRY_HEADER: &str = "Unable to parse hardpath entry as `name :: subline`";
}
