pub mod validate {
    use std::path::Path;

    pub fn path(path: &Path) -> Result<(), String> {
        let is_exist = path.exists();

        if is_exist {
            if path.is_dir() {
                let message = String::from("\"File\" expected, got \"directory\" instead");

                return Err(message);
            }

            if path.extension().unwrap() != "prisma" {
                let file_name = path.file_name().unwrap();
                let message = format!(
                    "Invalid File Extension. \"something.prisma\" expected, got {:?} instead",
                    file_name
                );

                return Err(message);
            }
        } else {
            let message = format!("No such file or directory: \"{}\"", path.to_str().unwrap());

            return Err(message);
        };

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn validate_existence() {
        let file_path = "/something/not/exist";

        let result = validate::path(Path::new(file_path));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "No such file or directory: \"/something/not/exist\""
        );
    }

    #[test]
    fn validate_directory() {
        let file_path = temp_dir();

        let result = validate::path(Path::new(file_path.to_str().unwrap()));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "\"File\" expected, got \"directory\" instead"
        );
    }

    #[test]
    fn validate_right_extension() {
        let file_path = temp_dir().join("schema.prisma");
        File::create(&file_path).unwrap();

        let result = validate::path(Path::new(file_path.to_str().unwrap()));

        assert!(result.is_ok());
    }

    #[test]
    fn validate_wrong_extension() {
        let file_path = temp_dir().join("schema.json");
        File::create(&file_path).unwrap();

        let result = validate::path(Path::new(file_path.to_str().unwrap()));

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid File Extension. \"something.prisma\" expected, got \"schema.json\" instead"
        );
    }
}
