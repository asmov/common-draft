mod error;
mod model;

use std::str::FromStr;

use quote::quote;
use proc_macro2::Span;
pub(crate) use asmov_common_hardpath_lib::*;
use syn::spanned::Spanned;
use crate::model::*;

pub use error::msg::*;

const IDENT_DOC: &'static str = "doc";
const COMMENT_BLOCK_START: &'static str = "```hardpath";
const COMMENT_BLOCK_END: &'static str = "```";

type Linespan = (String, proc_macro2::Span);

pub fn parse_hardpath_macro(item: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
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
            const FS: ::asmov_common_hardpath::HardpathTree = #tree_value;
        }

        impl #struct_ident {
            const __HARDPATH_MACRO_MODEL_BYTES: &'static [#macro_model_bytes_len; u8] = include_bytes!(#macro_model_bytes);
        }
    };

    Ok(output)
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ParseState {
    None,
    Active,
    Complete
}

impl syn::parse::Parse for HardpathItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let struct_item = syn::ItemStruct::parse(input)?;
        let ident_span = struct_item.ident.span();

        let mut codefence_lines: Vec<String> = Vec::new();
        let mut codefence_spans: Vec<Span> = Vec::new();
        let mut parsing_codefence = ParseState::None;

        struct_item.attrs.iter()
            .filter(|attr| attr.style == syn::AttrStyle::Outer && attr.path().is_ident(IDENT_DOC))
            .filter_map(|attr| match attr.meta {
                syn::Meta::NameValue(syn::MetaNameValue{value: syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(ref litstr),..}),..}) => {
                    Some((litstr.value(), attr.span()))
                }
                _ => None
            })
            .for_each(|(line, span)| {
                match parsing_codefence {
                    ParseState::None => {
                        if line.trim().starts_with(COMMENT_BLOCK_START) {
                            parsing_codefence = ParseState::Active;
                        }
                    },
                    ParseState::Active => {
                        if line.trim().starts_with(COMMENT_BLOCK_END) {
                            parsing_codefence = ParseState::Complete;
                        } else {
                            codefence_lines.push(line);
                            codefence_spans.push(span);
                        }
                    },
                    ParseState::Complete => ()
                }
            });

        if codefence_lines.is_empty() {
            return syn_err!(ident_span, E_CODEFENCE_NOT_FOUND);
        }

        let codefence = codefence_lines.join("\n");

        let tree = SoftpathTree::from_str(&codefence)
            .map_err(|err| todo!())?;

        let macro_model = HardpathMacroModel { tree: HardpathMacroModelTree(tree) };

        Ok(HardpathItem {
            syn_struct: struct_item,
            macro_model,
        })
    }
}
