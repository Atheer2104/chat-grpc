use super::validate_name;

#[derive(Debug)]
pub struct Lastname(String);

impl Lastname {
    pub fn parse(s: String) -> Result<Lastname, String> {
        match validate_name(&s) {
            Ok(_) => Ok(Self(s)),
            Err(_) => Err(format!("{} is not a valid lastname", s)),
        }
    }
}

impl AsRef<str> for Lastname {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
