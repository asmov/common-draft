use std::path::PathBuf;

pub type HardpathTreeNode = HardpathNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub enum PathKind {
    File,
    Directory,
}

pub struct HardpathNode {
    path_str: &'static str,
    name: &'static str,
    description: &'static str,
    parent_path_str: Option<&'static str>,
    children: &'static [HardpathNode],
}

impl HardpathNode where Self: 'static {
    pub fn path_str(&self) -> &'static str {
        self.path_str
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn description(&self) -> &'static str {
        self.description
    }

    pub fn parent_path_str(&self) -> Option<&'static str> {
        self.parent_path_str
    }

    pub fn parent(&self, tree: &'static HardpathTreeNode) -> Option<&'static HardpathNode> {
        match self.parent_path_str {
            Some(parent_path) => tree.find_child(parent_path),
            None => None,
        }
    }

    pub fn find_child(&self, path_str: &str) -> Option<&'static HardpathNode> {
        self.children.iter().find(|child| child.path_str == path_str)
    }

    pub fn children(&self) -> &'static [HardpathNode] {
        &self.children
    }

    pub fn relative_path(&self) -> PathBuf {
            PathBuf::from(self.path_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static HardpathNode> {
        self.children.iter()
    }

    pub fn is_base(&self) -> bool {
        self.parent_path_str.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        static TREE: HardpathNode = HardpathNode {
            path_str: ".",
            name: "Test Tree",
            description: "This is a test tree",
            parent_path_str: None,
            children: &[
                HardpathNode {
                    path_str: "child1",
                    name: "Child 1",
                    description: "This is child 1",
                    parent_path_str: Some("."),
                    children: &[],
                },
                HardpathNode {
                    path_str: "child2",
                    name: "Child 2",
                    description: "This is child 2",
                    parent_path_str: Some("."),
                    children: &[],
                },
            ],
        };

        TREE.iter().for_each(|node| {
            println!("Node: {}", node.name());
        });
    }
}
