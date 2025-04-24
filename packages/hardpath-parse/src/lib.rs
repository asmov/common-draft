mod error;
mod parser;

pub type Linespan = (String, proc_macro2::Span);

pub use error::msg::*;
pub use parser::*;

use asmov_common_hardpath_model as model;

pub fn str_to_softpath_tree(_s: &str) -> Result<model::SoftpathTree, &'static str> {
    todo!()
}

pub fn softpath_tree_to_string(_softpath: &model::SoftpathTree) -> Result<String, &'static str> {
    todo!()
}
