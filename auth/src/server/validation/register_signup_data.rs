use super::{Email, Firstname, Lastname, Password, Username};
use crate::proto::auth::RegisterRequest;

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
