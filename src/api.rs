use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::Extension,
};
use std::net::SocketAddr;
use crate::handlers::{list_tasks, create_task, complete_task, delete_task, AppState};

pub async fn run_server(database_url: &str, host: &str, port: u16) -> anyhow::Result<()> {
    let state = AppState {
        pool_url: database_url.to_string(),
    };

    let app = Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id/complete", put(complete_task))
        .route("/tasks/:id", delete(delete_task))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

