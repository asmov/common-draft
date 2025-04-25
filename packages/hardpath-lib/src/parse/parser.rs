use std::cell::RefCell;
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseState {
    None,
    Active,
    Done
}

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
    pub fn into_tree(self) -> SoftpathTree {
        let children = self.children.into_iter().map(|child| child.into_tree()).collect();
        let tree = SoftpathTree {
            id: self.id,
            path_kind: self.path_kind,
            path_name: self.path_name.to_owned(),
            name: self.name.map(|s| s.to_owned()),
            subline: self.subline.map(|s| s.to_owned()),
            parent_id: self.parent_id,
            children,
        };

        tree
    }
}

impl<'n> Into<SoftpathTree> for ParserNode<'n> {
    fn into(self) -> SoftpathTree {
        ParserNode::into_tree(self)
    }
}

pub struct HardpathParser<'p> {
    lines: Vec<&'p str>,
    indent: usize,
    header_lines_trimmed: usize,
    next_id: RefCell<usize>,
}

impl<'p> HardpathParser<'p> {
    const ROOT_NODE_ID: usize = 0;
    const ROOT_NODE_PATH_NAME: &'static str = ".";

    pub fn new() -> Result<Self> {
        Ok(HardpathParser {
            lines: Vec::new(),
            indent: 0,
            header_lines_trimmed: 0,
            next_id: RefCell::new(1),
        })
    }

    pub fn parse(mut self, schema: &'p str) -> Result<SoftpathTree> {

        self.init_lines(schema)?;
        let mut cursor = Cursor::new();
        let mut name = None;
        let mut subline = None;

        let line = cursor.next_line(&self.lines)
            .ok_or_else(|| Error::InvalidSchema)?;

        // attempt to read optional name and subline
        if line != "." {
            name = Some(line);

            let line = cursor.next_line(&self.lines)
                .ok_or_else(|| Error::InvalidSchema)?;

            if line != "." {
                subline = Some(line);

                // ensure that the next line is '.'
                let line = cursor.next_line(&self.lines)
                    .ok_or_else(|| Error::InvalidSchema)?;

                if line != "." {
                    return Err(Error::TreeRootNotFound);
                }
            }
        }

        let root_node_cursor = cursor.clone();

        let children = self.parse_direct_children(cursor, Self::ROOT_NODE_ID)?
            .into_iter()
            .map(|child| self.parse_child(child))
            .collect::<Result<Vec<_>>>()?;

        let tree = ParserNode {
            id: Self::ROOT_NODE_ID,
            path_kind: PathKind::Directory,
            cursor: root_node_cursor,
            path_name: Self::ROOT_NODE_PATH_NAME,
            name,
            subline,
            parent_id: None,
            children,
        };

        Ok(tree.into())
    }

    fn generate_id(&self) -> usize {
        let id = *self.next_id.borrow();
        *self.next_id.borrow_mut() += 1;
        id
    }

    /// Sanitize input for processing while preserving line numbers and indentation for error reporting.
    ///
    /// Throws an error if opening indentation is inconsistent rather than letting that error occur later.
    fn init_lines(&mut self, schema: &'p str) -> Result<()> {
        // determine indent and top trim
        let mut content_parse = ParseState::None;
        let mut footer_parse = ParseState::None;
        let mut header_lines_trimmed = 0;
        let mut indent = 0;
        let mut lines = Vec::new();
        let mut list_lines = Vec::new();

        for (index, line) in schema.lines().enumerate() {
            let trimmed = line.trim();

            match content_parse {
                ParseState::None => {
                    if line.is_empty() {
                        header_lines_trimmed += 1;
                    } else {
                        content_parse = ParseState::Active;
                        indent = line.chars()
                            .take_while(|c| c.is_whitespace())
                            .count();

                        lines.push(trimmed);
                    }
                },
                ParseState::Active => {
                    if trimmed.is_empty() {
                        content_parse = ParseState::Done;
                    } else {
                        let line_indent = line.len() - line.trim_start().len();
                        if line_indent != indent {
                            return Err(Error::LineIndent(index));
                        }

                        lines.push(trimmed);
                    }
                },
                ParseState::Done => {
                    match footer_parse {
                        ParseState::None => {

                        },
                        ParseState::Active => {

                        },
                        ParseState::Done => {

                        }
                    }
                }
            }
        }

        self.indent = indent;
        self.header_lines_trimmed = header_lines_trimmed;

        Ok(())
    }

    fn parse_direct_children(&self, mut cursor: Cursor, parent_id: usize) -> Result<Vec<ParserNode<'p>>> {
        let mut children = Vec::new();
        while let Some(linespan) = cursor.next_line(&self.lines) {
            let entry_kind = EntryKind::try_from(linespan)?;

            match entry_kind {
                EntryKind::Leaf | EntryKind::Branch => {
                    let entry_cursor = cursor.clone();
                    let (path_kind, path_name, name, subline)
                        = self.parse_node_head(&mut cursor, &linespan, entry_kind)?;
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

    fn parse_node_head(&self, cursor: &mut Cursor, line: &'p str, entry_kind: EntryKind) -> Result<(PathKind, &'p str, Option<&'p str>, Option<&'p str>)> {
        let path_name = entry_kind.after_slice(line);

        let (path_name, name) = if let Some((path_name, name)) = path_name.split_once(Self::NAME_SEPARATOR) {
            ( path_name.trim(), Some(name.trim()) )
        } else {
            ( path_name.trim(), None )
        };

        //todo: check for invalid characters
        if path_name.is_empty() {
            return Err(Error::EntryHeader);
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
    depth: usize,
    line_index: usize,
    char_index: usize
}

impl Cursor {
    const DEPTH_INDENT: usize = 4;

    fn new() -> Self {
        Cursor {
            depth: 0,
            line_index: 0,
            char_index: 0
        }
    }

    fn get_line<'a>(&self, lines: &'a Vec<Linespan>) -> Option<&'a Linespan> {
        lines.get(self.line_index)
    }

    fn peek_next_line<'a>(&self, lines: &'a Vec<Linespan>) -> Option<&'a Linespan> {
        lines.get(self.line_index + 1)
    }

    fn seek_slice(line: &str, char_index: usize) -> Option<&str> {
        if line.len() <= char_index {
            None
        } else {
            Some(&line[char_index..])
        }
    }

    fn next_line<'a> (&mut self, lines: &'a Vec<&str>) -> Option<&'a str> {
        self.char_index = self.depth * Cursor::DEPTH_INDENT;
        let line = lines.get(self.line_index)?;
        let line = Self::seek_slice(line, self.char_index)?;
        self.line_index += 1;
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
