#[cfg(test)]
mod tests {
    use std::path::Path;

    use tranquility::engine::config::validate_file;

    // Note: These tests are disabled as the schema has been refactored.
    // Application validation is now handled through the use cases layer.
    // Re-enable or update these tests as part of a dedicated schema testing framework.

    #[test]
    #[ignore]
    fn test_valid_json_file() {
        let result = validate_file(Path::new("tests/fixtures/valid_applications.json"));
        assert!(result, "Expected valid file");
    }

    #[test]
    #[ignore]
    fn test_invalid_yaml_file() {
        let result = validate_file(Path::new("tests/fixtures/invalid_applications.yaml"));
        assert!(!result, "Expected invalid YAML to fail");
    }

    // Remove xml support for now
    // #[test]
    // #[ignore]
    // fn test_valid_xml_file() {
    //     let result = validate_file(Path::new("tests/fixtures/valid_applications.xml"));
    //     assert!(result, "Expected valid XML file");
    // }
}
