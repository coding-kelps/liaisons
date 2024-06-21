use thiserror::Error as ThisError;
use crate::configuration::settings::{LLMCfg, RepositoryCfg, Prompt};
use crate::clients::repository;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("repository error: {0}")]
    RepositoryError(#[from] repository::Error),
}

pub struct PredictRelationCfg {
    pub llm_cfg: LLMCfg,
    pub repo_cfg: RepositoryCfg,
    pub prompt: Prompt,
}

/// Draft of the comming features for argument relations prediction.
#[allow(unused_variables)]
pub async fn predict_relations(cfg: PredictRelationCfg) -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    mod predict_relations {
    }
}
