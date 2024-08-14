use thiserror::Error as ThisError;
use crate::clients::llm::ClientTrait;
use crate::configuration::settings::{LLMCfg, RepositoryCfg, Prompt};
use crate::clients::{repository, llm};
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

    #[error("llm client error: {0}")]
    LLMError(#[from] llm::Error),
}

pub struct PredictRelationCfg {
    pub llm_cfg: LLMCfg,
    pub repo_cfg: RepositoryCfg,
    pub prompt: Prompt,
    pub args_id: Vec<u32>,
}

pub async fn predict_relations(cfg: PredictRelationCfg) -> Result<(), Error> {
    let mut repo_client = repository::Repository::new(&cfg.repo_cfg)
        .await?;
    let mut args = Vec::<models::Argument>::with_capacity(cfg.args_id.len());

    for arg_id in cfg.args_id {
        args.push(repo_client.retrieve_argument(arg_id).await?);
    }

    let llm_client = llm::Client::new(&cfg.llm_cfg);

    for arg_a in args.iter() {
        for arg_b in args.iter() {
            if arg_a.id == arg_b.id {
                continue
            } else {
                let relation = llm_client.predict(&cfg.prompt, arg_a, arg_b).await?;

                repo_client.add_relation(relation).await?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    mod predict_relations {
    }
}
