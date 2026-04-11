use regex::Regex;
use std::cell::LazyCell;

pub const PAGE_SIZE_NONE: u64 = 0;
pub const ROOT_ID: &str = "0";
pub const MOBILE_PHONE_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^\+?[1-9]\d{6,14}$").expect("Failed to compile mobile phone regex")
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_china_mobile() {
        assert!(MOBILE_PHONE_REGEX.is_match("13912345678"));
    }

    #[test]
    fn test_us_mobile() {
        assert!(MOBILE_PHONE_REGEX.is_match("+12025551234"));
    }

    #[test]
    fn test_uk_mobile() {
        assert!(MOBILE_PHONE_REGEX.is_match("+447911123456"));
    }

    #[test]
    fn test_japan_mobile() {
        assert!(MOBILE_PHONE_REGEX.is_match("+819012345678"));
    }

    #[test]
    fn test_with_plus_prefix() {
        assert!(MOBILE_PHONE_REGEX.is_match("+8613912345678"));
    }

    #[test]
    fn test_without_plus() {
        assert!(MOBILE_PHONE_REGEX.is_match("8613912345678"));
    }

    #[test]
    fn test_min_length() {
        // E.164 最短 7 位 (首位1-9 + 6位数字)
        assert!(MOBILE_PHONE_REGEX.is_match("+1234567"));
    }

    #[test]
    fn test_max_length() {
        // E.164 最长 15 位 (首位1-9 + 14位数字)
        assert!(MOBILE_PHONE_REGEX.is_match("+123456789012345"));
    }

    #[test]
    fn test_reject_starts_with_zero() {
        assert!(!MOBILE_PHONE_REGEX.is_match("0123456789"));
    }

    #[test]
    fn test_reject_too_short() {
        assert!(!MOBILE_PHONE_REGEX.is_match("+123456"));
    }

    #[test]
    fn test_reject_too_long() {
        assert!(!MOBILE_PHONE_REGEX.is_match("+1234567890123456"));
    }

    #[test]
    fn test_reject_empty() {
        assert!(!MOBILE_PHONE_REGEX.is_match(""));
    }

    #[test]
    fn test_reject_letters() {
        assert!(!MOBILE_PHONE_REGEX.is_match("+1abc567890"));
    }
}
