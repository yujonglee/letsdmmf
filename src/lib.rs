pub mod validate {
    use std::path::Path;

    pub fn path(path: &Path) -> (bool, String) {
        let is_exist = path.exists();

        if is_exist {
            if path.is_dir() {
                let message = String::from("\"File\" expected, got \"directory\" instead");

                return (true, message);
            }

            if path.extension().unwrap() != "prisma" {
                let file_name = path.file_name().unwrap();
                let message = format!(
                    "Invalid File Extension. \"something.prisma\" expected, got {:?} instead",
                    file_name
                );

                return (true, message);
            }
        } else {
            let message = format!("No such file or directory: \"{}\"", path.to_str().unwrap());

            return (true, message);
        };

        return (false, String::from(""));
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

        let (is_error, message) = validate::path(Path::new(file_path));

        assert!(is_error);
        assert_eq!(
            message,
            "No such file or directory: \"/something/not/exist\""
        );
    }

    #[test]
    fn validate_directory() {
        let file_path = temp_dir();

        let (is_error, message) = validate::path(Path::new(file_path.to_str().unwrap()));

        assert!(is_error);
        assert_eq!(message, "\"File\" expected, got \"directory\" instead");
    }

    #[test]
    fn validate_right_extension() {
        let file_path = temp_dir().join("schema.prisma");
        File::create(&file_path).unwrap();

        let (is_error, message) = validate::path(Path::new(file_path.to_str().unwrap()));

        assert!(!is_error);
        assert_eq!(message, "");
    }

    #[test]
    fn validate_wrong_extension() {
        let file_path = temp_dir().join("schema.json");
        File::create(&file_path).unwrap();

        let (is_error, message) = validate::path(Path::new(file_path.to_str().unwrap()));

        assert!(is_error);
        assert_eq!(
            message,
            "Invalid File Extension. \"something.prisma\" expected, got \"schema.json\" instead"
        );
    }
}
