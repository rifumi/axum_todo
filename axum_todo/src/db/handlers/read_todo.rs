use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::db::repositories::todo_repository::TodoRepository;

/// Search todo by id
/// if the id is not found, the function returns Err(StatusCode::NOT_FOUND)
/// if the id is found, the function returns Ok((200, Json<Todo>))
pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo_items = repository.all();
    (StatusCode::OK, Json(todo_items))
}
