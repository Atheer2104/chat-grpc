use unicode_segmentation::UnicodeSegmentation;
use validator::validate_email;

// username consts
const FORBIDDEN_CHARACTERS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
const MAX_USERNAME_LENGTH: u8 = 255;

// password consts
const MAX_PASSWORD_LENGTH: u8 = 255;
const MIN_PASSWORD_LENGTH: u8 = 8;

// names consts
const MAX_NAME_LENGTH: u8 = 255;

pub fn validate_username(username: &str) -> Result<(), String> {
    // is_empty_or_whitespace
    if username.trim().is_empty() {
        return Err("Username is empty or contains to many whitespaces".into());
    }

    // is_too_long
    if username.graphemes(true).count() > MAX_USERNAME_LENGTH.into() {
        return Err(format!(
            "Username is longer than {} chars",
            MAX_USERNAME_LENGTH
        ));
    }

    let is_forbidden_char = username.chars().find(|c| FORBIDDEN_CHARACTERS.contains(c));

    // contains_forbidden_characters
    if let Some(forbidden_char) = is_forbidden_char {
        return Err(format!(
            "Username contains '{}' which is a forbidden char",
            forbidden_char
        ));
    }

    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), String> {
    // is_empty_or_whitespace
    if password.trim().is_empty() {
        return Err("Password is empty or contains to many whitespaces".into());
    }

    // is_not_min_length
    if password.graphemes(true).count() < MIN_PASSWORD_LENGTH.into() {
        return Err(format!(
            "Password is less than {} chars",
            MIN_PASSWORD_LENGTH
        ));
    }

    // is_too_long
    if password.graphemes(true).count() > MAX_PASSWORD_LENGTH.into() {
        return Err(format!(
            "Password is longer than {} chars",
            MAX_PASSWORD_LENGTH
        ));
    }

    Ok(())
}

pub fn parse_email(s: &str) -> Result<(), String> {
    if validate_email(s) {
        Ok(())
    } else {
        Err("Email is not valid".into())
    }
}

pub fn validate_name(s: &str, quantity: &str) -> Result<(), String> {
    // is_empty_or_whitespace
    if s.trim().is_empty() {
        return Err(format!(
            "{} is empty or contains too many whitespaces",
            quantity
        ));
    }

    // is_too_long
    if s.graphemes(true).count() > MAX_NAME_LENGTH.into() {
        return Err(format!(
            "{} is longer than {} chars",
            quantity, MAX_NAME_LENGTH
        ));
    }

    // is_contain_numbers
    if s.chars().any(|c| c.is_numeric()) {
        return Err(format!("{} contains numbers", quantity));
    }

    Ok(())
}
