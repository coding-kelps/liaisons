use neo4rs::{Graph, query};
use thiserror::Error as ThisError;
use crate::configuration::settings;
use crate::models;
use crate::clients::repository;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("neo4j error: {0}")]
    Neo4jError(#[from] neo4rs::Error),
}

pub struct Neo4j {
    pub client: Graph,
    #[allow(dead_code)]
    uri: String,
    #[allow(dead_code)]
    user: String,
    #[allow(dead_code)]
    password: String,
}

impl Neo4j {
    pub async fn new(cfg: & settings::Neo4jCfg) -> Result<Self, Error> {
        let client = Graph::new(&cfg.uri, &cfg.user, &cfg.password).await?;

        Ok(Self {
            client: client,
            uri: cfg.uri.clone(),
            user: cfg.user.clone(),
            password: cfg.password.clone(),
        })
    }
}

impl repository::RepositoryTrait for Neo4j {
    async fn add_argument(&mut self, arg: models::Argument) -> Result<(), repository::Error> {
        // Trying some WRITE queries to Neo4j
        let mut txn = self.client.start_txn().await
            .map_err(Error::from)?;

        txn.run_queries([
            query("CREATE (p:Argument {title: $title, summary: $summary, raw: $raw})")
                .param("title", arg.summarized_info.title)
                .param("summary", arg.summarized_info.summary)
                .param("raw", arg.raw),
        ]).await.map_err(Error::from)?;

        txn.commit().await.map_err(Error::from)?;

        Ok(())
    }
}
