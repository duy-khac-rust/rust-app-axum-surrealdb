pub type ResultInftra<T> = std::result::Result<T, ErrorInftra>;

#[derive(Debug)]
pub enum ErrorInftra {
    ErrorSurrealdb(surrealdb::Error),
}

impl std::fmt::Display for ErrorInftra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<surrealdb::Error> for ErrorInftra {
    fn from(value: surrealdb::Error) -> Self {
        Self::ErrorSurrealdb(value)
    }
}

impl std::error::Error for ErrorInftra {}
