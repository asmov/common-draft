pub mod msg {
    pub const E_CODEFENCE_NOT_FOUND: &str = "Docblock code-fence ```hardpath``` not found or empty";
    pub const E_TREE_NAME_NOT_FOUND: &str = "Hardpath tree name not found at first line of docblock";
    pub const E_TREE_SUBLINE_NOT_FOUND: &str = "Hardpath tree subline not found at second line of docblock";
}
