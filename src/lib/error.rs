use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpRequestParseError {
    #[error("Error parsing http request line: {0}")]
    RequestLine(#[from] HttpRequestLineParseError),
}

#[derive(Error, Debug)]
pub enum HttpRequestLineParseError {
    #[error("{0}")]
    Version(#[from] HttpVersionParseError),
    #[error("Error parsing http request path: {0}")]
    Path(#[from] HttpPathParseError),
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
