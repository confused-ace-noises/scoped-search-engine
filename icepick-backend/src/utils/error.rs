use rocket::{self, http::Status, response::{status, Responder}};
use thiserror;
use url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Value not of type of '{0}'")]
    XValueNotOfType(&'static str),

    #[error("(Rocket) Network error: {0:?}")]
    RocketNetwork(#[from] rocket::Error),

    #[error("(Reqwest) Network error: {0:?}")]
    ReqwestNetwork(#[from] reqwest::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error("Library error: {0:?}")]
    LibError(&'static str),

    #[error(transparent)]
    ElapsedTimeTokioError(tokio::time::error::Elapsed),
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            Error::XValueNotOfType(error) => status::Custom(Status::InternalServerError, error.to_string()).respond_to(request),
            Error::RocketNetwork(error) => status::Custom(Status::InternalServerError, error.to_string()).respond_to(request),
            Error::ReqwestNetwork(error) => status::Custom(Status::InternalServerError, error.to_string()).respond_to(request),
            Error::IO(error) => status::Custom(Status::InternalServerError, error.to_string()).respond_to(request),
            Error::UrlParse(parse_error) => status::Custom(Status::BadRequest, parse_error.to_string()).respond_to(request),
            Error::Regex(error) => status::Custom(Status::BadRequest, error.to_string()).respond_to(request),
            Error::SerdeError(error) => status::Custom(Status::BadRequest, error.to_string()).respond_to(request),
            Error::LibError(error) => status::Custom(Status::InternalServerError, error.to_string()).respond_to(request),
            Error::ElapsedTimeTokioError(elapsed) => status::Custom(Status::InternalServerError, elapsed.to_string()).respond_to(request),
        }
    }
}