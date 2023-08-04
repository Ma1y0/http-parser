#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl HttpMethod {
    pub fn from(s: impl Into<String>) -> Option<Self> {
        let s = s.into();
        let s = s.as_str();
        match s {
            "GET" => Some(Self::GET),
            "HEAD" => Some(Self::HEAD),
            "POST" => Some(Self::POST),
            "PUT" => Some(Self::PUT),
            "DELETE" => Some(Self::DELETE),
            "CONNECT" => Some(Self::CONNECT),
            "OPTIONS" => Some(Self::OPTIONS),
            "TRACE" => Some(Self::TRACE),
            "PATCH" => Some(Self::PATCH),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GET => "GET",
            Self::HEAD => "HEAD",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::CONNECT => "CONNECT",
            Self::OPTIONS => "OPTIONS",
            Self::TRACE => "TRACE",
            Self::PATCH => "PATCH",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_method_from() {
        match HttpMethod::from("GET") {
            Some(a) => assert_eq!(HttpMethod::GET, a),
            None => assert!(false),
        }
    }

    #[test]
    fn http_method_as_str() {
        assert_eq!("GET", HttpMethod::GET.as_str());
    }
}
