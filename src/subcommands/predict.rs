use thiserror::Error as ThisError;
use crate::configuration::settings::{LLMCfg, RepositoryCfg, Prompt};
use crate::clients::repository;
use crate::clients::repository::RepositoryTrait;
use crate::models;


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
    pub args_id: Vec<u32>,
}

/// Draft of the comming features for argument relations prediction.
pub async fn predict_relations(cfg: PredictRelationCfg) -> Result<(), Error> {
    let mut repo_client = repository::Repository::new(&cfg.repo_cfg)
        .await?;
    let mut args = Vec::<models::Argument>::with_capacity(cfg.args_id.len());

    for arg_id in cfg.args_id {
        args.push(repo_client.retrieve_argument(arg_id).await?);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    mod predict_relations {
    }
}
