pub mod json;
pub mod schema;
pub mod xml;

pub enum SchemaExample {
    Json(json::VpsConfig),
    Xml(xml::VpsConfigXml),
}

pub fn generate_id(name: Option<&str>, host: &str, user: Option<&str>) -> String {
    let raw = name
        .or(Some(host))
        .unwrap_or("vps")
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "-");

    let user = user.unwrap_or("user").to_lowercase();
    format!("{}-{}", raw.trim_matches('-'), user)
}

pub fn schema_example(xml: bool) -> SchemaExample {
    let vps = json::VpsEntry {
        id: Some("example-vps-user".into()),
        name: Some("Example VPS".into()),
        user: get_username(),
        host: "example.com".into(),
        port: Some(json::FlexibleValue::from(22)),
        private_key: Some("/home/user/.ssh/id_rsa".into()),
        post_connect_script: Some("uptime && echo $USER".into()),
    };

    let schema = json::VpsConfig { vps: vec![vps] };
    if xml {
        SchemaExample::Xml(schema.into())
    } else {
        SchemaExample::Json(schema)
    }
}

pub fn vps_config_schema() {
    let example = schema_example(false);
    let json = match example {
        SchemaExample::Json(j) => serde_json::to_string_pretty(&j).unwrap(),
        SchemaExample::Xml(x) => serde_json::to_string_pretty(&x).unwrap(),
    };
    println!("{json}");
}

#[cfg(target_family = "unix")]
pub fn get_username() -> Option<String> {
    std::env::var("USER").ok()
}

#[cfg(target_family = "windows")]
pub fn get_username() -> Option<String> {
    std::env::var("USERNAME").ok()
}
