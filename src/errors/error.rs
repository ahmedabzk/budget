use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub struct CustomError(anyhow::Error);

impl<E> From<E> for CustomError
where
    E: Into<anyhow::Error> + std::error::Error,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        );
        (
            StatusCode::BAD_REQUEST,
            format!("missing credentials: {}", self.0),
        );
        (
            StatusCode::FORBIDDEN,
            format!("Not authenticated: {}", self.0),
        )
            .into_response()
    }
}
