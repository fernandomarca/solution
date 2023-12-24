#![allow(dead_code)]
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Serialize, Debug, Clone)]
pub enum CustomError {
    #[error("{0}")]
    Error(String),

    #[error("ApiError: {0}")]
    ApiError(String),

    #[error("RepositoryError: {0}")]
    RepositoryError(String),
}

#[cfg(test)]
mod custom_error_tests {
    use super::*;

    #[test]
    fn test_custom_error() {
        let error = CustomError::Error("some error".to_string());
        assert_eq!(error.to_string(), "some error");
    }

    #[test]
    fn test_custom_api_error() {
        let error = CustomError::ApiError("ApiError".to_string());
        assert_eq!(error.to_string(), "ApiError: ApiError");
    }

    #[test]
    fn test_custom_repository_error() {
        let error = CustomError::RepositoryError("RepositoryError".to_string());
        assert_eq!(error.to_string(), "RepositoryError: RepositoryError");
    }
}

// impl<'a> std::fmt::Display for CustomError<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             CustomError::Error(message) => write!(f, "Error: {}", message),
//             CustomError::ApiError(message) => write!(f, "ApiError: {}", message),
//             CustomError::RepositoryError(message) => write!(f, "RepositoryError: {}", message),
//         }
//     }
// }

// impl<'a> std::error::Error for CustomError<'a> {}
