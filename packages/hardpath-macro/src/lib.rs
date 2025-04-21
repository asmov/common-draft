use asmov_common_hardpath_macro_lib::parse_hardpath_macro;

#[proc_macro]
pub fn hardpath(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match parse_hardpath_macro(proc_macro2::TokenStream::from(item)) {
        Ok(token_stream) => proc_macro::TokenStream::from(token_stream),
        Err(err) => proc_macro::TokenStream::from(err.into_syn().to_compile_error())
    }
}
