use reqwest::Url;
use std::fs;

mod validate;

#[derive(Debug, PartialEq)]
pub enum Location {
    Url(String),
    Path(String),
}

impl Location {
    pub fn get_schema(self) -> Result<String, String> {
        let location = self.validate()?;

        match location {
            Location::Path(path) => {
                let schema = fs::read_to_string(path).expect("Failed to read schema from path");

                Ok(schema)
            }
            Location::Url(url) => {
                let schema = reqwest::blocking::get(url)
                    .expect("Failed to get response")
                    .text()
                    .expect("Failed to convert response to text");

                Ok(schema)
            }
        }
    }

    pub fn validate(self) -> Result<Location, String> {
        match self {
            Location::Path(ref path) => validate::path(&path).map(|_| self),
            Location::Url(ref url) => validate::url(&url).map(|_| self),
        }
    }
}

pub fn new(location: &str) -> Location {
    match Url::parse(&location) {
        Ok(_url) => Location::Url(String::from(location)),
        Err(_e) => {
            if location.starts_with("www.") {
                Location::Url(String::from(location))
            } else {
                Location::Path(String::from(location))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path() {
        assert_eq!(new("aa"), Location::Path(String::from("aa")));
        assert_eq!(new("bb/cc"), Location::Path(String::from("bb/cc")));
        assert_eq!(
            new("dd/ee.prisma"),
            Location::Path(String::from("dd/ee.prisma"))
        );
    }

    #[test]
    fn url() {
        assert_eq!(
            new("https://www.google.com"),
            Location::Url(String::from("https://www.google.com"))
        );
        assert_eq!(
            new("http://www.google.com"),
            Location::Url(String::from("http://www.google.com"))
        );
        assert_eq!(
            new("www.google.com"),
            Location::Url(String::from("www.google.com"))
        );
        assert_eq!(new("www."), Location::Url(String::from("www.")));
    }
}
