use std::{convert::Infallible, error::Error, fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub enum HttpRequestParseError {
    RequestLine(HttpRequestLineParseError),
}

impl Error for HttpRequestParseError {}

impl Display for HttpRequestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestLine(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug)]
pub enum HttpRequestLineParseError {
    Version(HttpVersionParseError),
    Path(HttpPathParseError),
}

impl Error for HttpRequestLineParseError {}

impl Display for HttpRequestLineParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Version(err) => write!(f, "{}", err),
            Self::Path(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug)]
pub enum HttpVersionParseError {
    Invalid,
    MajorVersionParseError(ParseIntError),
    MinorVersionParseError(ParseIntError),
}

impl Error for HttpVersionParseError {}

impl Display for HttpVersionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, "HttpVersion: Invalid version string"),
            Self::MajorVersionParseError(err) => write!(f, "HttpVersion: Unable to parse major version: {}", err),
            Self::MinorVersionParseError(err) => write!(f, "HttpVersion: Unable to parse major version: {}", err),
        }
    }
}

#[derive(Debug)]
pub enum HttpPathParseError {
    Invalid(Infallible),
    NoLeadingSlash,
}

impl Error for HttpPathParseError {}

impl Display for HttpPathParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}", err),
            Self::NoLeadingSlash => write!(f, "HttpPath: No leading slash"),
        }
    }
}

#[derive(Debug)]
pub enum HttpQueryParseError {
    Invalid
}

impl Error for HttpQueryParseError {}

impl Display for HttpQueryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, "HttpQuery: Invalid query")
        }
    }
}

#[derive(Debug)]
pub enum HttpQueryListParseError {
    Invalid(HttpQueryParseError),
}

impl Error for HttpQueryListParseError {}

impl Display for HttpQueryListParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(err) => write!(f, "{}", err),
        }
    }
}
