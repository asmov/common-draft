pub mod error;
mod model;
mod parse;

use quote::quote;
use proc_macro2::Span;
use model::*;
use error::*;

const IDENT_DOC: &'static str = "doc";
const COMMENT_BLOCK_START: &'static str = "```hardpath";
const COMMENT_BLOCK_END: &'static str = "```";

pub fn parse_hardpath_macro(item: proc_macro2::TokenStream) -> Result<proc_macro2::TokenStream, Error> {
    let item: HardpathItem = syn::parse2(item)?;
    let struct_ident = item.syn_struct.ident;
    let struct_visibility = item.syn_struct.vis;
    let tree_value = &item.macro_model.tree;

    let macro_model_bytes = item.macro_model.serialize().unwrap();
    let macro_model_bytes_len = macro_model_bytes.len();
    let macro_model_bytes = syn::LitByteStr::new(&macro_model_bytes, struct_ident.span());

    let output = quote! {
        #struct_visibility struct #struct_ident;

        impl #struct_ident {
            const FS: ::asmov_common_hardpath::HardpathNode = #tree_value;
        }

        impl #struct_ident {
            const __HARDPATH_MACRO_MODEL_BYTES: &'static [#macro_model_bytes_len; u8] = include_bytes!(#macro_model_bytes);
        }
    };

    Ok(output)
}
