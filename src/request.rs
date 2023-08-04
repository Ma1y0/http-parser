use crate::{HttpMethod, ParsingError};
use std::collections::HashMap;

pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub version: f32,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn from(s: &str) -> Result<Self, ParsingError> {
        let s: Vec<&str> = s.lines().collect();
        let mut body = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();

        if s.is_empty() {
            return Err(ParsingError::new("Input is empty"));
        }

        // GET / HTTP/1.1
        let first_line: Vec<&str> = match &s.first() {
            Some(a) => a.split_whitespace().collect(),
            None => return Err(ParsingError::new("There isn't a first line")),
        };

        // Checks if first line have 3 args
        if first_line.len() != 3 {
            return Err(ParsingError::new("Not enough args in first line"));
        }

        // Extracts HTTP Method
        let method = match HttpMethod::from(first_line[0]) {
            Some(a) => a,
            None => return Err(ParsingError::new("Invalid HTTP method")),
        };

        // Extracts path
        let path = first_line[1].to_string();
        if !path.contains('/') {
            return Err(ParsingError::new("Invalid path"));
        }

        // Extracts HTTP version
        let version = first_line[2];
        if !version.contains('/') {
            return Err(ParsingError::new("Invalid HTTP version"));
        }
        let version: Vec<&str> = version.split('/').collect();
        let version = version[1];
        let version: f32 = match version.parse() {
            Ok(a) => a,
            Err(e) => {
                return Err(ParsingError::new(
                    format!("Invalid HTTP version: {:?}", e).as_str(),
                ))
            }
        };

        // Extracts body and headers
        let mut read_body = false;
        for (i, line) in s.iter().enumerate() {
            if read_body {
                body.push_str(line);
                continue;
            }

            // Skipd first line
            if i == 0 {
                continue;
            }

            // After headers there is an empty line that is followed by body
            if line.is_empty() {
                read_body = true;
                continue;
            }

            // Extraxts headers
            let header: Vec<&str> = line.split(':').collect();
            if header.len() != 2 {
                return Err(ParsingError::new("Failed to parse header"));
            }
            headers.insert(header[0].to_string(), header[1][1..].to_string());

            // Everything after this should be body
        }

        Ok(Self {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_from_str() {
        let a = Request::from("POST / HTTP/1.1\naaa: 1234\n\n<h1>Hello World</h1>");
        match a {
            Ok(_) => assert!(true),
            Err(ref e) => assert!(false, "{e}"),
        };
    }
}
