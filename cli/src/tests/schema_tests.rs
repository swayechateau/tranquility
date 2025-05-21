
#[cfg(test)]
mod tests {
    use crate::schema::application::validate_file;

    #[test]
    fn test_valid_json_file() {
        let result = validate_file("tests/fixtures/valid_applications.json");
        assert!(result.is_ok(), "Expected valid file");
    }

    #[test]
    fn test_invalid_yaml_file() {
        let result = validate_file("tests/fixtures/invalid_applications.yaml");
        assert!(result.is_err(), "Expected invalid YAML to fail");
    }

    #[test]
    fn test_valid_xml_file() {
        let result = validate_file("tests/fixtures/valid_applications.xml");
        assert!(result.is_ok(), "Expected valid XML file");
    }
}
