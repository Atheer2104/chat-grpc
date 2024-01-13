use super::{RegisterDataError, UnicodeSegmentation};
use thiserror::Error;

#[derive(Debug)]
pub struct Username(String);

#[derive(Debug, Error)]
pub enum ValidateUsernameError {
    #[error("username is empty or only contains whitespace")]
    EmptyOrWhitespace,
    #[error("username is longer than {0} charcters")]
    TooLong(u8),
    #[error("username contains '{0}' which is a forbidden character")]
    ContainForbiddenCharacater(char),
}

impl From<ValidateUsernameError> for RegisterDataError {
    fn from(value: ValidateUsernameError) -> Self {
        RegisterDataError::new("username".into(), value.into())
    }
}

const FORBIDDEN_CHARACTERS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
const MAX_USERNAME_LENGTH: u8 = 255;

impl Username {
    pub fn parse(s: String) -> Result<Username, ValidateUsernameError> {
        // is_empty_or_whitespace
        if s.trim().is_empty() {
            return Err(ValidateUsernameError::EmptyOrWhitespace);
        }

        // is_too_long
        if s.graphemes(true).count() > MAX_USERNAME_LENGTH.into() {
            return Err(ValidateUsernameError::TooLong(MAX_USERNAME_LENGTH));
        }

        let is_forbidden_char = s.chars().find(|c| FORBIDDEN_CHARACTERS.contains(c));

        // contains_forbidden_characters
        if let Some(forbidden_char) = is_forbidden_char {
            return Err(ValidateUsernameError::ContainForbiddenCharacater(
                forbidden_char,
            ));
        }

        Ok(Self(s))
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Username;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_255_grapheme_long_username_is_valid() {
        let username = "각".repeat(255);
        assert_ok!(Username::parse(username));
    }

    #[test]
    fn a_username_longer_than_255_graphemes_is_rejected() {
        let username = "b".repeat(256);
        assert_err!(Username::parse(username));
    }

    #[test]
    fn whitespace_only_username_is_rejected() {
        let username = "    ".to_string();
        assert_err!(Username::parse(username));
    }

    #[test]
    fn empty_username_is_rejected() {
        let username = "".to_string();
        assert_err!(Username::parse(username));
    }

    #[test]
    fn username_containing_a_forbidden_character() {
        for username in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let username = username.to_string();
            assert_err!(Username::parse(username));
        }
    }

    #[test]
    fn a_valid_username_is_parsed_successfully() {
        let username = "atheer2104".to_string();
        assert_ok!(Username::parse(username));
    }
}
