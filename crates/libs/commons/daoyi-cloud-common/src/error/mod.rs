use crate::response::ApiResponse;
use salvo::http::StatusCode;
use salvo::prelude::*;
use salvo::writing::Json;

pub type ApiResult<T> = Result<T, ApiError>;

/// 业务错误码枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    Success = 0,
    BizError = 1000,
    NotFound = 1001,
    Validation = 2000,
    Unauthorized = 3000,
    TokenExpired = 3001,
    Forbidden = 3002,
    Internal = 5000,
    DbError = 5001,
    ExternalService = 5002,
}

impl ErrorCode {
    pub fn http_status(&self) -> StatusCode {
        match self {
            ErrorCode::Success | ErrorCode::BizError => StatusCode::OK,
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::Validation => StatusCode::BAD_REQUEST,
            ErrorCode::Unauthorized | ErrorCode::TokenExpired => StatusCode::UNAUTHORIZED,
            ErrorCode::Forbidden => StatusCode::FORBIDDEN,
            ErrorCode::Internal | ErrorCode::DbError | ErrorCode::ExternalService => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// 资源未找到
    #[error("服务器迷路了~")]
    NotFound,

    /// 请求方法不被允许
    #[error("请求方法不被允许")]
    MethodNotAllowed,

    /// 业务错误（前端弹窗提示）
    #[error("{0}")]
    Biz(String),

    /// 内部服务错误
    #[error("错误: {0}")]
    Internal(#[from] anyhow::Error),

    /// 数据库异常
    #[error("数据库异常: {0}")]
    DbErr(#[from] sea_orm::DbErr),

    /// 参数校验失败
    #[error("参数校验失败: {0}")]
    Validation(String),

    /// 密码加密错误
    #[error("密码错误: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    /// JWT 认证失败
    #[error("认证失败：{0}")]
    JWT(#[from] jsonwebtoken::errors::Error),

    /// 未授权
    #[error("未授权：{0}")]
    Unauthenticated(String),

    /// 权限不足
    #[error("权限不足：{0}")]
    Forbidden(String),

    /// Glob 匹配异常
    #[error("Glob异常: {0}")]
    Glob(#[from] wax::BuildError),
}

impl ApiError {
    /// 获取业务错误码
    pub fn error_code(&self) -> ErrorCode {
        match self {
            ApiError::NotFound | ApiError::MethodNotAllowed => ErrorCode::NotFound,
            ApiError::Biz(_) => ErrorCode::BizError,
            ApiError::Internal(_) => ErrorCode::Internal,
            ApiError::DbErr(_) => ErrorCode::DbError,
            ApiError::Validation(_) => ErrorCode::Validation,
            ApiError::Bcrypt(_) | ApiError::Glob(_) => ErrorCode::Internal,
            ApiError::JWT(_) => ErrorCode::TokenExpired,
            ApiError::Unauthenticated(_) => ErrorCode::Unauthorized,
            ApiError::Forbidden(_) => ErrorCode::Forbidden,
        }
    }

    /// 获取 HTTP 状态码
    pub fn status_code(&self) -> StatusCode {
        self.error_code().http_status()
    }

    /// 将当前错误自动记录日志
    pub fn log(&self) {
        match self {
            ApiError::Internal(_) | ApiError::DbErr(_) | ApiError::Glob(_) => {
                tracing::error!(error = %self, error_code = ?self.error_code());
            }
            ApiError::Validation(_)
            | ApiError::Unauthenticated(_)
            | ApiError::Forbidden(_)
            | ApiError::JWT(_)
            | ApiError::NotFound
            | ApiError::MethodNotAllowed => {
                tracing::warn!(error = %self, error_code = ?self.error_code());
            }
            ApiError::Biz(_) | ApiError::Bcrypt(_) => {
                tracing::info!(error = %self, error_code = ?self.error_code());
            }
        }
    }

    /// 转换为统一 API 响应
    pub fn to_api_response(&self) -> ApiResponse<()> {
        self.log();
        ApiResponse::err(self.error_code() as i32, &self.to_string())
    }

    /// 将本错误以 JSON 方式写入 Salvo Response
    pub fn write_to_response(self, res: &mut Response) {
        let status = self.status_code();
        let body = self.to_api_response();
        res.status_code(status);
        res.render(Json(body));
    }
}

/// 将 validator 的 ValidationErrors 格式化为易读的字段级错误信息
pub fn format_validation_errors(errors: &validator::ValidationErrors) -> String {
    format_to_vec(errors).join("; ")
}

fn format_to_vec(errors: &validator::ValidationErrors) -> Vec<String> {
    use validator::ValidationErrorsKind;
    errors
        .errors()
        .iter()
        .flat_map(|(field, errors_kind)| match errors_kind {
            ValidationErrorsKind::Field(field_errors) => field_errors
                .iter()
                .map(|error| {
                    let message = error
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "验证失败".to_string());
                    format!("[{}]{}", field, message)
                })
                .collect::<Vec<_>>(),
            ValidationErrorsKind::Struct(struct_errors) => format_to_vec(struct_errors),
            ValidationErrorsKind::List(list_errors) => list_errors
                .iter()
                .flat_map(|(_index, errors)| format_to_vec(errors))
                .collect::<Vec<_>>(),
        })
        .collect()
}
