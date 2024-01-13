mod email;
mod firstname;
mod lastname;
mod password;
mod username;

pub use email::*;
pub use firstname::*;
pub use lastname::*;
pub use password::*;
pub use username::*;

use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

use super::RegisterDataError;

#[derive(Debug, Error)]
pub enum ValidateNameError {
    #[error("name is empty of only contains whitespace")]
    EmptyOrWhitepace,
    #[error("name is longer than {0} characters")]
    TooLong(u8),
    #[error("name contains numerics")]
    ContainNumbers,
}

const MAX_NAME_LENGTH: u8 = 255;

pub fn validate_name(s: &String) -> Result<(), ValidateNameError> {
    // is_empty_or_whitespace
    if s.trim().is_empty() {
        return Err(ValidateNameError::EmptyOrWhitepace);
    }

    // is_too_long
    if s.graphemes(true).count() > MAX_NAME_LENGTH.into() {
        return Err(ValidateNameError::TooLong(MAX_NAME_LENGTH));
    }

    // is_contain_numbers
    if s.chars().any(|c| c.is_numeric()) {
        return Err(ValidateNameError::ContainNumbers);
    }

    Ok(())
}
