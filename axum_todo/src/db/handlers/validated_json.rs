use axum::{
    async_trait,
    extract::FromRequest,
    http::{Request, StatusCode},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedJson<T>
where
    S: Send + Sync,
    B: Send + 'static + http_body::Body,
    B::Data: Send,
    B::Error: Into<BoxError>,
    T: DeserializeOwned + Validate,
{
    type Rejection = (StatusCode, String);
    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                tracing::debug!("rejection Json::<T>::from_request() {}", rejection);
                let message = format!("Json parse error: [{}]", rejection);
                (StatusCode::BAD_REQUEST, message)
            })?;
        value.validate().map_err(|rejection| {
            tracing::debug!("rejection value.validate() {}", rejection);
            let message = format!("Validation error: [{}]", rejection).replace('\n', ", ");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}
