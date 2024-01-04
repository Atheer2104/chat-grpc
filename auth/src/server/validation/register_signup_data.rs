use unicode_segmentation::UnicodeSegmentation;
use validator::validate_email;

use crate::proto::auth::RegisterRequest;

pub struct Firstname(String);
pub struct Lastname(String);
pub struct Username(String);
pub struct Email(String);
pub struct Password(String);

fn validate_name(s: &String) -> Result<(), ()> {
    let is_empty_or_whitespace = s.trim().is_empty();

    let is_too_long = s.graphemes(true).count() > 256;

    let is_contain_numbers = s.chars().any(|c| c.is_numeric());

    if is_empty_or_whitespace || is_too_long || is_contain_numbers {
        Err(())
    } else {
        Ok(())
    }
}

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

impl Username {
    pub fn parse(s: String) -> Result<Username, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

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

impl Password {
    pub fn parse(s: String) -> Result<Password, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

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

impl TryFrom<RegisterRequest> for UserRegisterSignupData {
    type Error = String;

    fn try_from(value: RegisterRequest) -> Result<Self, Self::Error> {
        let firstname = Firstname::parse(value.firstname)?;
        let lastname = Lastname::parse(value.lastname)?;
        let username = Username::parse(value.username)?;
        let email = Email::parse(value.email)?;
        let password = Password::parse(value.password)?;

        Ok(Self {
            firstname,
            lastname,
            username,
            email,
            password,
        })
    }
}

pub struct UserRegisterSignupData {
    pub firstname: Firstname,
    pub lastname: Lastname,
    pub username: Username,
    pub email: Email,
    pub password: Password,
}
