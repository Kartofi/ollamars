use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default,Clone)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    #[default]
    String,
    Integer,
    Number,
    Boolean,
}
#[derive(Serialize, Deserialize, Debug, Default,Clone)]
#[serde(rename_all = "lowercase")]
pub enum FormatType {
    #[default]
    Object
}
#[derive(Serialize, Deserialize, Debug, Default,Clone)]
pub struct FieldTypeInfo {
    #[serde(rename = "type")]
    pub field_type: FieldType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub enum_str: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub enum_int: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug, Default,Clone)]
pub struct OutputFormat {
    #[serde(rename = "type")]
    pub output_type: FormatType,

    pub properties: HashMap<String, FieldTypeInfo>,
    pub required: Vec<String>,
}
impl OutputFormat {
    pub fn new(
        properties: HashMap<String, FieldTypeInfo>,
        required_fields: Vec<String>,
    ) -> OutputFormat {
        OutputFormat {
            output_type: FormatType::Object,
            properties,
            required: required_fields,
        }
    }
  }
