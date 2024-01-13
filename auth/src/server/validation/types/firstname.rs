use super::{validate_name, RegisterDataError};

#[derive(Debug)]
pub struct Firstname(String);

impl Firstname {
    pub fn parse(s: String) -> Result<Firstname, RegisterDataError> {
        match validate_name(&s) {
            Ok(_) => Ok(Self(s)),
            Err(e) => Err(RegisterDataError::new("firstname".into(), e.into())),
        }
    }
}

impl AsRef<str> for Firstname {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Firstname;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_empty_firstname_is_rejected() {
        let firstname = "".to_string();
        assert_err!(Firstname::parse(firstname));
    }

    #[test]
    fn a_whitespace_only_firstname_is_rejected() {
        let firstname = "              ".to_string();
        assert_err!(Firstname::parse(firstname));
    }

    #[test]
    fn a_255_grapheme_name_is_parsed() {
        let firstname = "᪬".repeat(255);
        assert_ok!(Firstname::parse(firstname));
    }

    #[test]
    fn firstname_longer_than_255_grapeheme_is_rejected() {
        let firstname = "ᾥ".repeat(262);
        assert_err!(Firstname::parse(firstname));
    }

    #[test]
    fn firstname_consist_of_numbers_only_is_rejected() {
        let firstname = "1413489134".to_string();
        assert_err!(Firstname::parse(firstname));
    }

    #[test]
    fn firstname_contains_numbers_is_rejected() {
        let firstname = "atheer2104".to_string();
        assert_err!(Firstname::parse(firstname));
    }

    #[test]
    fn valid_firstname_is_parsed() {
        let firstname = "Alice".to_string();
        assert_ok!(Firstname::parse(firstname));
    }
}
