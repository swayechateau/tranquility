#[cfg(test)]
mod tests {
    use std::path::Path;

    use tranquility::models::application::schema::validate_file;

    #[test]
    fn test_valid_json_file() {
        let result = validate_file(Path::new("tests/fixtures/valid_applications.json"));
        assert!(result, "Expected valid file");
    }

    #[test]
    fn test_invalid_yaml_file() {
        let result = validate_file(Path::new("tests/fixtures/invalid_applications.yaml"));
        assert!(!result, "Expected invalid YAML to fail");
    }

    // Remove xml support for now
    // #[test]
    // fn test_valid_xml_file() {
    //     let result = validate_file(Path::new("tests/fixtures/valid_applications.xml"));
    //     assert!(result, "Expected valid XML file");
    // }
}
