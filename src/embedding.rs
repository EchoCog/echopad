use anyhow::anyhow;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::embedding_normalization_method::EmbeddingNormalizationMethod;
use crate::normalization::l2;
use crate::normalization::rms_norm;

#[derive(Debug, Deserialize, Serialize)]
pub struct Embedding {
    pub embedding: Vec<f32>,
    pub normalization_method: EmbeddingNormalizationMethod,
    pub source_document_id: String,
}

impl Embedding {
    pub fn normalize(self, normalization_method: &EmbeddingNormalizationMethod) -> Result<Self> {
        if !self
            .normalization_method
            .can_transform_to(normalization_method)
        {
            return Err(anyhow!(
                "Cannot transform from {:?} to {normalization_method:?}",
                self.normalization_method
            ));
        }

        if self
            .normalization_method
            .needs_transformation_to(normalization_method)
        {
            return Ok(self);
        }

        Ok(Self {
            embedding: match normalization_method {
                EmbeddingNormalizationMethod::None => self.embedding,
                EmbeddingNormalizationMethod::L2 => l2(&self.embedding),
                EmbeddingNormalizationMethod::RmsNorm { epsilon } => {
                    rms_norm(&self.embedding, *epsilon)
                }
            },
            normalization_method: normalization_method.clone(),
            source_document_id: self.source_document_id.clone(),
        })
    }
}
