use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub version: f32,
    pub status: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(
        version: f32, status: String, headers: HashMap<String, String>, body: String,
    ) -> Self {
        Response {
            version,
            status,
            headers,
            body,
        }
    }

    pub fn to_string(self) -> String {
        let headers: String = self
            .headers
            .into_iter()
            .map(|x| x.0.to_owned() + ": " + &x.1 + "\n")
            .collect();

        format!(
            "HTTP/{} {}\n{}\n{}",
            self.version, self.status, headers, self.body
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn response_as_str() {
        let res = Response::new(
            1.1,
            String::from("200 OK"),
            HashMap::from([
                ("a".to_string(), "b".to_string()),
                ("1".to_string(), "2".to_string()),
            ]),
            String::from("<h1>Hello World</h1>"),
        );

        let number_of_lines = res.to_string().lines().collect::<Vec<&str>>().len();
        assert_eq!(5, number_of_lines);
    }
}
