use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HuggingFaceModelReference {
    pub filename: String,
    pub repo_id: String,
    pub revision: String,
}
