use rocket;
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
