use std::sync::Arc;

use axum::Json;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use tokio::net::ToSocketAddrs;

use crate::persistence::{self, Persister};

#[derive(Debug, serde::Deserialize)]
pub struct ExportParams {
    pub from: u64,
    pub to: u64,
}

async fn export_stats(
    db: State<Arc<persistence::MySqlPersister>>,
    Query(params): Query<ExportParams>,
) -> Response {
    match db.query_stats_by_time_range(params.from, params.to).await {
        Ok(stats) => (axum::http::StatusCode::OK, Json(stats)).into_response(),
        Err(err) => {
            log::error!("{}", err);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "failed to export stats",
            )
                .into_response()
        }
    }
}

pub struct APIServer {
    router: axum::Router,
}

impl APIServer {
    pub async fn new(db: Arc<crate::persistence::MySqlPersister>) -> Self {
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
