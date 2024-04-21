use std::fmt::Display;

pub const HTTP_VERSION_1_0: &str = "HTTP/1.0";
pub const HTTP_VERSION_1_1: &str = "HTTP/1.1";

// Commonly used HTTP headers
pub const HEADER_ACCEPT: &str = "Accept";
pub const HEADER_ACCEPT_CHARSET: &str = "Accept-Charset";
pub const HEADER_ACCEPT_ENCODING: &str = "Accept-Encoding";
pub const HEADER_ACCEPT_LANGUAGE: &str = "Accept-Language";
pub const HEADER_ACCEPT_RANGES: &str = "Accept-Ranges";
pub const HEADER_ACCESS_CONTROL_ALLOW_CREDENTIALS: &str = "Access-Control-Allow-Credentials";
pub const HEADER_ACCESS_CONTROL_ALLOW_HEADERS: &str = "Access-Control-Allow-Headers";
pub const HEADER_ACCESS_CONTROL_ALLOW_METHODS: &str = "Access-Control-Allow-Methods";
pub const HEADER_ACCESS_CONTROL_ALLOW_ORIGIN: &str = "Access-Control-Allow-Origin";
pub const HEADER_ACCESS_CONTROL_EXPOSE_HEADERS: &str = "Access-Control-Expose-Headers";
pub const HEADER_ACCESS_CONTROL_MAX_AGE: &str = "Access-Control-Max-Age";
pub const HEADER_ACCESS_CONTROL_REQUEST_HEADERS: &str = "Access-Control-Request-Headers";
pub const HEADER_ACCESS_CONTROL_REQUEST_METHOD: &str = "Access-Control-Request-Method";
pub const HEADER_AGE: &str = "Age";
pub const HEADER_ALLOW: &str = "Allow";
pub const HEADER_AUTHORIZATION: &str = "Authorization";
pub const HEADER_CACHE_CONTROL: &str = "Cache-Control";
pub const HEADER_CONNECTION: &str = "Connection";
pub const HEADER_CONTENT_TYPE: &str = "Content-Type";
pub const HEADER_CONTENT_LENGTH: &str = "Content-Length";
pub const HEADER_CONTENT_ENCODING: &str = "Content-Encoding";
pub const HEADER_CONTENT_DISPOSITION: &str = "Content-Disposition";
pub const HEADER_CONTENT_LANGUAGE: &str = "Content-Language";
pub const HEADER_CONTENT_LOCATION: &str = "Content-Location";
pub const HEADER_CONTENT_RANGE: &str = "Content-Range";
pub const HEADER_COOKIE: &str = "Cookie";
pub const HEADER_DATE: &str = "Date";
pub const HEADER_ETAG: &str = "ETag";
pub const HEADER_EXPECT: &str = "Expect";
pub const HEADER_EXPIRES: &str = "Expires";
pub const HEADER_HOST: &str = "Host";
pub const HEADER_IF_MATCH: &str = "If-Match";
pub const HEADER_IF_MODIFIED_SINCE: &str = "If-Modified-Since";
pub const HEADER_IF_NONE_MATCH: &str = "If-None-Match";
pub const HEADER_IF_RANGE: &str = "If-Range";
pub const HEADER_IF_UNMODIFIED_SINCE: &str = "If-Unmodified-Since";
pub const HEADER_LAST_MODIFIED: &str = "Last-Modified";
pub const HEADER_LOCATION: &str = "Location";
pub const HEADER_ORIGIN: &str = "Origin";
pub const HEADER_PRAGMA: &str = "Pragma";
pub const HEADER_PROXY_AUTHENTICATE: &str = "Proxy-Authenticate";
pub const HEADER_PROXY_AUTHORIZATION: &str = "Proxy-Author";
pub const HEADER_RANGE: &str = "Range";
pub const HEADER_REFERER: &str = "Referer";
pub const HEADER_RETRY_AFTER: &str = "Retry-After";
pub const HEADER_SERVER: &str = "Server";
pub const HEADER_SET_COOKIE: &str = "Set-Cookie";
pub const HEADER_STRICT_TRANSPORT_SECURITY: &str = "Strict-Transport-Security";
pub const HEADER_TE: &str = "TE";
pub const HEADER_TRAILER: &str = "Trailer";
pub const HEADER_TRANSFER_ENCODING: &str = "Transfer-Encoding";
pub const HEADER_UPGRADE: &str = "Upgrade";
pub const HEADER_USER_AGENT: &str = "User-Agent";
pub const HEADER_VARY: &str = "Vary";
pub const HEADER_VIA: &str = "Via";
pub const HEADER_WARNING: &str = "Warning";
pub const HEADER_WWW_AUTHENTICATE: &str = "WWW-Authenticate";
pub const HEADER_X_CONTENT_TYPE_OPTIONS: &str = "X-Content-Type-Options";
pub const HEADER_X_FORWARDED_FOR: &str = "X-Forwarded-For";
pub const HEADER_X_FORWARDED_PROTO: &str = "X-Forwarded-Proto";
pub const HEADER_X_FORWARDED_HOST: &str = "X-Forwarded-Host";
pub const HEADER_X_FORWARDED_PORT: &str = "X-Forwarded-Port";
pub const HEADER_X_FORWARDED_SERVER: &str = "X-Forwarded-Server";
pub const HEADER_X_FRAME_OPTIONS: &str = "X-Frame-Options";
pub const HEADER_X_POWERED_BY: &str = "X-Powered-By";
pub const HEADER_X_REQUESTED_WITH: &str = "X-Requested-With";
pub const HEADER_X_XSS_PROTECTION: &str = "X-XSS-Protection";
pub const HEADER_SEC_WEBSOCKET_KEY: &str = "Sec-WebSocket-Key";
pub const HEADER_SEC_WEBSOCKET_VERSION: &str = "Sec-WebSocket-Version";
pub const HEADER_SEC_WEBSOCKET_ACCEPT: &str = "Sec-WebSocket-Accept";
pub const HEADER_SEC_WEBSOCKET_PROTOCOL: &str = "Sec-WebSocket-Protocol";
pub const HEADER_SEC_WEBSOCKET_EXTENSIONS: &str = "Sec-WebSocket-Extensions";


