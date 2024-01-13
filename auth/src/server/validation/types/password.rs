use super::{RegisterDataError, UnicodeSegmentation};
use thiserror::Error;

#[derive(Debug)]
pub struct Password(String);

#[derive(Debug, Error)]
pub enum ValidatePasswordError {
    #[error("password is empty empty or only contains whitespace")]
    EmptyOrWhitespace,
    #[error("password is shorter than {0} characters")]
    TooShort(u8),
    #[error("password is longer than {0} characters")]
    TooLong(u8),
}

impl From<ValidatePasswordError> for RegisterDataError {
    fn from(value: ValidatePasswordError) -> Self {
        RegisterDataError::new("password".into(), value.into())
    }
}

const MAX_PASSWORD_LENGTH: u8 = 255;
const MIN_PASSWORD_LENGTH: u8 = 8;

impl Password {
    // TODO: add min length for password
    pub fn parse(s: String) -> Result<Password, ValidatePasswordError> {
        // is_empty_or_whitespace
        if s.trim().is_empty() {
            return Err(ValidatePasswordError::EmptyOrWhitespace);
        }

        // is_not_min_length
        if s.graphemes(true).count() < MIN_PASSWORD_LENGTH.into() {
            return Err(ValidatePasswordError::TooShort(MIN_PASSWORD_LENGTH));
        }

        // is_too_long
        if s.graphemes(true).count() > MAX_PASSWORD_LENGTH.into() {
            return Err(ValidatePasswordError::TooLong(MAX_PASSWORD_LENGTH));
        }

        Ok(Self(s))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Password;
    use claims::{assert_err, assert_ok};

    #[test]
    fn empty_password_is_rejected() {
        let password = "".to_string();
        assert_err!(Password::parse(password));
    }

    #[test]
    fn whitespace_only_password_is_rejected() {
        let password = " ".to_string();
        assert_err!(Password::parse(password));
    }

    #[test]
    fn a_255_grapheme_password_is_parsed() {
        let password = "ق".repeat(255);
        assert_ok!(Password::parse(password));
    }

    #[test]
    fn password_longer_than_255_grapehemes_is_rejected() {
        let password = "ز".repeat(300);
        assert_err!(Password::parse(password));
    }

    #[test]
    fn valid_password_is_parsed() {
        let password = "Very Strong Password 🔒".to_string();
        assert_ok!(Password::parse(password));
    }
}
