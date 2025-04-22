use quote::ToTokens;
use syn::{parse::Parse, spanned::Spanned};
use crate::*;

pub type Linespan = (String, Span);

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

        let mut doc_lines: Vec<Linespan> = Vec::new();
        let mut codefence_lines: Vec<Linespan> = Vec::new();
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
                        } else {
                            doc_lines.push((line, span))
                        }
                    },
                    ParseState::Active => {
                        if line.trim().starts_with(COMMENT_BLOCK_END) {
                            parsing_codefence = ParseState::Complete;
                        } else {
                            codefence_lines.push((line, span))
                        }
                    },
                    ParseState::Complete => {
                        doc_lines.push((line, span))
                    }
                }
            });

        if codefence_lines.is_empty() {
            return syn_err!(ident_span, E_CODEFENCE_NOT_FOUND);
        }

        let doc_lines: Vec<Linespan> = doc_lines.drain(0..2).collect();

        let title = doc_lines.get(0)
            .ok_or_else(|| syn_error!(ident_span, E_TREE_NAME_NOT_FOUND))?
            .0.to_owned();
        let subline = doc_lines.get(1)
            .ok_or_else(|| syn_error!(ident_span, E_TREE_SUBLINE_NOT_FOUND))?
            .0.to_owned();

        let tree = HardpathRawNode::parse_from_codefence(title, subline, codefence_lines, ident_span)?;
        let macro_model = HardpathMacroModel { tree };

        Ok(HardpathItem {
            syn_struct: struct_item,
            macro_model,
        })
    }
}

fn trim_indent(line: &(Span, String), indent: usize) -> syn::Result<&str> {
    let (span, line) = line;

    if line.len() < indent {
        return syn_err!(*span, E_LINE_INDENT);
    }

    Ok(&line[indent..])
}

enum EntryKind {
    None,
    Leaf,
    Branch,
    Continue
}

impl EntryKind {
    const NONE_PREFIX: &'static str = " ";
    const LEAF_PREFIX: &'static str = "|-- ";
    const BRANCH_PREFIX: &'static str = "|-+ ";
    const CONTINUE_PREFIX: &'static str = "| ";

    fn slice_after<'a,'b>(&'a self, line: &'b str) -> &'b str {
        match self {
            EntryKind::None => &line[1..],
            EntryKind::Leaf => &line[5..],
            EntryKind::Branch => &line[5..],
            EntryKind::Continue => &line[2..],
        }
    }
}

impl TryFrom<&Linespan> for EntryKind {
    type Error = syn::Error;

    fn try_from(linespan: &Linespan) -> Result<Self, Self::Error> {
        let (line, span) = linespan;
        match line {
            line if line.starts_with(Self::LEAF_PREFIX) => Ok(EntryKind::Leaf),
            line if line.starts_with(Self::BRANCH_PREFIX) => Ok(EntryKind::Branch),
            line if line.starts_with(Self::CONTINUE_PREFIX) => Ok(EntryKind::Continue),
            line if line.starts_with(Self::NONE_PREFIX) => Ok(EntryKind::None),
            _ => syn_err!(*span, E_ENTRY_KIND)
        }
    }
}

impl<'a> HardpathRawNode<'a> {
    const NAME_SEPARATOR: &'static str = " :: ";

    pub(crate) fn parse_from_codefence(name: String, subline: String, lines: Vec<Linespan>, ident_span: Span) -> syn::Result<Self> {
        // first node is '.' and establishes identation
        let dot_line = &lines.get(0)
            .ok_or_else(|| syn_error!(ident_span, E_TREE_ROOT_NOT_FOUND))?
            .0;

        let indent = dot_line.chars().take_while(|c| *c == ' ').count();
        let depth = 0;
        let line_index = 0;

        Self::parse_codefence_direct_children(depth, indent, None, line_index, &lines)?
            .into_iter()
            .map(|child| Self::parse_codefence_child(child, lines, indent))
            .collect::<syn::Result<Vec<Self>>>()?;

        let tree = HardpathRawNode {
            parent_path_str: None,
            path_str: ".".to_string(),
            name,
            subline,
            children,
            depth,
            line_index
        };

        Ok(tree)
    }

    fn parse_codefence_direct_children(depth: usize, indent: usize, parent_path_str: Option<&str>, line_index: usize, lines: &[Linespan]) -> syn::Result<Vec<Self>> {
        let mut children = Vec::new();

        if lines.len() <= line_index {
            return Ok(children);
        }

        for linespan in &lines[line_index..] {
            let entry_kind = EntryKind::try_from(linespan)?;

            let node;
            match entry_kind {
                EntryKind::Leaf => {
                    node = Self::parse_codefence_node_line(indent, depth, parent_path_str, entry_kind, line_index, linespan)?;
                }
                EntryKind::Branch => {
                    node = Self::parse_codefence_node_line(indent, depth, parent_path_str, entry_kind, line_index, linespan)?;
                }
                EntryKind::Continue => continue,
                EntryKind::None => break
            }

            children.push(node);
        }

        Ok(children)
    }

    fn parse_codefence_node_line<'l>(indent: usize, depth: usize, parent_path_str: Option<&str>, entry_kind: EntryKind, line_index: usize, linespan: &'l Linespan) -> syn::Result<(&'l str, &'l str)> {
        let (line, span) = linespan;
        let (path_str, name)= entry_kind
            .slice_after(line)
            .split_once(Self::NAME_SEPARATOR)
            .ok_or_else(|| syn_error!(*span, E_ENTRY_HEADER))?;

        let path_str = path_str.trim();
        let name = name.trim();

        Ok((path_str, name))
    }

    fn parse_codefense_child(child: Self, lines: &[Linespan], indent: &str) -> syn::Result<Self> {
        todo!()
    }
}

impl ToTokens for HardpathMacroModelNode {
    fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}
