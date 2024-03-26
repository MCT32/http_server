use std::convert::Infallible;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpRequestParseError {
    #[error("Error parsing http request line: {0}")]
    RequestLine(#[from] HttpRequestLineParseError),
}

#[derive(Error, Debug)]
pub enum HttpRequestLineParseError {
    #[error("{0}")]
    Method(#[from] HttpMethodParseError),
    #[error("{0}")]
    Version(#[from] HttpVersionParseError),
    #[error("Error parsing http request path: {0}")]
    Path(#[from] HttpPathParseError),
    #[error("No method")]
    NoMethod,
    #[error("No path")]
    NoPath,
    #[error("No version")]
    NoVersion,
}

#[derive(Error, Debug)]
pub enum HttpMethodParseError {
    #[error("Invalid method")]
    Invalid,
}

#[derive(Error, Debug)]
pub enum HttpVersionParseError {
    #[error("Error parsing http version")]
    Invalid,
    #[error("Error parsing major http version: {0}")]
    MajorVersionParseError(std::num::ParseIntError),
    #[error("Error parsing minor http version: {0}")]
    MinorVersionParseError(std::num::ParseIntError),
}

#[derive(Error, Debug)]
pub enum HttpPathParseError {
    #[error("Error parsing http path, no leading slash")]
    NoLeadingSlash,
    #[error("{0}")]
    PathError(#[from] Infallible),
    #[error("{0}")]
    QueryError(#[from] HttpQueryListParseError),
}

#[derive(Error, Debug)]
pub enum HttpQueryParseError {
    #[error("Error parsing http query")]
    Invalid
}

#[derive(Error, Debug)]
pub enum HttpQueryListParseError {
    #[error("Error parsing http query list: {0}")]
    Invalid(#[from] HttpQueryParseError),
}

#[derive(Error, Debug)]
pub enum HttpHeaderParseError {
    #[error("Empty header")]
    EmptyHeader,
}
