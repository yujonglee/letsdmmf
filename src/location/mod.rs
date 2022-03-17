use reqwest::Url;
use std::fs;

mod validate;

#[derive(Debug, PartialEq)]
pub enum Location {
    Url(String),
    Path(String),
}

impl Location {
    pub fn get_schema(&self) -> Result<String, String> {
        match self {
            Location::Path(path) => match validate::path(&path) {
                Ok(()) => {
                    let schema = fs::read_to_string(path).expect("Failed to read schema from path");

                    Ok(schema)
                }
                Err(message) => Err(message),
            },
            Location::Url(url) => match validate::url(&url) {
                Ok(url) => {
                    let schema = reqwest::blocking::get(url)
                        .expect("Failed to get response")
                        .text()
                        .expect("Failed to convert response to text");

                    Ok(schema)
                }
                Err(message) => Err(message),
            },
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
