use neo4rs::{query, Graph};
use thiserror::Error as ThisError;
use crate::configuration::settings;
use crate::models;
use crate::clients::repository;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("neo4j error: {0}")]
    Neo4jError(#[from] neo4rs::Error),

    #[error("neo4j deserialization error: {0}")]
    Neo4jDeError(#[from] neo4rs::DeError),

    #[error("no argument found for given request")]
    NoArgumentFound,

    #[error("no relation found for given request")]
    NoRelationFound,
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

    async fn retrieve_argument(&mut self, arg_id: u32) -> Result<models::Argument, repository::Error> {
        let client = self.client.clone();
        let query = query("MATCH (p:Argument) WHERE ID(p) = $id RETURN p")
            .param("id", arg_id);

        let mut result = client.execute(query).await.unwrap();

        while let Ok(Some(row)) = result.next().await {
            let node: neo4rs::Node = row.get("p").map_err(Error::from)?;

            return Ok(
                models::Argument::with_id(
                    node.id().try_into().unwrap(),
                    models::SummarizedInfo{
                        title: node.get::<String>("title").map_err(Error::from)?,
                        summary: node.get::<String>("summary").map_err(Error::from)?,
                    },
                    node.get::<String>("raw").map_err(Error::from)?,
                )
            )
        }

        Err(repository::Error::Neo4j(Error::NoArgumentFound))
    }

    async fn add_relation(&mut self, relation: models::Relation) -> Result<(), repository::Error> {
        let mut txn = self.client.start_txn().await
            .map_err(Error::from)?;

        txn.run_queries([
            query(format!("MATCH (a:Argument), (b:Argument) \
                    WHERE ID(a) = $id_a AND ID(b) = $id_b \
                    CREATE (a)-[:{}]->(b)", relation.relation_type.to_str()).as_str())
                .param("id_a", relation.arg_a_id)
                .param("id_b", relation.arg_b_id)
                //.param("confidence", relation.confidence.to_string())
                //.param("explanation", relation.explanation),
        ]).await.map_err(Error::from)?;

        txn.commit().await.map_err(Error::from)?;

        Ok(())
    }

    async fn retrieve_relation(&mut self, arg_a_id: u32, arg_b_id: u32) -> Result<models::Relation, repository::Error> {
        let client = self.client.clone();
        let query = query("MATCH (a:Argument)-[r]->(b:Argument) WHERE id(a) = $id_a AND id(b) = $id_b RETURN r")
            .param("id_a", arg_a_id)
            .param("id_b", arg_b_id);

        let mut result = client.execute(query).await.unwrap();

        while let Ok(Some(row)) = result.next().await {
            let relation: neo4rs::Relation = row.get("r").map_err(Error::from)?;

            return Ok(
                models::Relation{
                    arg_a_id: arg_a_id,
                    arg_b_id: arg_b_id,
                    relation_type: models::RelationType::from(&relation.typ()),
                    confidence: 1_f32,
                    explanation: String::from(""),
                }
            )
        }

        Err(repository::Error::Neo4j(Error::NoRelationFound))
    }
}

