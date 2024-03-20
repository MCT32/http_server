pub mod error;

use std::path::PathBuf;

use error::{HttpPathParseError, HttpQueryListParseError, HttpQueryParseError, HttpRequestLineParseError, HttpRequestParseError, HttpVersionParseError};
use regex::Regex;


#[derive(Debug)]
pub struct HttpRequest {
    pub request_line: HttpRequestLine,
    pub headers: HttpHeaderList,
    pub body: String,
}

impl TryFrom<&str> for HttpRequest {
    type Error = error::HttpRequestParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();

        let request_line = lines.next().unwrap();
        
        let request_line = match request_line.try_into() {
            Ok(line) => line,
            Err(err) => return Err(HttpRequestParseError::RequestLine(err)),
        };

        let mut header_lines: Vec<&str> = vec![];

        loop {
            if let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }

                header_lines.append(vec![line].as_mut());
            } else {
                break;
            }
        }

        let headers: HttpHeaderList = header_lines.join("\n").as_str().try_into().unwrap();

        let body: String = lines.collect::<Vec<&str>>().join("\n");

        Ok(Self {
            request_line,
            headers,
            body,
        })
    }
}

#[derive(Debug)]
pub struct HttpRequestLine {
    pub method: HttpMethod,
    pub path: HttpPath,
    pub version: HttpVersion,
}

impl TryFrom<&str> for HttpRequestLine {
    type Error = error::HttpRequestLineParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tokens = value.split_whitespace();

        Ok(Self {
            method: tokens.next().expect("missing method").try_into().unwrap(),
            path: match tokens.next().expect("missing path").try_into() {
                Ok(path) => path,
                Err(err) => return Err(HttpRequestLineParseError::Path(err)),
            },
            version: match tokens.next().expect("missing version").try_into() {
                Ok(version) => version,
                Err(err) => return Err(HttpRequestLineParseError::Version(err)),
            },
        })
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    Extension(String),
}

impl TryFrom<&str> for HttpMethod {
    type Error = std::io::Error;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "GET" => HttpMethod::GET,
            "HEAD" => HttpMethod::HEAD,
            "POST" => HttpMethod::POST,
            other => HttpMethod::Extension(other.to_string()),
        })
    }
}

#[derive(Debug)]
pub struct HttpPath {
    pub path: PathBuf,
    pub queries: HttpQueryList,
}

impl TryFrom<&str> for HttpPath {
    type Error = error::HttpPathParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with("/") {
            return Err(HttpPathParseError::NoLeadingSlash);
        }

        match value.split_once("?") {
            None => {
                Ok(HttpPath {
                    path: match value.try_into() {
                        Ok(path) => path,
                        Err(err) => return Err(HttpPathParseError::Invalid(err)),
                    },
                    queries: HttpQueryList { queries: vec![] },
                })
            },
            Some((path, query)) => {
                Ok(HttpPath {
                    path: match path.try_into() {
                        Ok(path) => path,
                        Err(err) => return Err(HttpPathParseError::Invalid(err)),
                    },
                    queries: query.try_into().unwrap(),
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct HttpQueryList {
    pub queries: Vec<HttpQuery>,
}

impl TryFrom<&str> for HttpQueryList {
    type Error = HttpQueryListParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let queries = match value
            .split("&")
            .map(|x| x.try_into())
            .collect::<Result<Vec<HttpQuery>, HttpQueryParseError>>() {
                Ok(queries) => queries,
                Err(err) => return Err(HttpQueryListParseError::Invalid(err)),
        };

        Ok(Self {
            queries,
        })
    }
}

#[derive(Debug)]
pub struct HttpQuery {
    pub name: String,
    pub value: String,
}

impl TryFrom<&str> for HttpQuery {
    type Error = HttpQueryParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((name, value)) = value.split_once("=") {
            Ok(Self {
                name: name.to_string(),
                value: value.to_string(),
            })
        } else {
            Err(HttpQueryParseError::Invalid)
        }
    }
}

#[derive(Debug)]
pub struct HttpVersion {
    pub major: u8,
    pub minor: u8,
}

impl TryFrom<&str> for HttpVersion {
    type Error = error::HttpVersionParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let regex = Regex::new(r"^HTTP\/(\d+).(\d+)$").unwrap();

        let (_, captures): (&str, [&str; 2]) = match regex.captures(value) {
            Some(captures) => captures.extract(),
            None => return Err(HttpVersionParseError::Invalid),
        };

        Ok(Self {
            major: match captures[0].parse() {
                Ok(major) => major,
                Err(err) => return Err(HttpVersionParseError::MajorVersionParseError(err)),
            },
            minor: match captures[1].parse() {
                Ok(minor) => minor,
                Err(err) => return Err(HttpVersionParseError::MinorVersionParseError(err)),
            },
        })
    }
}

#[derive(Debug)]
pub struct HttpHeaderList {
    pub headers: Vec<HttpHeader>,
}

impl TryFrom<&str> for HttpHeaderList {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            headers: value.lines().map(|header| header.try_into().unwrap()).collect(),
        })
    }
}

#[derive(Debug)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl TryFrom<&str> for HttpHeader {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split_once(":") {
            Some((name, value)) => Ok(Self {
                name: name.to_string(),
                value: value.trim_start().to_string(),
            }),
            None => panic!("invalid header"),
        }
    }
}
