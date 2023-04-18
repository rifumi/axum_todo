use axum::{
    routing::{
        get
        , post
    }
    , Router
};
use std::net::{ SocketAddr, Ipv4Addr };
use std::env;

#[tokio::main]
async fn main() {
    // logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // container port:3000にリクエストを受信した時のrouting設定
    let app = create_app();
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

mod api;
mod pages;

use pages::index::root;
use api::users::create_user;
fn create_app() -> Router {
    return Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body
        , http::{
            header
            , Method
            , Request
        }
    };
    use super::*;
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "<h1>Hello, world!</h1>");
    }

    use api::users::*;
    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder().uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "username": "Munetaka Murakami" }"#)).unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");
        assert_eq!(user, User { id: 1337, username: "Munetaka Murakami".to_string()});
    }
}