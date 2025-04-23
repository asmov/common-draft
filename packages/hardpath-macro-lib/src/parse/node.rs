use std::cell::RefCell;

use crate::*;
use super::*;

#[derive(Debug)]
pub(crate) struct ParserNode<'a> {
    pub(crate) id: usize,
    pub(crate) cursor: Cursor,
    pub(crate) path_kind: PathKind,
    pub(crate) path_name: &'a str,
    pub(crate) name: Option<&'a str>,
    pub(crate) subline: Option<&'a str>,
    pub(crate) parent_id: Option<usize>,
    pub(crate) children: Vec<ParserNode<'a>>,
}

pub(crate) struct NodeParser {
    ident_span: Span,
    lines: Vec<Linespan>,
    indent: usize,
    next_id: RefCell<usize>,
}

impl NodeParser {
    pub(crate) fn new(lines: Vec<Linespan>, ident_span: Span) -> syn::Result<Self> {
        // first node is '.' and establishes identation
        let dot_line = &lines.get(0)
            .ok_or_else(|| syn_error!(ident_span, E_TREE_ROOT_NOT_FOUND))?
            .0;
        let indent = dot_line.chars().take_while(|c| *c == ' ').count();

        Ok(NodeParser {
            ident_span,
            lines,
            indent,
            next_id: RefCell::new(1),
        })
    }

    pub(crate) fn parse<'a>(self, name: &'a str, subline: &'a str) -> syn::Result<ParserNode<'a>> {
        let cursor = Cursor::new(self.indent);

        let children = Self::parse_direct_children(cursor)?
            .into_iter()
            .map(|child| Self::parse_child(child))
            .collect::<syn::Result<Vec<Self>>>()?;

        let tree = ParserNode {
            id: 0,
            path_kind: PathKind::Directory,
            cursor: Cursor::new(self.indent),
            path_name: ".",
            name: Some(name),
            subline: Some(subline),
            parent_id: None,
            children,
        };

        Ok(tree)
    }

    pub fn generate_id(&self) -> usize {
        let id = *self.next_id;
        self.next_id += 1;
        id
    }

    fn seek_from<'a>(line: &'a Linespan, cursor: &Cursor) -> syn::Result<&'a str> {
        let (span, line) = line;

        if line.len() < cursor.char_index {
            return syn_err!(*span, E_LINE_INDENT);
        }

        Ok(&line[cursor.char_index..])
    }

    fn parse_codefence_direct_children(&self, cursor: Cursor) -> syn::Result<Vec<Self>> {
        if self.lines.len() <= cursor.line_index {
            return Ok(Vec::new());
        }

        let char_index = indent + depth * EntryKind::DEPTH_INDENT;

        let mut children = Vec::new();
        for linespan in &lines[line_index..] {
            let entry_kind = EntryKind::try_from(linespan)?;

            let node;
            match entry_kind {
                EntryKind::Leaf => {
                    node = Self::parse_codefence_node_head(indent, depth, parent_path_str, entry_kind, line_index, linespan)?;
                }
                EntryKind::Branch => {
                    node = Self::parse_codefence_node_head(indent, depth, parent_path_str, entry_kind, line_index, linespan)?;
                }
                EntryKind::Continue => continue,
                EntryKind::None =>  {

                }
            }

            children.push(node);
        }

        Ok(children)
    }

    fn parse_codefence_node_head(indent: usize, depth: usize, parent_path_str: Option<&str>, entry_kind: EntryKind, line_index: usize, linespan: &(String, Span)) -> syn::Result<Self> {
        let (line, span) = linespan;
        let (path_str, name)= entry_kind
            .slice_after(line)
            .split_once(Self::NAME_SEPARATOR)
            .ok_or_else(|| syn_error!(*span, E_ENTRY_HEADER))?;

        let path_str = path_str.trim();
        let name = name.trim();

        Ok(Self {
            parent_path_str,
            path_str,
            name,
            subline,
            children: Vec::new(),
            depth,
            line_index
        })
    }

    fn parse_codefense_child(child: Self, lines: &[Linespan], indent: &str) -> syn::Result<Self> {
        todo!()
    }

}

struct Cursor {
    indent: usize,
    depth: usize,
    line_index: usize,
    char_index: usize
}

impl Cursor {
    const DEPTH_INDENT: usize = 4;

    fn new(indent: usize) -> Self {
        Cursor {
            indent,
            depth: 0,
            line_index: 1, // skip '.' line
            char_index: 0
        }
    }

    fn next_line(self) -> Self {
        Cursor {
            indent: self.indent,
            depth: self.depth,
            line_index: self.line_index + 1,
            char_index: 0
        }
    }

    fn next_depth(self) -> Self {
        let depth = self.depth + 1;
        Cursor {
            indent: self.indent,
            depth,
            line_index: self.line_index,
            char_index: self.indent + depth * Cursor::DEPTH_INDENT
        }
    }
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
    const CONTINUE_PREFIX: &'static str = "|";

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

impl<'a> ParserNode<'a> {
    const NAME_SEPARATOR: &'static str = " :: ";

}

impl ToTokens for HardpathMacroModelNode {
    fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}
