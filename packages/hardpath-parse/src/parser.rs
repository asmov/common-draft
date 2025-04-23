use std::cell::RefCell;
use proc_macro2::Span;
use asmov_common_hardpath_model::*;
use crate::*;

#[derive(Debug)]
pub(crate) struct ParserNode<'n> {
    pub(crate) id: usize,
    pub(crate) cursor: Cursor,
    pub(crate) path_kind: PathKind,
    pub(crate) path_name: &'n str,
    pub(crate) name: Option<&'n str>,
    pub(crate) subline: Option<&'n str>,
    pub(crate) parent_id: Option<usize>,
    pub(crate) children: Vec<ParserNode<'n>>,
}

impl<'n> ParserNode<'n> {
    pub fn to_tree(self) -> SoftpathTree {
        todo!()
    }
}

impl<'n> Into<SoftpathTree> for ParserNode<'n> {
    fn into(self) -> SoftpathTree {
        ParserNode::to_tree(self)
    }
}

pub struct HardpathParser<'p> {
    ident_span: Span,
    lines: &'p Vec<Linespan>,
    indent: usize,
    next_id: RefCell<usize>,
}

impl<'p> HardpathParser<'p> {
    const NAME_SEPARATOR: &'static str = " :: ";

    pub fn new(lines: &'p Vec<Linespan>, ident_span: Span) -> syn::Result<Self> {
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

    pub fn parse(self, name: &'p str, subline: &'p str) -> syn::Result<SoftpathTree> {
        let cursor = Cursor::new(self.indent);

        let children = self.parse_direct_children(cursor, 0)?
            .into_iter()
            .map(|child| self.parse_child(child))
            .collect::<syn::Result<Vec<_>>>()?;

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

        Ok(tree.into())
    }

    pub fn generate_id(&self) -> usize {
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() += 1;
        id
    }

    fn seek_from(line: &'p Linespan, cursor: &Cursor) -> syn::Result<&'p str> {
        let (line, span) = line;

        if line.len() < cursor.char_index {
            return syn_err!(*span, E_LINE_INDENT);
        }

        Ok(&line[cursor.char_index..])
    }

    fn parse_direct_children(&self, mut cursor: Cursor, parent_id: usize) -> syn::Result<Vec<ParserNode<'p>>> {
        if cursor.next_depth(self.lines).is_none() {
            return Ok(Vec::new());
        };

        let mut children = Vec::new();
        while let Some(linespan) = cursor.next_line(self.lines) {
            let entry_kind = EntryKind::try_from(linespan)?;

            match entry_kind {
                EntryKind::Leaf | EntryKind::Branch => {
                    let entry_cursor = cursor.clone();
                    let (path_kind, path_name, name, subline) = self.parse_node_head(&mut cursor, &linespan, entry_kind)?;
                    let node = ParserNode {
                        id: self.generate_id(),
                        path_kind,
                        cursor: entry_cursor,
                        path_name,
                        name,
                        subline,
                        parent_id: Some(parent_id),
                        children: Vec::new(),
                    };

                    children.push(node);
                },
                EntryKind::Continue | EntryKind::Indent => continue, // todo: check for invalid characters
            }

        }

        Ok(children)
    }

    fn parse_node_head(&self, cursor: &mut Cursor, linespan: &'p Linespan, entry_kind: EntryKind) -> syn::Result<(PathKind, &'p str, Option<&'p str>, Option<&'p str>)> {
        let (line, span) = linespan;
        let path_name = entry_kind.after_slice(line);

        let (path_name, name) = if let Some((path_name, name)) = path_name.split_once(Self::NAME_SEPARATOR) {
            ( path_name.trim(), Some(name.trim()) )
        } else {
            ( path_name.trim(), None )
        };

        //todo: check for invalid characters
        if path_name.is_empty() {
            return syn_err!(*span, E_ENTRY_HEADER);
        }

        let path_kind = if path_name.ends_with('/') {
            PathKind::Directory
        } else {
            PathKind::File
        };

        let subline = if let Some(linespan) = cursor.peek_next_line(self.lines) {
            if let Ok(EntryKind::Indent) = EntryKind::try_from(linespan) {
                let (line, span) = cursor.next_line(self.lines).expect("Should exist");
                Some(entry_kind.after_slice(line).trim())
            } else {
                None
            }
        } else {
            None
        };

        Ok((path_kind, path_name, name, subline))
    }

    fn parse_child<'n>(&self, _child: ParserNode) -> syn::Result<ParserNode<'n>> {
        todo!()
    }

}

#[derive(Debug, Clone)]
pub(crate) struct Cursor {
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

    fn get_line<'a>(&self, lines: &'a Vec<Linespan>) -> Option<&'a Linespan> {
        lines.get(self.line_index)
    }

    fn peek_next_line<'a>(&self, lines: &'a Vec<Linespan>) -> Option<&'a Linespan> {
        lines.get(self.line_index + 1)
    }

    fn next_line<'a> (&mut self, lines: &'a Vec<Linespan>) -> Option<&'a Linespan> {
        let line_index = self.line_index + 1;
        let line = lines.get(line_index)?;
        self.line_index = line_index;
        self.char_index = self.indent + self.depth * Cursor::DEPTH_INDENT;
        Some(line)
    }

    fn next_depth<'a>(&mut self, lines: &'a Vec<Linespan>) -> Option<&'a str> {
        let depth = self.depth + 1;
        let char_index = self.indent + depth * Cursor::DEPTH_INDENT;

        let (line, _) = if let Some(linespan) = lines.get(self.line_index) {
            linespan
        } else {
            return None;
        };

        if line.len() <= char_index {
            return None;
        } else {
            self.char_index = char_index;
            Some(&line[char_index..])
        }
    }
}

enum EntryKind {
    Indent,
    Leaf,
    Branch,
    Continue
}

impl EntryKind {
    const INDENT_PREFIX: &'static str = "    ";
    const LEAF_PREFIX: &'static str = "|-- ";
    const BRANCH_PREFIX: &'static str = "|-+ ";
    const CONTINUE_PREFIX: &'static str = "|   ";

    fn after_slice<'a,'b>(&'a self, line: &'b str) -> &'b str {
        match self {
            EntryKind::Indent
            | EntryKind::Leaf
            | EntryKind::Branch
            | EntryKind::Continue => &line[5..],
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
            line if line.starts_with(Self::INDENT_PREFIX) => Ok(EntryKind::Indent),
            _ => syn_err!(*span, E_ENTRY_KIND)
        }
    }
}

impl<'a> ParserNode<'a> {

}