// Commonly used Content-Types
pub const CONTENT_TYPE_IMAGE_PNG: &str = "image/png";
pub const CONTENT_TYPE_IMAGE_JPEG: &str = "image/jpeg";
pub const CONTENT_TYPE_IMAGE_GIF: &str = "image/gif";
pub const CONTENT_TYPE_TEXT_HTML: &str = "text/html";
pub const CONTENT_TYPE_TEXT_CSS: &str = "text/css";
pub const CONTENT_TYPE_TEXT_PLAIN: &str = "text/plain";
pub const CONTENT_TYPE_APPLICATION_JSON: &str = "application/json";
pub const CONTENT_TYPE_APPLICATION_XML: &str = "application/xml";
pub const CONTENT_TYPE_APPLICATION_OCTET_STREAM: &str = "application/octet-stream";
pub const CONTENT_TYPE_APPLICATION_X_WWW_FORM_URLENCODED: &str = "application/x-www-form-urlencoded";
pub const CONTENT_TYPE_APPLICATION_FORM_DATA: &str = "application/form-data";
pub const CONTENT_TYPE_MULTIPART_FORM_DATA: &str = "multipart/form-data";
pub const CONTENT_TYPE_APPLICATION_JAVASCRIPT: &str = "application/javascript";

// Commonly used connection types
pub const CONNECTION_CLOSE: &str = "close";
pub const CONNECTION_KEEP_ALIVE: &str = "keep-alive";
pub const CONNECTION_UPGRADE: &str = "upgrade";

pub const UPGRADE_WEBSOCKET: &str = "websocket";


#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Options,
    Head,
    Trace,
    Connect,
    Patch,
    Brew,
}

