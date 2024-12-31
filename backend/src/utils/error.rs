use rocket;
use thiserror;
use url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Value not of type of '{0}'")]
    XValueNotOfType(&'static str),

    #[error("Network error: {0:?}")]
    RocketNetwork(#[from] rocket::Error),

    #[error("Network error: {0:?}")]
    ReqwestNetwork(#[from] reqwest::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),

    #[error(transparent)]
    Regex(#[from] regex::Error),
}
