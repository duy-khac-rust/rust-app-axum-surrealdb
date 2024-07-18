pub type ResultSite<T> =  std::result::Result<T, ErrorSite>;

use axum::http::StatusCode;


#[derive(Debug)]
pub enum ErrorSite {
    ErrorNotFound(axum::http::StatusCode),

}

impl std::fmt::Display for ErrorSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<axum::http::StatusCode> for ErrorSite {
    fn from(_value: axum::http::StatusCode) -> Self {
        Self::ErrorNotFound(StatusCode::NOT_FOUND)
    }
}

impl std::error::Error for ErrorSite {}
