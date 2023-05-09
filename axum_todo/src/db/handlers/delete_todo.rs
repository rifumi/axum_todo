use crate::db::repositories::todo_repository::TodoRepository;
use axum::{extract::Path, http::StatusCode, Extension};
use std::sync::Arc;

/// Returns StatusCode::NOT_FOUND if the delete() is the Err(),
/// otherwise returns the StatusCode::NO_CONTENT.
pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    repository
        .delete(id)
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}
