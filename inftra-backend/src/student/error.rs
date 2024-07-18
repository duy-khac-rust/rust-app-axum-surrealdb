// pub type ResultStudent<T> = std::result::Result<T, ErrorStudent>; 

// #[derive(Debug)]
// pub enum ErrorStudent {
//     ErrorNotFound(surrealdb::Error),
// }

// impl std::fmt::Display for ErrorStudent {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self:?}")
//     }
// }

// impl From<surrealdb::Error> for ErrorStudent {
//     fn from(_value: surrealdb::Error) -> Self {
//         Self::ErrorNotFound(surrealdb::Error::NOT_FOUND)
//     }
// }

// impl std::error::Error for ErrorStudent {}
