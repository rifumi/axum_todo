use crate::db::repositories::todo_repository::{TodoRepository, UpdateTodo};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use std::sync::Arc;

/// Returns Err(NOT_FOUND) if the update() is error,
/// otherwise returns the Ok value (201, Json<Todo>) of Result
pub async fn update_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(todo)))
}
