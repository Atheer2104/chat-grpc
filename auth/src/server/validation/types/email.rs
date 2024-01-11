use validator::validate_email;

#[derive(Debug)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid e-mail", s))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use claims::{assert_err, assert_ok};
    use fake::{faker::internet::en::SafeEmail, Fake};
    use quickcheck::{Arbitrary, Gen};
    use rand::{rngs::SmallRng, SeedableRng};

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(Email::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "bobgmail.com".to_string();
        assert_err!(Email::parse(email));
    }

    #[test]
    fn email_no_subject_is_rejected() {
        let email = "@gmail.com".to_string();
        assert_err!(Email::parse(email));
    }

    #[derive(Debug, Clone)]
    struct ValidEmail(pub String);

    // quickcheck is a property based testing where one defines the property of the value then it automatically and randomly generate such
    // data inputs and that what we test since these inputs should always succeds hence the name property, it also implements something known as
    // shrink for integers, floats, tuples, booleans, lists, strings, options and results, the idea with shrink is that it will once something fails when
    // it should succeeds it will try to find the smallest possible counter example. By defult the test will stop after 100 successfull attempts

    // this is the trait that we have to implement which represents the types that can be randomly generated
    impl Arbitrary for ValidEmail {
        fn arbitrary(_g: &mut Gen) -> Self {
            // stupid fix becuase Gen is a struct and not a trait that implemetns RNG which means that, we crate a new rng from scratch each tim
            // but this rng is very fast and it's what Gen consists actually of
            let mut rng = SmallRng::from_entropy();
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    // this macro makes it esaier to write quickcheck test internally it will include the #[test]
    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmail) {
        assert_ok!(Email::parse(valid_email.0));
    }
}
