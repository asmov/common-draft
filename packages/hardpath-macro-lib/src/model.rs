use quote::ToTokens;

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub(crate) struct HardpathMacroModel {
    pub(crate) tree: HardpathRawNode
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

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub(crate) struct HardpathRawNode {
    pub(crate) path_str: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) parent_path_str: Option<String>,
    pub(crate) children: Vec<HardpathRawNode>,
}

impl ToTokens for HardpathRawNode {
    fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
        todo!()
    }
}
