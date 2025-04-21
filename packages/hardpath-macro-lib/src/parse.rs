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

        let mut doc_lines: Vec<(Span, String)> = Vec::new();
        let mut codefence_lines: Vec<(Span, String)> = Vec::new();
        let mut parsing_codefence = ParseState::None;

        struct_item.attrs.iter()
            .filter(|attr| attr.style == syn::AttrStyle::Outer && attr.path().is_ident(IDENT_DOC))
            .filter_map(|attr| match attr.meta {
                syn::Meta::NameValue(syn::MetaNameValue{value: syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(ref litstr),..}),..}) => {
                    Some((attr.span(), litstr.value()))
                }
                _ => None
            })
            .for_each(|(span, line)| {
                match parsing_codefence {
                    ParseState::None => {
                        if line.trim().starts_with(COMMENT_BLOCK_START) {
                            parsing_codefence = ParseState::Active;
                        } else {
                            doc_lines.push((span, line))
                        }
                    },
                    ParseState::Active => {
                        if line.trim().starts_with(COMMENT_BLOCK_END) {
                            parsing_codefence = ParseState::Complete;
                        } else {
                            codefence_lines.push((span, line))
                        }
                    },
                    ParseState::Complete => {
                        doc_lines.push((span, line))
                    }
                }
            });

        if codefence_lines.is_empty() {
            return syn_err!(ident_span, E_CODEFENCE_NOT_FOUND);
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
