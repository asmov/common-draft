use syn::{parse::Parse, spanned::Spanned};
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ParseState {
    None,
    Active,
    Complete
}

impl Parse for HardpathItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let struct_item = syn::ItemStruct::parse(input)?;
        let ident_span = struct_item.ident.span();

        let doc_lines = struct_item.attrs.iter().filter_map(|attr| {
            if attr.style != syn::AttrStyle::Outer || !attr.path().is_ident(IDENT_DOC) {
                None
            } else {
                match attr.meta {
                    syn::Meta::NameValue(syn::MetaNameValue{value: syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(ref litstr),..}),..}) => {
                        Some((attr.span(), litstr.token().to_string()))
                    },
                    _ => None,
                }
            }
        }).collect::<Vec<_>>();

        if doc_lines.is_empty() {
            return Err(Error::CodefenceNotFound(ident_span).into());
        }

        let mut parsing_codefense = ParseState::None;
        let codefence_lines = doc_lines.iter().filter_map(|(span, line)| {
            match parsing_codefense {
                ParseState::None => {
                    if line.trim().starts_with(COMMENT_BLOCK_START) {
                        parsing_codefense = ParseState::Active;
                    }

                    None
                },
                ParseState::Active => {
                    if line.trim().starts_with(COMMENT_BLOCK_END) {
                        parsing_codefense = ParseState::Complete;
                        None
                    } else {
                        Some((span.clone(), line.clone()))
                    }
                },
                ParseState::Complete => None
            }
        }).collect::<Vec<_>>();

        if codefence_lines.is_empty() {
            return Err(Error::CodefenceNotFound(ident_span).into());
        }

        let macro_model = HardpathMacroModel::from_docblock(doc_lines, codefence_lines, &ident_span)?;

        Ok(HardpathItem {
            syn_struct: struct_item,
            macro_model,
        })
    }
}

impl HardpathMacroModel {
    fn from_docblock(docblock_lines: Vec<(Span,String)>, codefence_lines: Vec<(Span,String)>, ident_span: &Span) -> syn::Result<Self> {
        let children = Vec::new();

        let tree = HardpathRawNode {
            path_str: ".".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            parent_path_str: None,
            children,
        };

        Ok(Self {
            tree
        })
    }
}
