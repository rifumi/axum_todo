use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: u64,
    pub username: String,
}

mod tests {
    #[tokio::test]
    async fn test_create_user() {
        use super::*;
        use crate::api::users::CreateUser;
        use crate::api::users::User;
        use crate::create_user;
        use axum::http::StatusCode;
        use axum::Json;

        let response = create_user(Json(CreateUser {
            username: "Takeshi".to_string(),
        }))
        .await
        .into_response();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body();
        let bytes = hyper::body::to_bytes(body).await.unwrap();
        let user: User = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(
            user,
            User {
                id: 1337,
                username: "Takeshi".to_string()
            }
        );
    }
}
