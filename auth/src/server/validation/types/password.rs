use super::UnicodeSegmentation;

#[derive(Debug)]
pub struct Password(String);

impl Password {
    // TODO: add min length for password
    pub fn parse(s: String) -> Result<Password, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 255;

        if is_empty_or_whitespace || is_too_long {
            Err(format!("{} is not a valid password", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
