use std::cell::RefCell;
use proc_macro2::Span;
use asmov_common_hardpath_model::*;
use crate::*;

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

pub struct HardpathParser {
    ident_span: Span,
    lines: Vec<Linespan>,
    indent: usize,
    next_id: RefCell<usize>,
}

impl HardpathParser {
    pub fn new(lines: Vec<Linespan>, ident_span: Span) -> syn::Result<Self> {
        // first node is '.' and establishes identation
        let dot_line = &lines.get(0)
            .ok_or_else(|| syn_error!(ident_span, E_TREE_ROOT_NOT_FOUND))?
            .0;
        let indent = dot_line.chars().take_while(|c| *c == ' ').count();

        Ok(HardpathParser {
            ident_span,
            lines,
            indent,
            next_id: RefCell::new(1),
        })
    }

    pub fn parse<'a>(self, name: &'a str, subline: &'a str) -> syn::Result<ParserNode<'a>> {
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
        let mut cursor = cursor.next_depth();

        let mut children = Vec::new();
        while let Some(linespan) = cursor.next_line(self.lines) {
            let entry_kind = EntryKind::try_from(linespan)?;

            match entry_kind {
                EntryKind::Leaf | EntryKind::Branch => {
                    let node = self.parse_node_head(cursor, entry_kind)?;
                    children.push(node);
                },
                EntryKind::Continue => continue, // todo: check for invalid characters
                EntryKind::None => break, // todo: check for invalid characters
            }

        }

        Ok(children)
    }

    fn parse_node_head(&self, cursor: &mut Cursor, entry_kind: EntryKind) -> syn::Result<Self> {
        let (line, span) = cursor.line(self.lines);
        let (path_name, mut name)= entry_kind
            .slice_after(line)
            .split_once(Self::NAME_SEPARATOR);

        let path_name = path_name
            .ok_or_else(|| syn_error!(*span, E_ENTRY_HEADER))?
            .trim();

        if let Some(mut name) = name {
            name = name.trim();
        }

        let path_kind = if path_name.ends_with('/') {
            PathKind::Directory
        } else {
            PathKind::File
        };

        Ok(ParserNode {
            id: self.generate_id(),
            path_kind: PathKind::Directory,
            cursor: cursor.clone(),
            path_name: ".",
            name: Some(name),
            subline: Some(subline),
            parent_id: None,
            children,
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

    fn next_line<'a> (&mut self, lines: &'a Vec<Linespan>) -> Option<&'a Linespan> {
        let line_index = self.line_index + 1;
        if lines.len() < line_index {
            None
        } else {
            self.line_index = line_index;
            self.char_index = self.indent + self.depth * Cursor::DEPTH_INDENT;
            Some(&lines[line_index - 1])
        }
    }

    fn next_depth(self) -> Option<Self> {
        let depth = self.depth + 1;
        let char_index = self.indent + depth * Cursor::DEPTH_INDENT;
        Some(Cursor {
            indent: self.indent,
            depth,
            line_index: self.line_index,
            char_index
        })
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
