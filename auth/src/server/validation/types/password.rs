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
