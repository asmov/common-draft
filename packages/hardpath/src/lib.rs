use std::{ops::Deref, path::PathBuf};

pub type HardpathTreeNode = HardpathNode;
pub struct HardpathTree(std::sync::LazyLock<HardpathTreeNode>);

impl HardpathTree {
    pub const fn new() -> Self {
        Self(std::sync::LazyLock::new(|| HardpathTreeNode::new()))
    }
}

impl Deref for HardpathTree {
    type Target = HardpathTreeNode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct HardpathNode {
    path_str: &'static str,
    name: &'static str,
    description: &'static str,
    parent_path_str: Option<&'static str>,
    children: Vec<HardpathNode>,
}

impl HardpathTreeNode where Self: 'static {
    pub fn new() -> Self {
        let name = ".";

        let children = vec![HardpathNode::new_node(Some(&name))];

        HardpathNode {
            path_str: &name,
            name: "",
            description: "",
            parent_path_str: None,
            children,
        }
    }
}

impl HardpathNode where Self: 'static {
    const fn new_node(parent_path_str: Option<&'static str>) -> Self where Self: 'static {
        HardpathNode {
            path_str: "",
            name: "",
            description: "",
            parent_path_str,
            children: vec![],
        }
    }
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

    pub fn parent(&self, tree: &'static HardpathTreeNode) -> Option<&HardpathNode> {
        match self.parent_path_str {
            Some(parent_path) => tree.find_child(parent_path),
            None => None,
        }
    }

    pub fn find_child(&self, path_str: &str) -> Option<&HardpathNode> {
        self.children.iter().find(|child| child.path_str == path_str)
    }

    pub fn children(&self) -> &[HardpathNode] {
        &self.children
    }

    pub fn relative_path(&self) -> PathBuf {
            PathBuf::from(self.path_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = &HardpathNode> {
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
        static TREE: HardpathTree = HardpathTree::new();

        TREE.iter().for_each(|node| {
            println!("Node: {}", node.name());
        });
    }
}
