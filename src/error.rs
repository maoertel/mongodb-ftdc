use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum Error {
    Diqwest(diqwest::error::Error),
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
    Io(std::io::Error),
    IndicatifTemplate(indicatif::style::TemplateError),
    InvalidJobState(String),
    Download(String),
    CheckJobStatus(String),
    CreateJob(String),
    ReplicaSetNotFound(String),
    MongoJob(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::Diqwest(e) => std::fmt::Display::fmt(e, f),
            Error::Reqwest(e) => std::fmt::Display::fmt(e, f),
            Error::Json(e) => std::fmt::Display::fmt(e, f),
            Error::Io(e) => std::fmt::Display::fmt(e, f),
            Error::InvalidJobState(e) => std::fmt::Display::fmt(e, f),
            Error::Download(e) => std::fmt::Display::fmt(e, f),
            Error::CheckJobStatus(e) => std::fmt::Display::fmt(e, f),
            Error::CreateJob(e) => std::fmt::Display::fmt(e, f),
            Error::ReplicaSetNotFound(e) => std::fmt::Display::fmt(e, f),
            Error::MongoJob(e) => std::fmt::Display::fmt(e, f),
            Error::IndicatifTemplate(e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl From<diqwest::error::Error> for Error {
    fn from(diqwest_error: diqwest::error::Error) -> Self {
        Error::Diqwest(diqwest_error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(reqwest_error: reqwest::Error) -> Self {
        Error::Reqwest(reqwest_error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(serde_json_error: serde_json::Error) -> Self {
        Error::Json(serde_json_error)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Error::Io(io_error)
    }
}

impl From<indicatif::style::TemplateError> for Error {
    fn from(template_error: indicatif::style::TemplateError) -> Self {
        Error::IndicatifTemplate(template_error)
    }
}
