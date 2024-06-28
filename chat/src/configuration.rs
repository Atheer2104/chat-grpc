#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/configuration/config.yaml");

    let settings = config::Config::builder()
        .add_source(config::File::new(path, config::FileFormat::Yaml))
        .build()?;

    settings.try_deserialize::<Settings>()
}
