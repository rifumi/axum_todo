use crate::db::repositories::todo_repository::CreateTodo;
use crate::db::repositories::todo_repository::TodoRepository;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
pub async fn create_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);
    (StatusCode::CREATED, Json(todo))
}
