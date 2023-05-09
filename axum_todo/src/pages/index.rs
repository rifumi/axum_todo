use axum::response::Html;

pub async fn root() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_root() {
        let s1 = root().await;
        let s = s1.0.to_owned();
        assert_eq!(s, "<h1>Hello, world!</h1>");
    }
}
