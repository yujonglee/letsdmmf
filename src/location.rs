use reqwest::Url;

#[derive(Debug, PartialEq)]
pub enum Location {
    Url(String),
    Path(String),
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
