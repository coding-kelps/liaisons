mod neo4j;
use crate::models;
use crate::settings;
use thiserror::Error as ThisError;

pub enum Repository {
    Neo4j(neo4j::Neo4j),
}

/// The different kinds of error expected from a client.
#[derive(Debug, ThisError)]
pub enum Error {    
    #[error("Neo4j repository client error: {0}")]
    Neo4j(#[from] neo4j::Error),
}

impl Repository {
    pub async fn new(cfg: &settings::RepositoryCfg) -> Result<Self, Error> {
        match cfg {
            settings::RepositoryCfg::Neo4j(neo4j_cfg) =>
                Ok(Repository::Neo4j(neo4j::Neo4j::new(neo4j_cfg).await?)),
        }
    }
}

/// Trait defining all the Large Language Model client expected features for
/// this program.
pub trait RepositoryTrait {
    /// Summarize the underlying argument of a user generated web-content
    /// (e.g., Twitter Post)
    async fn add_argument(&mut self, arg: models::Argument) -> Result<(), Error>;

    async fn retrieve_argument(&mut self, arg_id: u32) -> Result<models::Argument, Error>;
}

impl RepositoryTrait for Repository {
    async fn add_argument(&mut self, arg: models::Argument) -> Result<(), Error> {
        match self {
            Repository::Neo4j(client) => client.add_argument(arg)
                .await,
        }
    }

    async fn retrieve_argument(&mut self, arg_id: u32) -> Result<models::Argument, Error> {
        match self {
            Repository::Neo4j(client) => client.retrieve_argument(arg_id)
                .await,
        }
    }
}
