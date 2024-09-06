use std::fmt::{self, Display, Formatter};
use std::io;
use crate::http::request::{HttpRequest, Version};

pub struct HttpResponse {
    pub version: Version,
    pub status: ResponseStatus,
    pub content_length: usize,
    pub accept_ranges: AcceptRanges,
    pub response_body: String,
    pub current_path: String,
    pub package_name: String,
    pub package_version: String,
}

impl HttpResponse {
    pub fn new(request: HttpRequest) -> io::Result<HttpResponse> {
        let version = Version::V2_0;
        let status = ResponseStatus::OK;
        let response_body = r#"
[package]
name = "simple-http"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;

        let content_length = response_body.len(); // Length of the response body

        let response_header = format!(
            "{} {}\r\n\
             Content-Length: {}\r\n\
             {}\r\n\
             X-Package-Name: {}\r\n\
             X-Package-Version: {}\r\n\r\n",
            version,
            status,
            content_length,
            AcceptRanges::None,
            "simple-http",
            "0.1.0"
        );

        Ok(HttpResponse {
            version,
            status,
            content_length,
            accept_ranges: AcceptRanges::None,
            response_body: format!("{}{}", response_header, response_body),
            current_path: request.resource.path.clone(),
            package_name: "simple-http".to_string(),
            package_version: "0.1.0".to_string(),
        })
    }
}

#[derive(Debug)]
pub enum ResponseStatus {
    OK = 200,
    NotFound = 404,
}

impl Display for ResponseStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ResponseStatus::OK => "200 OK",
            ResponseStatus::NotFound => "404 Not Found",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub enum AcceptRanges {
    Bytes,
    None,
}

impl Display for AcceptRanges {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            AcceptRanges::Bytes => "Accept-Ranges: bytes",
            AcceptRanges::None => "Accept-Ranges: none",
        };
        write!(f, "{}", msg)
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}\r\n\
             Content-Length: {}\r\n\
             {}\r\n\
             X-Package-Name: {}\r\n\
             X-Package-Version: {}\r\n\r\n\
             {}",
            self.version,
            self.status,
            self.content_length,
            self.accept_ranges,
            self.package_name,
            self.package_version,
            self.response_body
        )
    }
}
