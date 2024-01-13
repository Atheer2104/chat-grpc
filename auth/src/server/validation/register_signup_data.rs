use super::{Email, Firstname, Lastname, Password, Username};
use crate::proto::auth::RegisterRequest;
use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub struct RegisterDataError {
    pub field: String,
    #[source]
    pub message: anyhow::Error,
}

impl std::fmt::Display for RegisterDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "field {} has following error: {:?}",
            self.field, self.message
        )
    }
}

impl RegisterDataError {
    pub fn new(field: String, message: anyhow::Error) -> RegisterDataError {
        Self { field, message }
    }
}

impl TryFrom<RegisterRequest> for RegisterData {
    type Error = RegisterDataError;

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

pub struct RegisterData {
    pub firstname: Firstname,
    pub lastname: Lastname,
    pub username: Username,
    pub email: Email,
    pub password: Password,
}
