use super::UnicodeSegmentation;

#[derive(Debug)]
pub struct Username(String);

impl Username {
    pub fn parse(s: String) -> Result<Username, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 255;

        // some forbidden characters
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|c| forbidden_characters.contains(&c));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid username", s))
        } else {
            Ok(Self(s))
        }
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
