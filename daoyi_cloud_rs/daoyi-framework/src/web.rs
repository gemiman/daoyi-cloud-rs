use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonResult<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T: Serialize> CommonResult<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptchaResponse<T: Serialize> {
    pub rep_code: String,
    pub rep_msg: String,
    pub rep_data: T,
}

impl<T: Serialize> CaptchaResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            rep_code: "0000".to_string(),
            rep_msg: "mock success".to_string(),
            rep_data: data,
        }
    }
}
