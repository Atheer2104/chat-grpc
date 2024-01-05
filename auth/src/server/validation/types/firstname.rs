use super::validate_name;

#[derive(Debug)]
pub struct Firstname(String);

impl Firstname {
    pub fn parse(s: String) -> Result<Firstname, String> {
        match validate_name(&s) {
            Ok(_) => Ok(Self(s)),
            Err(_) => Err(format!("{} is not a valid firstname", s)),
        }
    }
}

impl AsRef<str> for Firstname {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
