use secrecy::Secret;

#[derive(serde::Deserialize, Clone)]
pub struct Secrets {
    pub jwt_secret: Secret<String>,
}

pub fn get_secrets() -> Result<Secrets, config::ConfigError> {
    let path = "../secrets.yaml";

    let secrets = config::Config::builder()
        .add_source(config::File::new(path, config::FileFormat::Yaml))
        .build()?;

    secrets.try_deserialize::<Secrets>()
}
