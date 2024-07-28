use unicode_segmentation::UnicodeSegmentation;

// username consts
const FORBIDDEN_CHARACTERS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
const MAX_USERNAME_LENGTH: u8 = 255;

// password consts
const MAX_PASSWORD_LENGTH: u8 = 255;
const MIN_PASSWORD_LENGTH: u8 = 8;

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
