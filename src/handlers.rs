use axum::{
    extract::{Path, State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use crate::models::Task;
use serde::{Deserialize, Serialize};
use crate::db;

#[derive(Clone)]
pub struct AppState {
    pub pool_url: String,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
}

pub async fn list_tasks(State(state): State<AppState>) -> impl IntoResponse {
    match db::get_all_tasks(&state.pool_url).await {
        Ok(tasks) => (StatusCode::OK, Json(tasks)).into_response(),
        Err(e) => {
            tracing::error!("list_tasks error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "db error").into_response()
        }
    }
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    let task = Task::new(payload.title, payload.description);
    match db::create_task(&state.pool_url, &task).await {
        Ok(_) => (StatusCode::CREATED, Json(task)).into_response(),
        Err(e) => {
            tracing::error!("create_task error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "db error").into_response()
        }
    }
}

pub async fn complete_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match db::mark_task_completed(&state.pool_url, &id).await {
        Ok(_) => (StatusCode::OK, "ok").into_response(),
        Err(e) => {
            tracing::error!("complete_task error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "db error").into_response()
        }
    }
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match db::delete_task_by_id(&state.pool_url, &id).await {
        Ok(_) => (StatusCode::OK, "deleted").into_response(),
        Err(e) => {
            tracing::error!("delete_task error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "db error").into_response()
        }
    }
}

