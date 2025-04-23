mod error;
mod parser;

pub type Linespan = (String, proc_macro2::Span);

pub use error::msg::*;
pub use parser::*;
