use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Model {
    pub name: String,
    pub size: u64,
    pub details: ModelInfo,

    /// Only when getting loaded models shall this be a value
    pub size_vram: Option<u64>,
    /// Only when getting loaded models shall this be a value
    pub expires_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct ModelInfo {
    pub family: String,
    pub parameter_size: String,
    pub quantization_level: String,
}
