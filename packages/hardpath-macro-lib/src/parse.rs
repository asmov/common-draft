use syn::parse::Parse;
use crate::*;

impl Parse for HardpathItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let struct_item = syn::ItemStruct::parse(input)?;
        let struct_ident = struct_item.ident;

        // parse the ```hardpath``` comment block
        // match only doc attributes and push all of the lines into a vec


        let mut title; // the extracted first line of the docblock
        let mut subline; // the extracted second line of the docblock
        let mut codefence_lines: Option<Vec<(Span, String)>> = None; // the extracted codefence: ```hardpath

        for attr in struct_item.attrs {
            if attr.style != syn::AttrStyle::Outer || !attr.path().is_ident(IDENT_DOC) {
                continue;
            }

            match attr.meta {
                syn::Meta::NameValue(syn::MetaNameValue{value: syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(ref litstr),..}),..}) => {
                    let line = litstr.token().to_string();

                    if let Some(lines) = &mut doc_lines {
                        if line.trim().starts_with(COMMENT_BLOCK_END) {
                            lines.push((attr.span(), line));
                            break;
                        } else {
                            lines.push((attr.span(), line));
                        }
                    } else {
                        if line.trim().starts_with(COMMENT_BLOCK_START) {
                            doc_lines = Some(vec![(attr.span(), line)]);
                        }

                        continue;
                    }
                }
                _ => {}
            }
        }

        if doc_lines.is_none() {
            return Err(Error::CommentBlockNotFound(struct_ident));
        }

        let hardpath_comment_lines = doc_lines.expect("should exist");
        if hardpath_comment_lines.is_empty() {
            return Err(Error::CommentBlockEmpty(struct_ident));
        }

        let hardpath_raw_tree = HardpathRawNode::parse_codefence(&struct_ident, hardpath_comment_lines)?;


        Ok(HardpathItem {
            syn_struct: struct_item,
        })
    }
}
