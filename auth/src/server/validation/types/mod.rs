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

use unicode_segmentation::UnicodeSegmentation;

pub fn validate_name(s: &String) -> Result<(), ()> {
    let is_empty_or_whitespace = s.trim().is_empty();

    let is_too_long = s.graphemes(true).count() > 255;

    let is_contain_numbers = s.chars().any(|c| c.is_numeric());

    if is_empty_or_whitespace || is_too_long || is_contain_numbers {
        Err(())
    } else {
        Ok(())
    }
}
