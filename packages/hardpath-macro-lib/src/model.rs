use quote::ToTokens;

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub(crate) struct HardpathMacroModel {
    pub(crate) tree: HardpathMacroModelNode
}

impl HardpathMacroModel {
    const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

    pub(crate) fn serialize(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        let bytes = bincode::encode_to_vec(self, Self::BINCODE_CONFIG)?;
        Ok(bytes)
    }

    pub(crate) fn deserialize(bytes: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        let (model, _): (Self, _) = bincode::decode_from_slice(bytes, Self::BINCODE_CONFIG)?;
        Ok(model)
    }
}

#[derive(Debug)]
pub(crate) struct HardpathItem {
    pub(crate) syn_struct: syn::ItemStruct,
    pub(crate) macro_model: HardpathMacroModel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub(crate) enum PathKind {
    File,
    Directory,
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub(crate) struct HardpathMacroModelNode {
    pub(crate) parent_path_str: Option<String>,
    pub(crate) path_str: String,
    pub(crate) path_kind: PathKind,
    pub(crate) name: String,
    pub(crate) subline: String,
    pub(crate) children: Vec<HardpathMacroModelNode>,
}

#[derive(Debug)]
pub(crate) struct HardpathRawNode<'a> {
    pub(crate) depth: usize,
    pub(crate) line_index: usize,
    pub(crate) path_str: &'a str,
    pub(crate) name: &'a str,
    pub(crate) subline: &'a str,
    pub(crate) parent_path_str: Option<&'a str>,
    pub(crate) children: Vec<HardpathRawNode<'a>>,
}
