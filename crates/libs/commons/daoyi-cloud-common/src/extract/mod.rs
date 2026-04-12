pub mod validations;

use crate::error::{ApiError, format_validation_errors};
use salvo::prelude::*;
use serde::de::DeserializeOwned;
use validator::Validate;

/// 从请求体提取并校验 JSON
pub async fn extract_valid_json<T: DeserializeOwned + Validate + Send>(
    req: &mut Request,
    _depot: &mut Depot,
) -> Result<T, ApiError> {
    let body = req
        .payload()
        .await
        .map_err(|e| ApiError::Validation(format!("请求体读取失败: {}", e)))?;
    let data: T = serde_json::from_slice(body)
        .map_err(|e| ApiError::Validation(format!("请求体解析失败: {}", e)))?;
    data.validate()
        .map_err(|e| ApiError::Validation(format_validation_errors(&e)))?;
    Ok(data)
}

/// 从查询参数提取并校验
pub async fn extract_valid_query<T: DeserializeOwned + Validate + Send>(
    req: &mut Request,
    _depot: &mut Depot,
) -> Result<T, ApiError> {
    let query_str = req.uri().query().unwrap_or("");
    let data: T = serde_html_form::from_str(query_str)
        .map_err(|e| ApiError::Validation(format!("查询参数解析失败: {}", e)))?;
    data.validate()
        .map_err(|e| ApiError::Validation(format_validation_errors(&e)))?;
    Ok(data)
}

/// 从路径参数提取
pub fn extract_path_param<T: serde::de::DeserializeOwned + std::fmt::Debug>(
    req: &mut Request,
    name: &str,
) -> Result<T, ApiError> {
    req.param::<T>(name)
        .ok_or_else(|| ApiError::Validation(format!("路径参数 {} 缺失或格式错误", name)))
}
