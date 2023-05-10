use crate::db::repositories::todo_repository_for_memory::TodoRepositoryForMemory;
use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use std::env;
use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    // logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository);
    // container port:3000にリクエストを受信した時のrouting設定
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

mod api;
mod db;
mod pages;

use crate::db::{
    handlers::{create_todo::*, delete_todo::*, read_todo::*, update_todo::*},
    repositories::todo_repository::TodoRepository,
};
use api::users::create_user;
use pages::index::root;
use std::sync::Arc;
fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/todos", post(create_todo::<T>).get(all_todo::<T>))
        .route(
            "/todos/:id",
            get(find_todo::<T>)
                .delete(delete_todo::<T>)
                .patch(update_todo::<T>),
        )
        .layer(Extension(Arc::new(repository)))
}

#[cfg(test)]
mod tests {
    use crate::db::repositories::todo_repository::{CreateTodo, Todo};
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
        response::Response,
    };

    use super::*;
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "<h1>Hello, world!</h1>");
    }

    use api::users::*;
    #[tokio::test]
    async fn should_return_user_data() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "username": "Munetaka Murakami" }"#))
            .unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");
        assert_eq!(
            user,
            User {
                id: 1337,
                username: "Munetaka Murakami".to_string()
            }
        );
    }

    #[tokio::test]
    async fn should_created_todo() {
        let expected = Todo::new(1, "should_return_created_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        let req = build_todo_req_with_json(
            "/todos",
            Method::POST,
            r#"{ "text": "should_return_created_todo" }"#.to_string(),
        );
        let res = create_app(repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn test_created_todo_validation_max() {
        let expected = "Over text length";
        let repository = TodoRepositoryForMemory::new();
        let req = build_todo_req_with_json(
            "/todos",
            Method::POST,
            r#"{ "text": "12345678901234567890123456789012345678901234567890123456789012345" }"#
                .to_string(),
        );
        let res = create_app(repository).oneshot(req).await.unwrap();
        let res_to_str = res_to_str(res).await;
        let res_str = res_to_str.as_str();
        dbg!(res_str);
        assert_eq!(true, res_str.contains(expected));
    }

    #[tokio::test]
    async fn test_created_todo_validation_min() {
        let expected = "Can not be empty";
        let repository = TodoRepositoryForMemory::new();
        let req = build_todo_req_with_json("/todos", Method::POST, r#"{ "text": "" }"#.to_string());
        let res = create_app(repository).oneshot(req).await.unwrap();
        let res_to_str = res_to_str(res).await;
        let res_str = res_to_str.as_str();
        dbg!(res_str);
        assert_eq!(true, res_str.contains(expected));
    }

    #[tokio::test]
    async fn should_find_todo() {
        let expected = Todo::new(1, "should_find_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("should_find_todo".to_string()));
        let req = build_todo_req_with_empty(Method::GET, "/todos/1");
        let res = create_app(repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_get_all_todos() {
        let expected = Todo::new(1, "should_get_all_todos".to_string());
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("should_get_all_todos".to_string()));
        let req = build_todo_req_with_empty(Method::GET, "/todos");
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Vec<Todo> = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instance. body:{}", body));
        assert_eq!(vec![expected], todo);
    }

    #[tokio::test]
    async fn should_update_todos() {
        let expected = Todo::new(1, "should_update_todo".to_string());
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("before_update_todo".to_string()));
        let req = build_todo_req_with_json(
            "/todos/1",
            Method::PATCH,
            r#"{
            "id": 1, "text": "should_update_todo", "completed": false
        }"#
            .to_string(),
        );
        let res = create_app(repository).oneshot(req).await.unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn test_update_todo_validation_max() {
        let expected = "Over text length";
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("before_update_todo".to_string()));
        let req = build_todo_req_with_json(
            "/todos/1",
            Method::PATCH,
            r#"{ "text": "12345678901234567890123456789012345678901234567890123456789012345" }"#
                .to_string(),
        );
        let res = create_app(repository).oneshot(req).await.unwrap();
        let res_to_str = res_to_str(res).await;
        let res_str = res_to_str.as_str();
        dbg!(res_str);
        assert_eq!(true, res_str.contains(expected));
    }

    #[tokio::test]
    async fn test_update_todo_validation_min() {
        let expected = "Can not be empty";
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("before_update_todo".to_string()));
        let req =
            build_todo_req_with_json("/todos/1", Method::PATCH, r#"{ "text": "" }"#.to_string());
        let res = create_app(repository).oneshot(req).await.unwrap();
        let res_to_str = res_to_str(res).await;
        let res_str = res_to_str.as_str();
        dbg!(res_str);
        assert_eq!(true, res_str.contains(expected));
    }

    #[tokio::test]
    async fn should_delete_todo() {
        let repository = TodoRepositoryForMemory::new();
        repository.create(CreateTodo::new("should_delete_todo".to_string()));
        let req = build_todo_req_with_empty(Method::DELETE, "/todos/1");
        let res = create_app(repository).oneshot(req).await.unwrap();
        assert_eq!(StatusCode::NO_CONTENT, res.status());
    }

    /// Returns Json request from path, method, json string
    fn build_todo_req_with_json(path: &str, method: Method, json_body: String) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json_body))
            .unwrap()
    }

    /// Returns Json request from path, method. body is empty.
    fn build_todo_req_with_empty(method: Method, path: &str) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .body(Body::empty())
            .unwrap()
    }

    /// Convert type Response to Todo
    async fn res_to_todo(res: Response) -> Todo {
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Todo = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instance. body:{}", body));
        todo
    }

    /// Convert type Response to String
    async fn res_to_str(res: Response) -> String {
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        body
    }
}
