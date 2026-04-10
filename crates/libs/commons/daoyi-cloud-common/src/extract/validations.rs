use crate::constants::global_values::MOBILE_PHONE_REGEX;
use std::borrow::Cow;
use validator::ValidationError;

pub fn validate_page_size(page_size: u64) -> Result<(), validator::ValidationError> {
    match page_size {
        s if s < 1 => {
            let mut err = validator::ValidationError::new("page_size_range");
            err.message = Some("每页条数最小值为 1".into());
            Err(err)
        }
        s if s > 200 => {
            let mut err = validator::ValidationError::new("page_size_range");
            err.message = Some("每页条数最大值为 200".into());
            Err(err)
        }
        _ => Ok(()),
    }
}

pub fn validate_mobile_phone(value: &str) -> Result<(), validator::ValidationError> {
    if MOBILE_PHONE_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(build_validation_error("手机号格式不正确"))
    }
}

fn build_validation_error(message: &'static str) -> validator::ValidationError {
    ValidationError {
        code: Cow::from("invalid"),
        message: Some(Cow::from(message)),
        params: Default::default(),
    }
}
