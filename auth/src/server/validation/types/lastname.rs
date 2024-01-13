use super::{validate_name, RegisterDataError};

#[derive(Debug)]
pub struct Lastname(String);

impl Lastname {
    pub fn parse(s: String) -> Result<Lastname, RegisterDataError> {
        match validate_name(&s) {
            Ok(_) => Ok(Self(s)),
            Err(e) => Err(RegisterDataError::new("lastname".into(), e.into())),
        }
    }
}

impl AsRef<str> for Lastname {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Lastname;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_empty_lastname_is_rejected() {
        let lastname = "".to_string();
        assert_err!(Lastname::parse(lastname));
    }

    #[test]
    fn a_whitespace_only_lastname_is_rejected() {
        let lastname = "              ".to_string();
        assert_err!(Lastname::parse(lastname));
    }

    #[test]
    fn a_255_grapheme_name_is_parsed() {
        let lastname = "م".repeat(255);
        assert_ok!(Lastname::parse(lastname));
    }

    #[test]
    fn lastname_longer_than_255_grapeheme_is_rejected() {
        let lastname = "आ".repeat(262);
        assert_err!(Lastname::parse(lastname));
    }

    #[test]
    fn lastname_consist_of_numbers_only_is_rejected() {
        // arabic numbers are used here
        let lastname = "٣٨٣٢٠".to_string();
        assert_err!(Lastname::parse(lastname));
    }

    #[test]
    fn lastname_contains_numbers_is_rejected() {
        let lastname = "doe09".to_string();
        assert_err!(Lastname::parse(lastname));
    }

    #[test]
    fn valid_lastname_is_parsed() {
        let lastname = "smith".to_string();
        assert_ok!(Lastname::parse(lastname));
    }
}
