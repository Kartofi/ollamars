use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug)]
pub struct Model {
    pub name: String,
    pub size: u64,
    pub details: ModelInfo,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ModelInfo{
    pub family: String,
    pub parameter_size: String,
    pub quantization_level: String
}
