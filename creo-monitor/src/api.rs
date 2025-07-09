use std::collections::HashMap;

use axum::Json;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use sqlx::MySqlPool;
use tokio::net::ToSocketAddrs;

use crate::persistence;

mod models;

#[derive(Debug, serde::Deserialize)]
pub struct ExportParams {
    pub from: u64,
    pub to: u64,
}

async fn export_stats(db: State<DB>, Query(params): Query<ExportParams>) -> Response {
    let mut body: HashMap<&'static str, serde_json::Value> = HashMap::default();
    match db.query_stats_by_time_range(params.from, params.to).await {
        Ok(stats) => {
            body.insert(
                "stats",
                serde_json::to_value(stats).expect("serialization failed"),
            );
        }
        Err(err) => {
            log::error!("Failed to query container stats: {}", err);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "failed to export stats",
            )
                .into_response();
        }
    }
    match db
        .query_metadata_by_time_range(params.from, params.to)
        .await
    {
        Ok(metadata) => {
            body.insert(
                "metadata",
                serde_json::to_value(metadata).expect("serialization failed"),
            );
        }
        Err(err) => {
            log::error!("Failed to query container metadata: {}", err);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "failed to export stats",
            )
                .into_response();
        }
    }

    (axum::http::StatusCode::OK, Json(body)).into_response()
}

pub struct APIServer {
    router: axum::Router,
}

impl APIServer {
    pub async fn new(db: DB) -> Self {
        let router = axum::Router::new()
            .route("/export", get(export_stats))
            .with_state(db);
        Self { router }
    }

    pub async fn listen(self, addr: impl ToSocketAddrs) {
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect("TCP Listener bind");
        axum::serve(listener, self.router.into_make_service())
            .await
            .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct DB {
    db: MySqlPool,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read database entry: {0}")]
    ReadError(#[source] sqlx::Error),
}

type Result<T> = std::result::Result<T, Error>;

impl DB {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }

    async fn query_stats_by_time_range(
        &self,
        from: u64,
        to: u64,
    ) -> Result<HashMap<models::ContainerIdentifier, Vec<models::ContainerStats>>> {
        let stats = sqlx::query_as::<_, persistence::ContainerStats>(
            r#"
            SELECT * FROM container_stats WHERE timestamp BETWEEN ? and ? ORDER BY container_id, machine_id, timestamp
        "#,
        )
        .bind(from)
        .bind(to)
        .fetch_all(&self.db)
        .await
        .map_err(Error::ReadError)?;
        let mut out: HashMap<models::ContainerIdentifier, Vec<models::ContainerStats>> =
            HashMap::default();

        for stat in stats {
            let id =
                models::ContainerIdentifier::new(stat.container_id.into(), stat.machine_id.into());

            out.entry(id).or_default().push(stat.into());
        }

        Ok(out)
    }

    async fn query_metadata_by_time_range(
        &self,
        from: u64,
        to: u64,
    ) -> Result<HashMap<models::ContainerIdentifier, models::ContainerMetadata>> {
        let metadata = sqlx::query_as::<_, persistence::ContainerMetadata>(
            r#"
SELECT container_id, machine_id, hostname, label_key, label_value
FROM container_metadata
WHERE container_id IN (
    SELECT DISTINCT container_id FROM container_stats
    WHERE timestamp BETWEEN ? AND ?
)
ORDER BY container_id, machine_id
"#,
        )
        .bind(from)
        .bind(to)
        .fetch_all(&self.db)
        .await
        .map_err(Error::ReadError)?;

        let mut out: HashMap<models::ContainerIdentifier, models::ContainerMetadata> =
            HashMap::default();

        for meta in metadata {
            let id =
                models::ContainerIdentifier::new(meta.container_id.into(), meta.machine_id.into());

            out.entry(id)
                .or_insert_with(|| models::ContainerMetadata {
                    hostname: meta.hostname,
                    labels: HashMap::default(),
                })
                .labels
                .insert(meta.label_key, meta.label_value);
        }

        Ok(out)
    }
}
