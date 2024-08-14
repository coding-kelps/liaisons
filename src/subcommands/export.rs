use std::path::PathBuf;
use serde::Serialize;
use serde_json;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use thiserror::Error as ThisError;
use crate::configuration::settings::RepositoryCfg;
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

pub struct ExportCfg {
    pub repo_cfg: RepositoryCfg,
    pub args_id: Vec<u32>,
    pub file_path: PathBuf,
}

#[derive(Serialize)]
struct ExportOutput {
    arguments: Vec::<models::Argument>,
    relations: Vec::<models::Relation>,
}

pub async fn export(cfg: ExportCfg) -> Result<(), Error> {
    let mut repo_client = repository::Repository::new(&cfg.repo_cfg)
        .await?;
    let mut args = Vec::<models::Argument>::with_capacity(cfg.args_id.len());
    let mut rels = Vec::<models::Relation>::with_capacity(cfg.args_id.len() * cfg.args_id.len());

    for arg_id in cfg.args_id {
        args.push(repo_client.retrieve_argument(arg_id).await?);
    }

    for arg_a in args.iter() {
        for arg_b in args.iter() {
            if arg_a.id == arg_b.id {
                continue
            } else {
                rels.push(repo_client.retrieve_relation(arg_a.id.clone().unwrap(), 
                    arg_b.id.clone().unwrap()).await?);
            }
        }
    }

    let json = serde_json::to_string_pretty(&ExportOutput{
        arguments: args,
        relations: rels,
    })?;

    let mut file = File::create(cfg.file_path).await?;

    file.write_all(json.as_bytes()).await?;
    file.flush().await?;

    Ok(())
}
