use quote::{quote, ToTokens};

const IDENT_DOC: &'static str = "doc";
const COMMENT_BLOCK_START: &'static str = "```hardpath";
const COMMENT_BLOCK_END: &'static str = "```";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SynParse(#[from] syn::Error),
    #[error("```hardpath``` comment block not found")]
    CommentBlockNotFound(syn::Ident),
    #[error("```hardpath``` comment block is empty")]
    CommentBlockEmpty(syn::Ident),
}

impl Error {
    pub fn into_syn(self) -> syn::Error {
        match self {
            Self::SynParse(e) => e,
            Self::CommentBlockNotFound(ref span)
            | Self::CommentBlockEmpty(ref span) =>
            {
                syn::Error::new_spanned(span, self.to_string())
            }
        }
    }
}

pub fn parse_hardpath_macro(item: proc_macro2::TokenStream) -> Result<proc_macro2::TokenStream, Error> {
    let const_item: syn::ItemConst = syn::parse2(item)?;
    let const_ident: syn::Ident = const_item.ident;

    //dbg!(&const_item);

    // parse the ```hardpath``` comment block
    // match only doc attributes and push all of the lines into a vec
    let mut hardpath_comment_lines: Option<Vec<String>> = None;
    for attr in const_item.attrs {
        if attr.style != syn::AttrStyle::Outer || !attr.path().is_ident(IDENT_DOC) {
            continue;
        }

        match attr.meta {
            syn::Meta::NameValue(syn::MetaNameValue{value: syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(ref litstr),..}),..}) => {
                let line = litstr.token().to_string();

                if let Some(lines) = &mut hardpath_comment_lines {
                    if line.trim().starts_with(COMMENT_BLOCK_END) {
                        break;
                    } else {
                        lines.push(line);
                        continue;
                    }
                } else {
                    if line.trim().starts_with(COMMENT_BLOCK_START) {
                        hardpath_comment_lines = Some(Vec::new());
                    }

                    continue;
                }
            }
            _ => {}
        }
    }

    if hardpath_comment_lines.is_none() {
        return Err(Error::CommentBlockNotFound(const_ident));
    }

    let hardpath_comment_lines = hardpath_comment_lines.expect("should exist");
    if hardpath_comment_lines.is_empty() {
        return Err(Error::CommentBlockEmpty(const_ident));
    }

    let hardpath_raw_tree = HardpathRawNode::parse_comments(&const_ident, hardpath_comment_lines)?;

    let output = quote! {
        const #const_ident: HardpathNode = #hardpath_raw_tree;
    };

    //dbg!(output.to_string());

    Ok(output)
}

struct HardpathRawNode {
    path_str: String,
    name: String,
    description: String,
    parent_path_str: Option<String>,
    children: Vec<HardpathRawNode>,
}

impl HardpathRawNode {
    fn parse_comments(const_ident: &syn::Ident, lines: Vec<String>) -> Result<Self, syn::Error> {
        let children = Vec::new();

        let tree_node = HardpathRawNode {
            path_str: ".".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            parent_path_str: None,
            children: children,
        };

        Ok(tree_node)
    }
}


impl ToTokens for HardpathRawNode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}
