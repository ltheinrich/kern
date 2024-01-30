//! String-level operations

/// Check if char is alphanumeric (latin)
///
/// Returns true for latin alphabet and numbers: a-z, A-Z, 0-9
///
/// Returns false for other characters
pub fn is_alphanumeric_char(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9')
}

/// Check if string is alphanumeric (latin)
///
/// Returns true for latin alphabet and numbers: a-z, A-Z, 0-9
///
/// Returns false for other characters
pub fn is_alphanumeric(s: impl AsRef<str>) -> bool {
    s.as_ref().chars().all(is_alphanumeric_char)
}
