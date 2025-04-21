use quote::ToTokens;
use crate::*;

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub(crate) struct HardpathMacroModel {
    tree: HardpathRawNode
}

impl HardpathMacroModel {
    const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

    fn serialize(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        let bytes = bincode::encode_to_vec(self, Self::BINCODE_CONFIG)?;
        Ok(bytes)
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        let (model, _): (Self, _) = bincode::decode_from_slice(bytes, Self::BINCODE_CONFIG)?;
        Ok(model)
    }
}

#[derive(Debug)]
pub(crate) struct HardpathItem {
    syn_struct: syn::ItemStruct,
    model_bytes: Vec<u8>,
    tree_value_tokens: proc_macro2::TokenStream,
    macro_model: HardpathMacroModel,
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub(crate) struct HardpathRawNode {
    path_str: String,
    name: String,
    description: String,
    parent_path_str: Option<String>,
    children: Vec<HardpathRawNode>,
}

impl HardpathRawNode {
    fn parse_comments(const_ident: &syn::Ident, lines: Vec<String>) -> Result<Self, syn::Error> {
        let children = Vec::new();

        let tree_node = HardpathRawNode {
            path_str: ".".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            parent_path_str: None,
            children: children,
        };

        Ok(tree_node)
    }
}

impl ToTokens for HardpathRawNode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}
