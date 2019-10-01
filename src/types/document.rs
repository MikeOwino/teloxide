use crate::types::PhotoSize;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<u32>,
}