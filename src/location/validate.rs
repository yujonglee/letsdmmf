use reqwest::Url;
use std::path::Path;

pub fn path(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    let is_exist = path.exists();

    if !is_exist {
        let message = format!("No such file or directory: \"{}\"", path.to_str().unwrap());

        return Err(message);
    };

    if path.is_dir() {
        let message = String::from("\"File\" expected, got \"directory\" instead");

        return Err(message);
    };

    match path.extension() {
        Some(extension) => {
            if extension != "prisma" {
                let file_name = path.file_name().unwrap();
                let message = format!(
                    "Invalid File Extension. \"something.prisma\" expected, got {:?} instead",
                    file_name
                );

                Err(message)
            } else {
                Ok(())
            }
        }
        None => Ok(()),
    }
}

#[cfg(test)]
mod path {
    use super::*;
    use std::env::temp_dir;
    use std::fs::File;

    #[test]
    fn validate_existence() {
        let file_path = "/something/not/exist";

        let result = path(file_path);

        assert_eq!(
            result.unwrap_err(),
            "No such file or directory: \"/something/not/exist\""
        );
    }

    #[test]
    fn validate_directory() {
        let file_path = temp_dir();

        let result = path(file_path.to_str().unwrap());

        assert_eq!(
            result.unwrap_err(),
            "\"File\" expected, got \"directory\" instead"
        );
    }

    #[test]
    fn validate_right_extension() {
        let file_path = temp_dir().join("schema.prisma");
        File::create(&file_path).unwrap();

        let result = path(file_path.to_str().unwrap());

        assert!(result.is_ok());
    }

    #[test]
    fn validate_wrong_extension() {
        let file_path = temp_dir().join("schema.json");
        File::create(&file_path).unwrap();

        let result = path(file_path.to_str().unwrap());

        assert_eq!(
            result.unwrap_err(),
            "Invalid File Extension. \"something.prisma\" expected, got \"schema.json\" instead"
        );
    }

    #[test]
    fn validate_no_extension() {
        let file_path = temp_dir().join("schema");
        File::create(&file_path).unwrap();

        let result = path(file_path.to_str().unwrap());

        assert!(result.is_ok());
    }
}

pub fn url(url: &str) -> Result<String, String> {
    match Url::parse(&url) {
        Ok(url) => Ok(url.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod url {
    use super::*;

    #[test]
    fn valid() {
        assert_eq!(
            url("https://www.google.com").unwrap(),
            "https://www.google.com/"
        );
        assert_eq!(
            url("http://www.google.com").unwrap(),
            "http://www.google.com/"
        );
    }

    #[test]
    fn invalid() {
        assert_eq!(
            url("www.google.com").unwrap_err(),
            "relative URL without a base"
        );
        assert_eq!(
            url("google.com").unwrap_err(),
            "relative URL without a base"
        );
    }
}
