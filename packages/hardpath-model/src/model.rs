use std::path::PathBuf;

pub type HardpathTree = HardpathNode;
pub type SoftpathTree = SoftpathNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub enum PathKind {
    File,
    Directory,
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub struct SoftpathNode {
    pub id: usize,
    pub path_kind: PathKind,
    pub path_name: String,
    pub name: String,
    pub description: String,
    pub parent_id: Option<usize>,
    pub children: Vec<SoftpathNode>,
}


#[derive(Debug)]
pub struct HardpathNode {
    id: usize,
    path_kind: PathKind,
    path_name: &'static str,
    name: &'static str,
    description: &'static str,
    parent_id: Option<usize>,
    children: &'static [HardpathNode],
}

impl HardpathNode where Self: 'static {
    pub const fn path_kind(&self) -> PathKind {
        self.path_kind
    }

    pub fn path_name(&self) -> &'static str {
        self.path_name
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn description(&self) -> &'static str {
        self.description
    }

    pub fn parent_id(&self) -> Option<usize> {
        self.parent_id
    }

    pub fn parent(&self, tree: &'static HardpathTree) -> Option<&'static HardpathNode> {
        match self.parent_id {
            Some(parent_id) => {
                if tree.id == parent_id {
                    return Some(tree);
                }

                todo!()
            },
            None => None,
        }
    }

    pub fn find(&self, _path: &str) -> Option<&'static HardpathNode> {
        todo!()
    }

    pub fn children(&self) -> &'static [HardpathNode] {
        &self.children
    }

    pub fn relative_path(&self) -> PathBuf {
            PathBuf::from(self.path_name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static HardpathNode> {
        self.children.iter()
    }

    pub fn is_base(&self) -> bool {
        self.parent_id.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        static TREE: HardpathNode = HardpathNode {
            id: 0,
            path_kind: PathKind::Directory,
            path_name: ".",
            name: "Test Tree",
            description: "This is a test tree",
            parent_id: None,
            children: &[
                HardpathNode {
                    id: 1,
                    path_kind: PathKind::Directory,
                    path_name: "child1",
                    name: "Child 1",
                    description: "This is child 1",
                    parent_id: Some(0),
                    children: &[],
                },
                HardpathNode {
                    id: 2,
                    path_name: "child2",
                    path_kind: PathKind::Directory,
                    name: "Child 2",
                    description: "This is child 2",
                    parent_id: Some(0),
                    children: &[],
                },
            ],
        };

        TREE.iter().for_each(|node| {
            println!("Node: {}", node.name());
        });
    }
}
