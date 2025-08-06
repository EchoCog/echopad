use serde::Deserialize;
use serde::Serialize;

use crate::embedding_input_document::EmbeddingInputDocument;
use crate::embedding_normalization_method::EmbeddingNormalizationMethod;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GenerateEmbeddingBatchParams {
    pub input_batch: Vec<EmbeddingInputDocument>,
    pub normalization_method: EmbeddingNormalizationMethod,
    pub start_batch_index: usize,
}
