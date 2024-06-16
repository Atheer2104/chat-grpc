use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct Secrets {
    pub jwt_secret: Secret<String>,
}

pub fn get_secrets() -> Result<Secrets, config::ConfigError> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/configuration/secrets.yaml");

    let secrets = config::Config::builder()
        .add_source(config::File::new(path, config::FileFormat::Yaml))
        .build()?;

    secrets.try_deserialize::<Secrets>()
}