impl Method {
    pub fn from_str(method: &str) -> Option<Method> {
        match method {
            "GET" => Some(Method::Get),
            "POST" => Some(Method::Post),
            "PUT" => Some(Method::Put),
            "DELETE" => Some(Method::Delete),
            "OPTIONS" => Some(Method::Options),
            "HEAD" => Some(Method::Head),
            "TRACE" => Some(Method::Trace),
            "CONNECT" => Some(Method::Connect),
            "PATCH" => Some(Method::Patch),
            "BREW" => Some(Method::Brew),
            _ => None
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
            Method::Options => write!(f, "OPTIONS"),
            Method::Head => write!(f, "HEAD"),
            Method::Trace => write!(f, "TRACE"),
            Method::Connect => write!(f, "CONNECT"),
            Method::Patch => write!(f, "PATCH"),
            Method::Brew => write!(f, "BREW"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    // 100
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,

    // 200
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,

    // 300
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    SwitchProxy,
    TemporaryRedirect,
    PermanentRedirect,

    // 400
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    // 500
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,

    // Custom
    Custom(u16),

}

#[derive(Debug, PartialEq)]
pub enum StatusClass {
    Informational,
    Success,
    Redirection,
    ClientError,
    ServerError,
    Unknown,
}

impl Status {
    pub fn as_u16(&self) -> u16 {
        match *self {
            Status::Continue => 100,
            Status::SwitchingProtocols => 101,
            Status::Processing => 102,
            Status::EarlyHints => 103,
            Status::Ok => 200,
            Status::Created => 201,
            Status::Accepted => 202,
            Status::NonAuthoritativeInformation => 203,
            Status::NoContent => 204,
            Status::ResetContent => 205,
            Status::PartialContent => 206,
            Status::MultiStatus => 207,
            Status::AlreadyReported => 208,
            Status::IMUsed => 226,
            Status::MultipleChoices => 300,
            Status::MovedPermanently => 301,
            Status::Found => 302,
            Status::SeeOther => 303,
            Status::NotModified => 304,
            Status::UseProxy => 305,
            Status::SwitchProxy => 306,
            Status::TemporaryRedirect => 307,
            Status::PermanentRedirect => 308,
            Status::BadRequest => 400,
            Status::Unauthorized => 401,
            Status::PaymentRequired => 402,
            Status::Forbidden => 403,
            Status::NotFound => 404,
            Status::MethodNotAllowed => 405,
            Status::NotAcceptable => 406,
            Status::ProxyAuthenticationRequired => 407,
            Status::RequestTimeout => 408,
            Status::Conflict => 409,
            Status::Gone => 410,
            Status::LengthRequired => 411,
            Status::PreconditionFailed => 412,
            Status::PayloadTooLarge => 413,
            Status::URITooLong => 414,
            Status::UnsupportedMediaType => 415,
            Status::RangeNotSatisfiable => 416,
            Status::ExpectationFailed => 417,
            Status::ImATeapot => 418,
            Status::MisdirectedRequest => 421,
            Status::UnprocessableEntity => 422,
            Status::Locked => 423,
            Status::FailedDependency => 424,
            Status::TooEarly => 425,
            Status::UpgradeRequired => 426,
            Status::PreconditionRequired => 428,
            Status::TooManyRequests => 429,
            Status::RequestHeaderFieldsTooLarge => 431,
            Status::UnavailableForLegalReasons => 451,
            Status::InternalServerError => 500,
            Status::NotImplemented => 501,
            Status::BadGateway => 502,
            Status::ServiceUnavailable => 503,
            Status::GatewayTimeout => 504,
            Status::HTTPVersionNotSupported => 505,
            Status::VariantAlsoNegotiates => 506,
            Status::InsufficientStorage => 507,
            Status::LoopDetected => 508,
            Status::NotExtended => 510,
            Status::NetworkAuthenticationRequired => 511,
            Status::Custom(code) => code,
        }
    }

    pub fn reason_phrase(&self) -> &'static str {
        match *self {
            Status::Continue => "Continue",
            Status::SwitchingProtocols => "Switching Protocols",
            Status::Processing => "Processing",
            Status::EarlyHints => "Early Hints",
            Status::Ok => "OK",
            Status::Created => "Created",
            Status::Accepted => "Accepted",
            Status::NonAuthoritativeInformation => "Non-Authoritative Information",
            Status::NoContent => "No Content",
            Status::ResetContent => "Reset Content",
            Status::PartialContent => "Partial Content",
            Status::MultiStatus => "Multi-Status",
            Status::AlreadyReported => "Already Reported",
            Status::IMUsed => "IM Used",
            Status::MultipleChoices => "Multiple Choices",
            Status::MovedPermanently => "Moved Permanently",
            Status::Found => "Found",
            Status::SeeOther => "See Other",
            Status::NotModified => "Not Modified",
            Status::UseProxy => "Use Proxy",
            Status::SwitchProxy => "Switch Proxy",
            Status::TemporaryRedirect => "Temporary Redirect",
            Status::PermanentRedirect => "Permanent Redirect",
            Status::BadRequest => "Bad Request",
            Status::Unauthorized => "Unauthorized",
            Status::PaymentRequired => "Payment Required",
            Status::Forbidden => "Forbidden",
            Status::NotFound => "Not Found",
            Status::MethodNotAllowed => "Method Not Allowed",
            Status::NotAcceptable => "Not Acceptable",
            Status::ProxyAuthenticationRequired => "Proxy Authentication Required",
            Status::RequestTimeout => "Request Timeout",
            Status::Conflict => "Conflict",
            Status::Gone => "Gone",
            Status::LengthRequired => "Length Required",
            Status::PreconditionFailed => "Precondition Failed",
            Status::PayloadTooLarge => "Payload Too Large",
            Status::URITooLong => "URI Too Long",
            Status::UnsupportedMediaType => "Unsupported Media Type",
            Status::RangeNotSatisfiable => "Range Not Satisfiable",
            Status::ExpectationFailed => "Expectation Failed",
            Status::ImATeapot => "I'm a teapot",
            Status::MisdirectedRequest => "Misdirected Request",
            Status::UnprocessableEntity => "Unprocessable Entity",
            Status::Locked => "Locked",
            Status::FailedDependency => "Failed Dependency",
            Status::TooEarly => "Too Early",
            Status::UpgradeRequired => "Upgrade Required",
            Status::PreconditionRequired => "Precondition Required",
            Status::TooManyRequests => "Too Many Requests",
            Status::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            Status::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            Status::InternalServerError => "Internal Server Error",
            Status::NotImplemented => "Not Implemented",
            Status::BadGateway => "Bad Gateway",
            Status::ServiceUnavailable => "Service Unavailable",
            Status::GatewayTimeout => "Gateway Timeout",
            Status::HTTPVersionNotSupported => "HTTP Version Not Supported",
            Status::VariantAlsoNegotiates => "Variant Also Negotiates",
            Status::InsufficientStorage => "Insufficient Storage",
            Status::LoopDetected => "Loop Detected",
            Status::NotExtended => "Not Extended",
            Status::NetworkAuthenticationRequired => "Network Authentication Required",
            Status::Custom(_) => "Custom",
        }
    }

    pub fn class(&self) -> StatusClass {
        match self.as_u16() {
            100..=199 => StatusClass::Informational,
            200..=299 => StatusClass::Success,
            300..=399 => StatusClass::Redirection,
            400..=499 => StatusClass::ClientError,
            500..=599 => StatusClass::ServerError,
            _ => StatusClass::Unknown,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.as_u16(), self.reason_phrase())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub key: String,
    pub value: String,
}

impl Header {
    
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        Header {
            key: key.into(),
            value: value.into(),
        }
    }
    
    pub fn parse(header_line: String) -> Self {

        let mut parts = header_line.trim().splitn(2, ':');
        let key = parts.next().unwrap().to_string();
        let value = parts.next().unwrap_or("").trim().to_string();
        
        Header {
            key,
            value,
        }
    }
    
    pub fn key(&self) -> &str {
        &self.key
    }
    
    pub fn value(&self) -> &str {
        &self.value
    }
}